extern crate anyhow;
extern crate learnopengl_utils as utils;
extern crate nalgebra_glm as glm;
extern crate renderer;

use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::rc::Rc;
use std::time::Instant;

use anyhow::Result;

use camera::{Camera, MovementDirection};
use renderer::application;
use renderer::application::{App, Application, View};
use renderer::input::{InputManager, Key};
use renderer::renderer_context::{OpenGLVersion, RendererContext, WindowDimension};
use renderer::time::Time;
use utils::gl;

mod camera;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum LightType {
    Directional,
    Point,
    Spot,
}

struct Settings {
    pub shininess: f32,
    pub light_type: LightType,
    pub light_type_selected: usize,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            shininess: 32.,
            light_type: LightType::Directional,
            light_type_selected: 0,
        }
    }
}

struct SettingsView {
    settings: Rc<RefCell<Settings>>,
    light_types: Vec<LightType>,
}

impl SettingsView {
    pub fn new(settings: Rc<RefCell<Settings>>, light_types: Vec<LightType>) -> Self {
        Self {
            settings,
            light_types,
        }
    }
}

impl View for SettingsView {
    fn name(&self) -> &str {
        "Settings"
    }

    fn show(&mut self, ui: &imgui::Ui) {
        let mut settings = self.settings.borrow_mut();

        ui.slider_config("Shininess", 1., 256.)
            .flags(imgui::SliderFlags::LOGARITHMIC)
            .build(&mut settings.shininess);
        ui.combo(
            "Light type",
            &mut settings.light_type_selected,
            self.light_types.as_slice(),
            |lt| std::borrow::Cow::from(format!("{}", lt)),
        );
        settings.light_type = self.light_types[settings.light_type_selected];
    }
}

impl Display for LightType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LightType::Directional => write!(f, "Directional"),
            LightType::Point => write!(f, "Point"),
            LightType::Spot => write!(f, "Spot"),
        }
    }
}

struct State {
    vertex_array_object: gl::VertexArrayId,
    cube_program: gl::ProgramId,
    light_program: gl::ProgramId,

    diffuse_texture: gl::TextureId,
    specular_texture: gl::TextureId,

    cubes: Vec<glm::Vec3>,

    camera: Camera,

    settings: Rc<RefCell<Settings>>,

    views: Vec<Box<dyn View>>,
}

impl Application for State {
    fn tick(&mut self, time: &Time<Instant>, input_manager: &dyn InputManager) {
        if input_manager.key_down(Key::W) {
            self.camera
                .process_keyboard(MovementDirection::FORWARD, time);
        }
        if input_manager.key_down(Key::S) {
            self.camera
                .process_keyboard(MovementDirection::BACKWARD, time);
        }
        if input_manager.key_down(Key::A) {
            self.camera.process_keyboard(MovementDirection::LEFT, time);
        }
        if input_manager.key_down(Key::D) {
            self.camera.process_keyboard(MovementDirection::RIGHT, time);
        }

        let mouse_movement = input_manager.mouse_movement();
        self.camera
            .process_mouse_movement((mouse_movement.0 as _, -mouse_movement.1 as _), true);

        let (_, scroll_y) = input_manager.scroll();
        self.camera.process_mouse_scroll(scroll_y as _);

        gl::viewport((0, 0), (800, 600));
        gl::enable(gl::Capability::DEPTH_TEST);

        gl::clear_color(0xFF020202);
        gl::clear(gl::ClearMask::COLOR_BUFFER_BIT | gl::ClearMask::DEPTH_BUFFER_BIT);

        let projection = glm::perspective(800. / 600., self.camera.zoom().to_radians(), 0.1, 100.);
        let view = self.camera.view_matrix();

        gl::bind_vertex_array(self.vertex_array_object);

        gl::use_program(self.cube_program);
        gl::uniform_matrix_4fv(
            gl::uniform_location(self.cube_program, "projection"),
            false,
            glm::value_ptr(&projection),
        );
        gl::uniform_matrix_4fv(
            gl::uniform_location(self.cube_program, "view"),
            false,
            glm::value_ptr(&view),
        );
        gl::uniform_3fv(
            gl::uniform_location(self.cube_program, "viewPos"),
            glm::value_ptr(self.camera.position()),
        );

        gl::active_texture(gl::TextureUnit::fixed(0));
        gl::bind_texture(gl::TextureTarget::TEXTURE_2D, self.diffuse_texture);
        gl::uniform_1i(
            gl::uniform_location(self.cube_program, "material.diffuse"),
            0,
        );

        gl::active_texture(gl::TextureUnit::fixed(1));
        gl::bind_texture(gl::TextureTarget::TEXTURE_2D, self.specular_texture);
        gl::uniform_1i(
            gl::uniform_location(self.cube_program, "material.specular"),
            1,
        );

        let settings = self.settings.borrow();
        gl::uniform_1f(
            gl::uniform_location(self.cube_program, "material.shininess"),
            settings.shininess,
        );

        match settings.light_type {
            LightType::Directional => gl::uniform_3fv(
                gl::uniform_location(self.cube_program, "light.direction"),
                glm::value_ptr(&glm::vec3(-0.2, -1., -0.3)),
            ),
            LightType::Point => {
                gl::uniform_3fv(
                    gl::uniform_location(self.cube_program, "light.position"),
                    glm::value_ptr(&glm::vec3(1.2, 1., 2.)),
                );
                gl::uniform_1f(
                    gl::uniform_location(self.cube_program, "light.constant"),
                    1.,
                );
                gl::uniform_1f(
                    gl::uniform_location(self.cube_program, "light.linear"),
                    0.09,
                );
                gl::uniform_1f(
                    gl::uniform_location(self.cube_program, "light.quadratic"),
                    0.032,
                );
            }
            LightType::Spot => {
                gl::uniform_3fv(
                    gl::uniform_location(self.cube_program, "light.position"),
                    glm::value_ptr(self.camera.position()),
                );
                gl::uniform_3fv(
                    gl::uniform_location(self.cube_program, "light.direction"),
                    glm::value_ptr(self.camera.front()),
                );
                gl::uniform_1f(
                    gl::uniform_location(self.cube_program, "light.cutoff"),
                    12.5f32.to_radians().cos(),
                );
                gl::uniform_1f(
                    gl::uniform_location(self.cube_program, "light.outerCutoff"),
                    17.5f32.to_radians().cos(),
                );
            }
        }

        gl::uniform_3fv(
            gl::uniform_location(self.cube_program, "light.ambient"),
            glm::value_ptr(&glm::vec3(0.2, 0.2, 0.2)),
        );
        gl::uniform_3fv(
            gl::uniform_location(self.cube_program, "light.diffuse"),
            glm::value_ptr(&glm::vec3(0.5, 0.5, 0.5)),
        );
        gl::uniform_3fv(
            gl::uniform_location(self.cube_program, "light.specular"),
            glm::value_ptr(&glm::vec3(1., 1., 1.)),
        );
        gl::uniform_1ui(
            gl::uniform_location(self.cube_program, "light.type"),
            settings.light_type_selected as _,
        );

        for (index, position) in self.cubes.iter().enumerate() {
            let model = glm::translation(position);
            let angle = 20. * (index as f32);
            let model = glm::rotate(&model, angle.to_radians(), &glm::vec3(1., 0.3, 0.5));
            gl::uniform_matrix_4fv(
                gl::uniform_location(self.cube_program, "model"),
                false,
                glm::value_ptr(&model),
            );
            gl::draw_arrays(gl::DrawMode::TRIANGLES, 0, 36);
        }

        if settings.light_type == LightType::Point {
            gl::use_program(self.light_program);
            let model = glm::Mat4::identity();
            let model = glm::translate(&model, &glm::vec3(1.2, 1., 2.));
            let model = glm::scale(&model, &glm::vec3(0.2, 0.2, 0.2));
            gl::uniform_matrix_4fv(
                gl::uniform_location(self.light_program, "projection"),
                false,
                glm::value_ptr(&projection),
            );
            gl::uniform_matrix_4fv(
                gl::uniform_location(self.light_program, "view"),
                false,
                glm::value_ptr(&view),
            );
            gl::uniform_matrix_4fv(
                gl::uniform_location(self.light_program, "model"),
                false,
                glm::value_ptr(&model),
            );
            gl::draw_arrays(gl::DrawMode::TRIANGLES, 0, 36);
        }
    }

    fn views(&mut self) -> &mut [Box<dyn View>] {
        &mut self.views
    }
}

impl App for State {
    fn new() -> Result<Self>
    where
        Self: Sized,
    {
        let cube = renderer::primitives::cube();
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
            gl::ComponentSize::SIZE_3,
            gl::ComponentType::FLOAT,
            false,
            size_of::<f32>() * 8,
            size_of::<f32>() * 3,
        );
        gl::enable_vertex_attrib_array(1);
        gl::vertex_attrib_pointer(
            2,
            gl::ComponentSize::SIZE_2,
            gl::ComponentType::FLOAT,
            false,
            size_of::<f32>() * 8,
            size_of::<f32>() * 6,
        );
        gl::enable_vertex_attrib_array(2);
        gl::bind_vertex_array(gl::VertexArrayId::NO_VERTEX_ARRAY);

        let cube_program = utils::program(
            include_str!("../assets/cube.vert"),
            include_str!("../assets/cube.frag"),
        )?;

        let light_program = utils::program(
            include_str!("../assets/light.vert"),
            include_str!("../assets/light.frag"),
        )?;

        let diffuse_texture = utils::load_texture_2d(include_bytes!("../assets/container2.png"))?;
        let specular_texture =
            utils::load_texture_2d(include_bytes!("../assets/container2_specular.png"))?;

        let settings = Rc::new(RefCell::new(Settings::default()));
        Ok(Self {
            vertex_array_object,
            cube_program,
            light_program,

            diffuse_texture,
            specular_texture,

            cubes: vec![
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

            camera: Camera::new(glm::vec3(0., 0., 3.), glm::vec3(0., 1., 0.), -90., 0.),

            settings: settings.clone(),

            views: vec![Box::new(SettingsView::new(
                settings.clone(),
                vec![LightType::Directional, LightType::Point, LightType::Spot],
            ))],
        })
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
