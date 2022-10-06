#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::must_use_candidate)]
extern crate alloc;
extern crate core;
extern crate gl;
extern crate sdl2;

use std::f32::consts::PI;
use std::path::Path;

use gl::types::{GLfloat, GLintptr, GLsizei};

use crate::renderer::{Buffer, BufferUsage, RenderPass, Shader, ShaderKind, Texture, VertexAttribute, VertexBinding};
use crate::resources::Resources;

pub mod renderer;
pub mod resources;

#[derive(Copy, Clone)]
#[repr(C, packed)]
struct Vertex(f32, f32, f32);

#[derive(Copy, Clone)]
#[repr(C, packed)]
struct Color(f32, f32, f32);

#[derive(Copy, Clone)]
#[repr(C, packed)]
struct VertexData(Vertex, Color);

fn main() -> Result<(), String> {
    type Mat4 = nalgebra_glm::TMat4<f32>;

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

    let vertex_buffer = initialize_vertices()?;
    let index_buffer = initialize_indices()?;
    let mut model_matrix = Buffer::allocate(BufferUsage::Uniform, std::mem::size_of::<Mat4>())
        .expect("Failed to initialize model matrix");

    let perspective = nalgebra_glm::perspective(1f32, PI / 3f32, 0.001f32, 100f32);
    let view = nalgebra_glm::look_at(
        &nalgebra_glm::vec3(0f32, 0f32, 4f32),
        &nalgebra_glm::vec3(0f32, 0f32, 0f32),
        &nalgebra_glm::vec3(0f32, 1f32, 0f32));

    let vertex_bindings = [
        VertexBinding::new(0, VertexAttribute::new(renderer::VertexAttributeFormat::RGB32F, 0)),
        VertexBinding::new(1, VertexAttribute::new(renderer::VertexAttributeFormat::RG32F, 0)),
    ];

    let res = Resources::from_relative_exe_path(Path::new("assets"))
        .expect("Failed to open resources folder");
    let texture = Texture::from(&mut res.load_image("/textures/cube.tga").unwrap())
        .expect("Failed to load texture");
    let vertex_shader = Shader::from_source(
        res.load_cstring("/shaders/triangle.vert").unwrap().to_str().unwrap(), ShaderKind::Vertex)
        .expect("Failed to create vertex shader");
    let fragment_shader = Shader::from_source(
        res.load_cstring("/shaders/triangle.frag").unwrap().to_str().unwrap(), ShaderKind::Fragment)
        .expect("Failed to create fragment shader");
    let render_pass = RenderPass::new(&vertex_shader, &fragment_shader, &vertex_bindings, &[&model_matrix], &[texture])
        .expect("Failed to initialize render pass");

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LESS);
    }

    let mut angle = 0f32;
    let matrix_ptr = model_matrix.map::<Mat4>();
    let mut event_pump = sdl.event_pump().expect("Failed to get event pump");
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } |
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Escape), .. } =>
                    break 'main Ok(()),
                _ => {}
            }
        }

        angle += 0.001f32;
        let model = nalgebra_glm::rotation(angle, &nalgebra_glm::vec3(0f32, 1f32, 0.5f32));
        let matrix = perspective * view * model;
        matrix_ptr.copy_from_slice(&[matrix]);

        clear_screen(0.3, 0.3, 0.5);

        unsafe {
            render_pass.display();
            gl::BindVertexBuffer(0, vertex_buffer.handle(),
                                 0 as GLintptr,
                                 GLsizei::try_from(std::mem::size_of::<f32>() * 3).unwrap());
            gl::BindVertexBuffer(1, vertex_buffer.handle(),
                                 GLintptr::try_from(std::mem::size_of::<f32>() * 72).unwrap(),
                                 GLsizei::try_from(std::mem::size_of::<f32>() * 2).unwrap());
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, index_buffer.handle());
            let count = GLsizei::try_from(index_buffer.size() / std::mem::size_of::<u16>())
                .unwrap();
            gl::DrawElements(gl::TRIANGLES, count, gl::UNSIGNED_SHORT, std::ptr::null());
        }

        window.gl_swap_window();
    }
}

/// # Errors
/// - Fail to initialize vertex buffer
fn initialize_vertices() -> Result<Buffer, String> {
    let vertices = vec![
        // Vertices
        // Front face
        -0.5f32, 0.5f32, 0.5f32, -0.5f32, -0.5f32, 0.5f32, 0.5f32, -0.5f32, 0.5f32, 0.5f32, 0.5f32, 0.5f32,
        // Right face
        0.5f32, 0.5f32, 0.5f32, 0.5f32, -0.5f32, 0.5f32, 0.5f32, -0.5f32, -0.5f32, 0.5f32, 0.5f32, -0.5f32,
        // Back face
        0.5f32, 0.5f32, -0.5f32, 0.5f32, -0.5f32, -0.5f32, -0.5f32, -0.5f32, -0.5f32, -0.5f32, 0.5f32, -0.5f32,
        // Left face
        -0.5f32, 0.5f32, -0.5f32, -0.5f32, -0.5f32, -0.5f32, -0.5f32, -0.5f32, 0.5f32, -0.5f32, 0.5f32, 0.5f32,
        // Top face
        -0.5f32, 0.5f32, -0.5f32, -0.5f32, 0.5f32, 0.5f32, 0.5f32, 0.5f32, 0.5f32, 0.5f32, 0.5f32, -0.5f32,
        // Bottom face
        -0.5f32, -0.5f32, 0.5f32, -0.5f32, -0.5f32, -0.5f32, 0.5f32, -0.5f32, -0.5f32, 0.5f32, -0.5f32, 0.5f32,

        // Front face
        0.0f32, 1.0f32, 0.0f32, 0.0f32, 1.0f32, 0.0f32, 1.0f32, 1.0f32,
        // Right face
        0.0f32, 1.0f32, 0.0f32, 0.0f32, 1.0f32, 0.0f32, 1.0f32, 1.0f32,
        // Back face
        0.0f32, 1.0f32, 0.0f32, 0.0f32, 1.0f32, 0.0f32, 1.0f32, 1.0f32,
        // Left face
        0.0f32, 1.0f32, 0.0f32, 0.0f32, 1.0f32, 0.0f32, 1.0f32, 1.0f32,
        // Top face
        0.0f32, 1.0f32, 0.0f32, 0.0f32, 1.0f32, 0.0f32, 1.0f32, 1.0f32,
        // Bottom face
        0.0f32, 1.0f32, 0.0f32, 0.0f32, 1.0f32, 0.0f32, 1.0f32, 1.0f32,
    ];
    let mut vertex_buffer = Buffer::allocate(BufferUsage::Vertex, std::mem::size_of::<f32>() * vertices.len())?;
    let ptr = vertex_buffer.map::<f32>();
    ptr.copy_from_slice(&vertices);
    vertex_buffer.unmap();
    Ok(vertex_buffer)
}

/// # Errors
/// - Fail to initialize index buffer
pub fn initialize_indices() -> Result<Buffer, String> {
    let indices = vec![
        // Front face
        0u16, 1u16, 2u16, 0u16, 2u16, 3u16,
        // Right face
        4u16, 5u16, 6u16, 4u16, 6u16, 7u16,
        // Back face
        8u16, 9u16, 10u16, 8u16, 10u16, 11u16,
        // Left face
        12u16, 13u16, 14u16, 12u16, 14u16, 15u16,
        // Top face
        16u16, 17u16, 18u16, 16u16, 18u16, 19u16,
        // Bottom face
        20u16, 21u16, 22u16, 20u16, 22u16, 23u16,
    ];
    let mut index_buffer = Buffer::allocate(BufferUsage::Index, std::mem::size_of::<u16>() * indices.len())?;
    let ptr = index_buffer.map::<u16>();
    ptr.copy_from_slice(&indices);
    index_buffer.unmap();
    Ok(index_buffer)
}

fn clear_screen(red: f32, green: f32, blue: f32) {
    unsafe {
        gl::ClearColor(
            red as GLfloat,
            green as GLfloat,
            blue as GLfloat,
            1f32 as GLfloat);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }
}
