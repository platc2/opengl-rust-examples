extern crate anyhow;
extern crate nalgebra_glm as glm;
extern crate renderer;

use std::time::Instant;

use anyhow::{anyhow, Result};

use renderer::{application, Texture};
use renderer::application::Application;
use renderer::input_manager::{InputManager, Key};
use renderer::renderer_context::{OpenGLVersion, RendererContext, WindowDimension};
use renderer::time::Time;

struct State {
    shader_program: gl::ProgramId,
    vertex_array_object: gl::VertexArrayId,

    quit: bool,
}

mod gl {
    pub use renderer::gl::buffer::*;
    pub use renderer::gl::program::*;
    pub use renderer::gl::rendering::*;
    pub use renderer::gl::shader::*;
    pub use renderer::gl::state::*;
    pub use renderer::gl::vertex_array::*;
    pub use renderer::gl::vertex_attrib::*;
}

impl Application for State {
    fn tick(&mut self, _: &Time<Instant>, input_manager: &dyn InputManager) {
        if input_manager.key_down(Key::ESCAPE) {
            self.quit = true;
        }

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

    fn quit(&self) -> bool { self.quit }
}

impl State {
    pub fn new(shader_program: gl::ProgramId, vertex_array_object: gl::VertexArrayId) -> Self {
        Self {
            shader_program,
            vertex_array_object,
            quit: false,
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

    let shader_program = program(
        include_str!("../assets/triangle.vert"),
        include_str!("../assets/triangle.frag"),
    )?;

    let state = State::new(shader_program, vertex_array_object);

    application::main_loop(context, state)
}

fn shader(shader_kind: gl::ShaderKind, shader_source: &str) -> Result<gl::ShaderId> {
    let shader = gl::create_shader(shader_kind);
    gl::shader_source(shader, shader_source);
    gl::compile_shader(shader);
    if gl::shader_compile_status(shader) {
        Ok(shader)
    } else {
        let info_log = gl::shader_info_log(shader);
        Err(anyhow!("Error compiling {shader_kind:?} shader: {}", info_log.unwrap_or("Unknown error".to_owned())))
    }
}

fn program(
    vertex_shader_source: &str,
    fragment_shader_source: &str,
) -> Result<gl::ProgramId> {
    let program = gl::create_program();
    let mut vertex_shader = shader(gl::ShaderKind::VERTEX_SHADER, vertex_shader_source)?;
    let mut fragment_shader = shader(gl::ShaderKind::FRAGMENT_SHADER, fragment_shader_source)?;
    gl::attach_shader(program, vertex_shader);
    gl::attach_shader(program, fragment_shader);
    gl::link_program(program);

    gl::delete_shader(&mut vertex_shader);
    gl::delete_shader(&mut fragment_shader);

    if gl::program_link_status(program) {
        Ok(program)
    } else {
        let info_log = gl::program_info_log(program);
        Err(anyhow!("Failed to link shader program: {}", info_log.unwrap_or("Unknown error".to_owned())))
    }
}
