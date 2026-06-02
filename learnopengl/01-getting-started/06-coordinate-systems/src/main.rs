extern crate anyhow;
extern crate learnopengl_utils as utils;
extern crate nalgebra_glm as glm;
extern crate renderer;
extern crate stb_image;

use std::cell::{Ref, RefCell};
use std::rc::Rc;
use std::time::Instant;

use anyhow::Result;
use imgui::Ui;
use renderer::application;
use renderer::application::{App, Application, View};
use renderer::input::{InputManager, Key};
use renderer::renderer_context::{OpenGLVersion, RendererContext, WindowDimension};
use renderer::time::Time;
use utils::gl;

struct TextureSettings {
    pub texture_factor: f32,
    pub flip_face: bool,
    pub texture_scale: f32,
}

impl Default for TextureSettings {
    fn default() -> Self {
        Self {
            texture_factor: 0.2,
            flip_face: false,
            texture_scale: 1.,
        }
    }
}

struct TextureSettingsView {
    settings: Rc<RefCell<TextureSettings>>,
}

impl TextureSettingsView {
    pub fn new(settings: Rc<RefCell<TextureSettings>>) -> Self {
        Self { settings }
    }
}

impl View for TextureSettingsView {
    fn name(&self) -> &str {
        "Texture Settings"
    }

    fn show(&mut self, ui: &Ui) {
        let mut settings = self.settings.borrow_mut();
        ui.slider("Texture mix factor", 0., 1., &mut settings.texture_factor);
        ui.checkbox("Flip face horizontally", &mut settings.flip_face);
        ui.slider("Texture scale", 0.25, 4., &mut settings.texture_scale);
    }
}

struct ViewSettings {
    pub fov: f32,
    pub aspect_ratio: f32,
}

impl Default for ViewSettings {
    fn default() -> Self {
        Self {
            fov: 45.,
            aspect_ratio: 800. / 600.,
        }
    }
}

struct ViewSettingsView {
    settings: Rc<RefCell<ViewSettings>>,
}

impl ViewSettingsView {
    pub fn new(settings: Rc<RefCell<ViewSettings>>) -> Self {
        Self { settings }
    }
}

impl View for ViewSettingsView {
    fn name(&self) -> &str {
        "View Settings"
    }

    fn show(&mut self, ui: &Ui) {
        let mut settings = self.settings.borrow_mut();
        ui.slider("Field of view", 1., 179., &mut settings.fov);
        ui.slider("Aspect ratio", 0.25, 4., &mut settings.aspect_ratio);
    }
}

struct CubePositions {
    positions: Vec<glm::Vec3>,
}

impl Default for CubePositions {
    fn default() -> Self {
        Self {
            positions: vec![
                glm::vec3(0., 0., 0.),
                glm::vec3(2., 5., -15.),
                glm::vec3(-1.5, -2.2, -2.5),
                glm::vec3(-3.8, -2., -12.3),
                glm::vec3(2.4, -0.4, -3.5),
                glm::vec3(-1.7, 3., -7.5),
                glm::vec3(1.3, -2., -2.5),
                glm::vec3(1.5, 2., -2.5),
                glm::vec3(1.5, 0.2, -1.5),
                glm::vec3(-1.3, 1., -1.5),
            ],
        }
    }
}

struct CubePositionsView {
    positions: Rc<RefCell<CubePositions>>,
}

impl CubePositionsView {
    fn new(positions: Rc<RefCell<CubePositions>>) -> Self {
        Self { positions }
    }
}

impl View for CubePositionsView {
    fn name(&self) -> &str {
        "Cube Positions"
    }

    fn show(&mut self, ui: &Ui) {
        if ui.collapsing_header("Cubes", imgui::TreeNodeFlags::empty()) {
            for (index, position) in self.positions.borrow_mut().positions.iter_mut().enumerate() {
                ui.text(format!("Cube n°{index}"));
                ui.same_line();
                ui.input_float(format!("## cube-{index}-x"), &mut position.x)
                    .build();
                ui.same_line();
                ui.input_float(format!("## cube-{index}-y"), &mut position.y)
                    .build();
                ui.same_line();
                ui.input_float(format!("## cube-{index}-z"), &mut position.z)
                    .build();
            }
        }
    }
}

struct State {
    shader_program: gl::ProgramId,
    vertex_array_object: gl::VertexArrayId,
    container_texture: gl::TextureId,
    face_texture: gl::TextureId,

    position: glm::Vec3,

    texture_settings: Rc<RefCell<TextureSettings>>,
    view_settings: Rc<RefCell<ViewSettings>>,
    cube_positions: Rc<RefCell<CubePositions>>,
    views: Vec<Box<dyn View>>,
}

impl App for State {
    fn new() -> Result<Self>
    where
        Self: Sized,
    {
        let cube = utils::primitives::cube();
        let vertex_data = cube.as_f32_slice();

        let vertex_array_object = gl::create_vertex_array();
        gl::bind_vertex_array(vertex_array_object);

        let triangle_vbo = gl::create_buffer();
        gl::bind_buffer(gl::BufferTarget::ARRAY_BUFFER, triangle_vbo);
        gl::buffer_data(
            gl::BufferTarget::ARRAY_BUFFER,
            vertex_data,
            gl::BufferUsage::STATIC_DRAW,
        );

        gl::vertex_attrib_pointer(
            0,
            gl::ComponentSize::SIZE_3,
            gl::ComponentType::FLOAT,
            false,
            size_of::<f32>() * 8,
            0,
        );
        gl::enable_vertex_attrib_array(0);
        gl::vertex_attrib_pointer(
            1,
            gl::ComponentSize::SIZE_2,
            gl::ComponentType::FLOAT,
            false,
            size_of::<f32>() * 8,
            size_of::<f32>() * 6,
        );
        gl::enable_vertex_attrib_array(1);
        gl::bind_vertex_array(gl::VertexArrayId::NO_VERTEX_ARRAY);

        let shader_program = utils::program(
            include_str!("../assets/triangle.vert"),
            include_str!("../assets/triangle.frag"),
        )?;

        let container_texture = utils::load_texture_2d(include_bytes!("../assets/container.jpg"))?;
        let face_texture = utils::load_texture_2d(include_bytes!("../assets/awesomeface.png"))?;
        gl::tex_parameter_iuiv(
            gl::TextureTarget::TEXTURE_2D,
            gl::TextureParameter::TEXTURE_WRAP_S,
            &[gl::sys::MIRRORED_REPEAT],
        );
        gl::tex_parameter_iuiv(
            gl::TextureTarget::TEXTURE_2D,
            gl::TextureParameter::TEXTURE_WRAP_T,
            &[gl::sys::REPEAT],
        );

        let texture_settings = Rc::new(RefCell::new(TextureSettings::default()));
        let view_settings = Rc::new(RefCell::new(ViewSettings::default()));
        let cube_positions = Rc::new(RefCell::new(CubePositions::default()));
        Ok(Self {
            shader_program,
            vertex_array_object,
            container_texture,
            face_texture,

            position: glm::vec3(0., 0., -3.),

            texture_settings: texture_settings.clone(),
            view_settings: view_settings.clone(),
            cube_positions: cube_positions.clone(),
            views: vec![
                Box::new(TextureSettingsView::new(texture_settings.clone())),
                Box::new(ViewSettingsView::new(view_settings.clone())),
                Box::new(CubePositionsView::new(cube_positions.clone())),
            ],
        })
    }
}

impl Application for State {
    fn tick(&mut self, time: &Time<Instant>, input_manager: &dyn InputManager) {
        let speed = time.duration().as_secs_f32();
        if input_manager.key_down(Key::W) {
            self.position.z += speed;
        }
        if input_manager.key_down(Key::S) {
            self.position.z -= speed;
        }
        if input_manager.key_down(Key::A) {
            self.position.x += speed;
        }
        if input_manager.key_down(Key::D) {
            self.position.x -= speed;
        }
        if input_manager.key_down(Key::SPACE) {
            self.position.y -= speed;
        }
        if input_manager.key_down(Key::LEFT_CONTROL) {
            self.position.y += speed;
        }

        gl::viewport((0, 0), (800, 600));
        gl::enable(gl::Capability::DEPTH_TEST);

        gl::clear_color(0xFF334C4C);
        gl::clear(gl::ClearMask::COLOR_BUFFER_BIT | gl::ClearMask::DEPTH_BUFFER_BIT);

        gl::active_texture(gl::TextureUnit::fixed(0));
        gl::bind_texture(gl::TextureTarget::TEXTURE_2D, self.container_texture);

        gl::active_texture(gl::TextureUnit::fixed(1));
        gl::bind_texture(gl::TextureTarget::TEXTURE_2D, self.face_texture);

        gl::use_program(self.shader_program);

        let texture_settings = self.texture_settings.borrow();
        let view_settings = self.view_settings.borrow();
        let projection = glm::perspective(
            view_settings.aspect_ratio,
            view_settings.fov.to_radians(),
            0.1,
            100.,
        );
        let view = glm::translation(&self.position);
        gl::uniform_matrix_4fv(
            gl::uniform_location(self.shader_program, "projection"),
            false,
            glm::value_ptr(&projection),
        );
        gl::uniform_matrix_4fv(
            gl::uniform_location(self.shader_program, "view"),
            false,
            glm::value_ptr(&view),
        );
        gl::uniform_1i(gl::uniform_location(self.shader_program, "texture1"), 0);
        gl::uniform_1i(gl::uniform_location(self.shader_program, "texture2"), 1);
        gl::uniform_1f(
            gl::uniform_location(self.shader_program, "texture_factor"),
            texture_settings.texture_factor,
        );
        gl::uniform_1i(
            gl::uniform_location(self.shader_program, "flip_face"),
            if texture_settings.flip_face { 1 } else { 0 },
        );
        gl::uniform_1f(
            gl::uniform_location(self.shader_program, "texture_scale"),
            texture_settings.texture_scale,
        );

        gl::bind_vertex_array(self.vertex_array_object);
        let t = time.duration_since_start().as_secs_f32();
        let cubes = self.cube_positions.borrow();
        for (index, position) in cubes.positions.iter().enumerate() {
            let model = glm::translation(position);
            let angle = if index % 3 == 0 {
                25. * t
            } else {
                20. * (index as f32)
            };
            let model = glm::rotate(&model, angle.to_radians(), &glm::vec3(1., 0.3, 0.5));
            gl::uniform_matrix_4fv(
                gl::uniform_location(self.shader_program, "model"),
                false,
                glm::value_ptr(&model),
            );
            gl::draw_arrays(gl::DrawMode::TRIANGLES, 0, 36);
        }
    }

    fn views(&mut self) -> &mut [Box<dyn View>] {
        self.views.as_mut_slice()
    }
}

pub fn main() -> Result<()> {
    let context = RendererContext::init(
        "LearnOpenGL",
        &WindowDimension::of(800, 600),
        &OpenGLVersion::of(3, 3),
    )?;

    application::start::<State>(context)
}
