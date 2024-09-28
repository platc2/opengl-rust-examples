extern crate anyhow;
extern crate learnopengl_utils as utils;
extern crate nalgebra_glm as glm;
extern crate renderer;

use std::time::Instant;

use anyhow::Result;

use renderer::application;
use renderer::application::Application;
use renderer::input_manager::InputManager;
use renderer::renderer_context::{OpenGLVersion, RendererContext, WindowDimension};
use renderer::time::Time;
use utils::gl;

struct State {
    shader_program: gl::ProgramId,
    vertex_array_object: gl::VertexArrayId,
}

impl Application for State {
    fn tick(&mut self, _: &Time<Instant>, _: &dyn InputManager) {
        gl::viewport((0, 0), (800, 600));

        gl::clear_color(0xFF334C4C);
        gl::clear(gl::ClearMask::COLOR_BUFFER_BIT);

        gl::use_program(self.shader_program);
        gl::bind_vertex_array(self.vertex_array_object);
        gl::uniform_2f(gl::uniform_location(self.shader_program, "offset"), 0., 0.);
        gl::draw_arrays(gl::DrawMode::TRIANGLES, 0, 3);

        gl::uniform_2f(gl::uniform_location(self.shader_program, "offset"), 0.5, 0.);
        gl::draw_arrays(gl::DrawMode::TRIANGLES, 0, 3);
    }
}

impl State {
    pub fn new(shader_program: gl::ProgramId, vertex_array_object: gl::VertexArrayId) -> Self {
        Self {
            shader_program,
            vertex_array_object,
        }
    }
}

pub fn main() -> Result<()> {
    let context = RendererContext::init(
        "LearnOpenGL",
        &WindowDimension::of(800, 600),
        &OpenGLVersion::of(3, 3),
    )?;

    let vertex_data: [f32; 18] = [
        0.5, -0.5, 0., 1., 0., 0.,
        -0.5, -0.5, 0., 0., 1., 0.,
        0., 0.5, 0., 0., 0., 1.
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
        size_of::<f32>() * 6,
        0);
    gl::enable_vertex_attrib_array(0);
    gl::vertex_attrib_pointer(
        1,
        gl::ComponentSize::SIZE_3,
        gl::ComponentType::FLOAT,
        false,
        size_of::<f32>() * 6,
        size_of::<f32>() * 3,
    );
    gl::enable_vertex_attrib_array(1);
    gl::bind_vertex_array(gl::VertexArrayId::NO_VERTEX_ARRAY);

    let shader_program = utils::program(
        include_str!("../assets/triangle.vert"),
        include_str!("../assets/triangle.frag"),
    )?;

    let state = State::new(shader_program, vertex_array_object);

    application::main_loop(context, state)
}
