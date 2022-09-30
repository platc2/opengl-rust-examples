#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::must_use_candidate)]
extern crate core;
extern crate gl;
extern crate sdl2;

use crate::renderer::buffer::Usage;
use crate::renderer::render_pass::VertexBinding;
use crate::renderer::VertexAttribute;

pub mod renderer;

#[derive(Copy, Clone)]
#[repr(C, packed)]
struct Vertex(f32, f32, f32);

#[derive(Copy, Clone)]
#[repr(C, packed)]
struct Color(f32, f32, f32);

#[derive(Copy, Clone)]
#[repr(C, packed)]
struct VertexData(Vertex, Color);


fn main() {
    let sdl = sdl2::init().expect("Failed to initialize SDL2 context");
    let video_subsystem = sdl.video().expect("Failed to initialize video subsystem");
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 1);
    let window = video_subsystem
        .window("Hello Triangle", 900, 700)
        .opengl()
        .resizable()
        .build()
        .expect("Failed to create window");
    let _gl_context = window.gl_create_context().expect("Failed to create OpenGL context");
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s).cast::<std::ffi::c_void>());

    let render_pass = renderer::RenderPass::new(
        include_str!("triangle.vert"),
        include_str!("triangle.frag"),
        &[
            VertexBinding::new(0, VertexAttribute::new(renderer::vertex_attribute::Format::RGB32F, 0)),
            VertexBinding::new(1, VertexAttribute::new(renderer::vertex_attribute::Format::RGB32F, 0))
        ])
        .expect("Failed to initialize render pass");

    let vertices: Vec<VertexData> = vec![
        VertexData(Vertex(-0.5, -0.5, 0.0), Color(1.0, 0.0, 0.0)),
        VertexData(Vertex(0.5, -0.5, 0.0), Color(0.0, 1.0, 0.0)),
        VertexData(Vertex(0.0, 0.5, 0.0), Color(0.0, 0.0, 1.0)),
    ];
    let data_length = std::mem::size_of::<VertexData>() * vertices.len();

    let mut vertex_buffer = renderer::Buffer::allocate(Usage::Vertex, data_length)
        .expect("Failed to allocate vertex buffer");
    let data_pointer = vertex_buffer.map::<VertexData>();
    data_pointer.copy_from_slice(&vertices);
    vertex_buffer.unmap();

    unsafe {
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    let mut event_pump = sdl.event_pump().expect("Failed to get event pump");
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } |
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Escape), .. } =>
                    break 'main,
                _ => {}
            }
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        unsafe {
            let stride = gl::types::GLsizei::try_from(std::mem::size_of::<VertexData>())
                .unwrap();
            gl::BindVertexBuffer(0, vertex_buffer.handle(),
                                 0 as gl::types::GLintptr, stride);
            gl::BindVertexBuffer(1, vertex_buffer.handle(),
                                 gl::types::GLintptr::try_from(std::mem::size_of::<Vertex>()).unwrap(),
                                 stride);
            render_pass.display();
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        window.gl_swap_window();
    }
}
