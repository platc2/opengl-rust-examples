extern crate anyhow;
extern crate learnopengl_utils as utils;
extern crate nalgebra_glm as glm;
extern crate renderer;
extern crate russimp;
extern crate stb_image;

use std::path::Path;
use std::time::Instant;

use anyhow::{anyhow, Result};

use camera::{Camera, MovementDirection};
use model::Model;
use renderer::application;
use renderer::application::Application;
use renderer::input_manager::{InputManager, Key};
use renderer::renderer_context::{OpenGLVersion, RendererContext, WindowDimension};
use renderer::time::Time;
use utils::gl;

mod camera;
mod mesh;
mod texture;
mod vertex;
mod model;

struct State {
    vertex_array_object: gl::VertexArrayId,
    cube_program: gl::ProgramId,
    light_program: gl::ProgramId,

    camera: Camera,
    model: Model,

    shininess: u32,
}

impl Application for State {
    fn tick(&mut self, time: &Time<Instant>, input_manager: &dyn InputManager) {
        if input_manager.key_down(Key::W) {
            self.camera.process_keyboard(MovementDirection::FORWARD, time);
        }
        if input_manager.key_down(Key::S) {
            self.camera.process_keyboard(MovementDirection::BACKWARD, time);
        }
        if input_manager.key_down(Key::A) {
            self.camera.process_keyboard(MovementDirection::LEFT, time);
        }
        if input_manager.key_down(Key::D) {
            self.camera.process_keyboard(MovementDirection::RIGHT, time);
        }

        let mouse_movement = input_manager.mouse_movement();
        self.camera.process_mouse_movement((mouse_movement.0 as _, -mouse_movement.1 as _), true);

        let (_, scroll_y) = input_manager.scroll();
        self.camera.process_mouse_scroll(scroll_y as _);

        gl::viewport((0, 0), (800, 600));
        gl::enable(gl::Capability::DEPTH_TEST);

        gl::clear_color(0xFF334C4C);
        gl::clear(gl::ClearMask::COLOR_BUFFER_BIT | gl::ClearMask::DEPTH_BUFFER_BIT);

        let projection = glm::perspective(800. / 600., 45f32.to_radians(), 0.1, 100.);
        let view = self.camera.view_matrix();

        gl::bind_vertex_array(self.vertex_array_object);

        gl::use_program(self.cube_program);
        let model = glm::Mat4::identity();
        gl::uniform_matrix_4fv(gl::uniform_location(self.cube_program, "projection"), false, glm::value_ptr(&projection));
        gl::uniform_matrix_4fv(gl::uniform_location(self.cube_program, "view"), false, glm::value_ptr(&view));
        gl::uniform_matrix_4fv(gl::uniform_location(self.cube_program, "model"), false, glm::value_ptr(&model));
        gl::uniform_3fv(gl::uniform_location(self.cube_program, "objectColor"), glm::value_ptr(&glm::vec3(1., 0.5, 0.31)));
        gl::uniform_3fv(gl::uniform_location(self.cube_program, "lightColor"), glm::value_ptr(&glm::vec3(1., 1., 1.)));
        gl::uniform_3fv(gl::uniform_location(self.cube_program, "lightPos"), glm::value_ptr(&glm::vec3(1.2, 1., 2.)));
        gl::uniform_3fv(gl::uniform_location(self.cube_program, "viewPos"), glm::value_ptr(self.camera.position()));
        gl::uniform_1ui(gl::uniform_location(self.cube_program, "shininess"), self.shininess);

        self.model.draw(self.cube_program);

        gl::bind_vertex_array(self.vertex_array_object);
        gl::use_program(self.light_program);
        let model = glm::Mat4::identity();
        let model = glm::translate(&model, &glm::vec3(1.2, 1., 2.));
        let model = glm::scale(&model, &glm::vec3(0.2, 0.2, 0.2));
        gl::uniform_matrix_4fv(gl::uniform_location(self.light_program, "projection"), false, glm::value_ptr(&projection));
        gl::uniform_matrix_4fv(gl::uniform_location(self.light_program, "view"), false, glm::value_ptr(&view));
        gl::uniform_matrix_4fv(gl::uniform_location(self.light_program, "model"), false, glm::value_ptr(&model));
        gl::draw_arrays(gl::DrawMode::TRIANGLES, 0, 36);
    }

    fn gui(&mut self, ui: &imgui::Ui) {
        ui.window("Settings")
            .save_settings(false)
            .always_auto_resize(true)
            .build(|| {
                ui.slider("Shininess", 1, 256, &mut self.shininess);
            });
    }
}

impl State {
    pub fn new(vertex_array_object: gl::VertexArrayId, cube_program: gl::ProgramId, light_program: gl::ProgramId, model: Model) -> Self {
        Self {
            vertex_array_object,
            cube_program,
            light_program,

            camera: Camera::new(glm::vec3(0., 0., 3.), glm::vec3(0., 1., 0.), -90., 0.),
            model,

            shininess: 32,
        }
    }
}

pub fn main() -> Result<()> {
    let context = RendererContext::init(
        "LearnOpenGL",
        &WindowDimension::of(800, 600),
        &OpenGLVersion::of(3, 3),
    )?;

    let vertex_data: [f32; 216] = [
        -0.5, -0.5, -0.5, 0., 0., -1.,
        0.5, -0.5, -0.5, 0., 0., -1.,
        0.5, 0.5, -0.5, 0., 0., -1.,
        0.5, 0.5, -0.5, 0., 0., -1.,
        -0.5, 0.5, -0.5, 0., 0., -1.,
        -0.5, -0.5, -0.5, 0., 0., -1.,
        -0.5, -0.5, 0.5, 0., 0., 1.,
        0.5, -0.5, 0.5, 0., 0., 1.,
        0.5, 0.5, 0.5, 0., 0., 1.,
        0.5, 0.5, 0.5, 0., 0., 1.,
        -0.5, 0.5, 0.5, 0., 0., 1.,
        -0.5, -0.5, 0.5, 0., 0., 1.,
        -0.5, 0.5, 0.5, -1., 0., 0.,
        -0.5, 0.5, -0.5, -1., 0., 0.,
        -0.5, -0.5, -0.5, -1., 0., 0.,
        -0.5, -0.5, -0.5, -1., 0., 0.,
        -0.5, -0.5, 0.5, -1., 0., 0.,
        -0.5, 0.5, 0.5, -1., 0., 0.,
        0.5, 0.5, 0.5, 1., 0., 0.,
        0.5, 0.5, -0.5, 1., 0., 0.,
        0.5, -0.5, -0.5, 1., 0., 0.,
        0.5, -0.5, -0.5, 1., 0., 0.,
        0.5, -0.5, 0.5, 1., 0., 0.,
        0.5, 0.5, 0.5, 1., 0., 0.,
        -0.5, -0.5, -0.5, 0., -1., 0.,
        0.5, -0.5, -0.5, 0., -1., 0.,
        0.5, -0.5, 0.5, 0., -1., 0.,
        0.5, -0.5, 0.5, 0., -1., 0.,
        -0.5, -0.5, 0.5, 0., -1., 0.,
        -0.5, -0.5, -0.5, 0., -1., 0.,
        -0.5, 0.5, -0.5, 0., 1., 0.,
        0.5, 0.5, -0.5, 0., 1., 0.,
        0.5, 0.5, 0.5, 0., 1., 0.,
        0.5, 0.5, 0.5, 0., 1., 0.,
        -0.5, 0.5, 0.5, 0., 1., 0.,
        -0.5, 0.5, -0.5, 0., 1., 0.
    ];

    let vertex_array_object = gl::create_vertex_array();
    gl::bind_vertex_array(vertex_array_object);

    let triangle_vbo = gl::create_buffer();
    gl::bind_buffer(gl::BufferTarget::ARRAY_BUFFER, triangle_vbo);
    gl::buffer_data(gl::BufferTarget::ARRAY_BUFFER, &vertex_data, gl::BufferUsage::STATIC_DRAW);

    gl::vertex_attrib_pointer(
        0,
        gl::ComponentSize::SIZE_3,
        gl::ComponentType::FLOAT,
        false,
        core::mem::size_of::<f32>() * 6,
        0);
    gl::enable_vertex_attrib_array(0);
    gl::vertex_attrib_pointer(
        1,
        gl::ComponentSize::SIZE_3,
        gl::ComponentType::FLOAT,
        false,
        core::mem::size_of::<f32>() * 6,
        core::mem::size_of::<f32>() * 3,
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

    let binding = std::env::current_exe()?;
    let exe_file_name = binding.parent()
        .ok_or(anyhow!("Failed to fetch exe path"))?;
    let binding = exe_file_name.join(Path::new("assets/backpack/backpack.obj"));
    let model = binding
        .to_str()
        .ok_or(anyhow!("Failed to get model path"))?;
    let model = Model::new(model);

    let state = State::new(vertex_array_object, cube_program, light_program, model);

    application::main_loop(context, state)
}
