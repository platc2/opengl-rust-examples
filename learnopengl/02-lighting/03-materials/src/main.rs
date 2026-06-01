extern crate anyhow;
extern crate learnopengl_utils as utils;
extern crate nalgebra_glm as glm;
extern crate renderer;
extern crate serde;
extern crate serde_json;

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

use anyhow::Result;

use camera::{Camera, MovementDirection};
use material::{JsonMaterial, Material};
use renderer::application;
use renderer::application::{App, Application, View};
use renderer::input::{InputManager, Key};
use renderer::renderer_context::{OpenGLVersion, RendererContext, WindowDimension};
use renderer::time::Time;
use utils::gl;

mod camera;
mod material;

struct MaterialSettings {
    materials: Vec<Material>,
    selected_index: usize,
}

impl MaterialSettings {
    pub fn new(materials: Vec<Material>) -> Self {
        Self {
            materials,
            selected_index: 0,
        }
    }
}

struct MaterialSettingsView {
    material_names: Vec<String>,
    selected_index: Rc<RefCell<usize>>,
}

impl MaterialSettingsView {
    pub fn new(material_names: Vec<String>, selected_index: Rc<RefCell<usize>>) -> Self {
        Self {
            material_names,
            selected_index,
        }
    }
}

impl View for MaterialSettingsView {
    fn name(&self) -> &str {
        "Material Settings"
    }

    fn show(&mut self, ui: &imgui::Ui) {
        ui.combo(
            "Material",
            &mut self.selected_index.borrow_mut(),
            self.material_names.as_slice(),
            |mat| std::borrow::Cow::from(mat),
        );
    }
}

struct State {
    vertex_array_object: gl::VertexArrayId,
    cube_program: gl::ProgramId,
    light_program: gl::ProgramId,

    camera: Camera,
    materials: Vec<Material>,
    selected_index: Rc<RefCell<usize>>,

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
            gl::ComponentSize::SIZE_3,
            gl::ComponentType::FLOAT,
            false,
            size_of::<f32>() * 8,
            size_of::<f32>() * 3,
        );
        gl::enable_vertex_attrib_array(1);
        gl::bind_vertex_array(gl::VertexArrayId::NO_VERTEX_ARRAY);

        let cube_program = utils::program(
            include_str!("../assets/cube.vert"),
            include_str!("../assets/cube.frag"),
        )?;

        let light_program = utils::program(
            include_str!("../assets/light.vert"),
            include_str!("../assets/light.frag"),
        )?;

        let materials: Vec<JsonMaterial> =
            serde_json::from_str(include_str!("../assets/materials.json"))?;
        let materials: Vec<Material> = materials
            .into_iter()
            .map(|json_material| json_material.into())
            .collect();

        let selected_index = Rc::new(RefCell::new(0));
        let material_names = materials
            .iter()
            .map(|material| material.name().to_string())
            .collect();
        Ok(Self {
            vertex_array_object,
            cube_program,
            light_program,

            camera: Camera::new(glm::vec3(0., 0., 3.), glm::vec3(0., 1., 0.), -90., 0.),
            materials,
            selected_index: selected_index.clone(),

            views: vec![Box::new(MaterialSettingsView::new(
                material_names,
                selected_index.clone(),
            ))],
        })
    }
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

        gl::clear_color(0xFF334C4C);
        gl::clear(gl::ClearMask::COLOR_BUFFER_BIT | gl::ClearMask::DEPTH_BUFFER_BIT);

        let projection = glm::perspective(800. / 600., self.camera.zoom().to_radians(), 0.1, 100.);
        let view = self.camera.view_matrix();

        gl::bind_vertex_array(self.vertex_array_object);

        gl::use_program(self.cube_program);
        let model = glm::Mat4::identity();
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
        gl::uniform_matrix_4fv(
            gl::uniform_location(self.cube_program, "model"),
            false,
            glm::value_ptr(&model),
        );
        gl::uniform_3fv(
            gl::uniform_location(self.cube_program, "viewPos"),
            glm::value_ptr(self.camera.position()),
        );

        let material = &self.materials[*self.selected_index.borrow()];
        gl::uniform_3fv(
            gl::uniform_location(self.cube_program, "material.ambient"),
            glm::value_ptr(material.ambient()),
        );
        gl::uniform_3fv(
            gl::uniform_location(self.cube_program, "material.diffuse"),
            glm::value_ptr(material.diffuse()),
        );
        gl::uniform_3fv(
            gl::uniform_location(self.cube_program, "material.specular"),
            glm::value_ptr(material.specular()),
        );
        gl::uniform_1f(
            gl::uniform_location(self.cube_program, "material.shininess"),
            material.shininess(),
        );

        gl::uniform_3fv(
            gl::uniform_location(self.cube_program, "light.position"),
            glm::value_ptr(&glm::vec3(1.2, 1., 2.)),
        );
        gl::uniform_3fv(
            gl::uniform_location(self.cube_program, "light.ambient"),
            glm::value_ptr(&glm::vec3(1., 1., 1.)),
        );
        gl::uniform_3fv(
            gl::uniform_location(self.cube_program, "light.diffuse"),
            glm::value_ptr(&glm::vec3(1., 1., 1.)),
        );
        gl::uniform_3fv(
            gl::uniform_location(self.cube_program, "light.specular"),
            glm::value_ptr(&glm::vec3(1., 1., 1.)),
        );

        gl::draw_arrays(gl::DrawMode::TRIANGLES, 0, 36);

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
