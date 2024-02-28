#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::must_use_candidate)]

extern crate alloc;
extern crate core;
extern crate gl_bindings as gl;
extern crate imgui;
extern crate renderer;
extern crate sdl2;

use std::path::Path;

use anyhow::{Context, Result};

use renderer::{application, Buffer, BufferUsage, RenderPass, Shader, ShaderKind, VertexAttribute, VertexBinding};
use renderer::renderer_context::{OpenGLVersion, RendererContext, WindowDimension};
use renderer::resources::Resources;

use state::State;

mod state;

fn main() -> Result<()> {
    let context = RendererContext::init(
        "Hello Triangle",
        &WindowDimension::of(900, 700),
        &OpenGLVersion::of(4, 5),
    )?;

    let res = Resources::from_relative_exe_path(Path::new("assets"))?;

    let vertex_buffer = initialize_vertices()?;

    let vertex_shader = res
        .load_string("/shaders/triangle.vert")
        .map_err(Into::into)
        .and_then(|source| Shader::from_source(&source, ShaderKind::Vertex))
        .context("Failed to initialize vertex shader")?;
    let fragment_shader = res
        .load_string("/shaders/triangle.frag")
        .map_err(Into::into)
        .and_then(|source| Shader::from_source(&source, ShaderKind::Fragment))
        .context("Failed to initialize fragment shader")?;

    let gamma_buffer = Buffer::allocate(BufferUsage::Uniform, std::mem::size_of::<f32>())?;

    let vertex_bindings = [
        VertexBinding::new(
            0,
            VertexAttribute::new(renderer::VertexAttributeFormat::RG32F, 0),
        ),
        VertexBinding::new(
            1,
            VertexAttribute::new(renderer::VertexAttributeFormat::RGB32F, 0),
        ),
    ];

    let main_render_pass = RenderPass::new(
        &vertex_shader,
        &fragment_shader,
        &vertex_bindings,
        &[&gamma_buffer],
        &[],
        &[],
    )?;

    let state = State::new(
        main_render_pass,
        gamma_buffer,
        vertex_buffer,
    );

    application::main_loop(context, state)
}

/// # Errors
/// - Fail to initialize vertex buffer
fn initialize_vertices() -> Result<Buffer> {
    let vertices = vec![
        -0.5f32, -0.5f32, 1f32, 0f32, 0f32, 0.5f32, -0.5f32, 0f32, 1f32, 0f32, 0f32, 0.5f32, 0f32,
        0f32, 1f32,
    ];
    let mut vertex_buffer = Buffer::allocate(
        BufferUsage::Vertex,
        std::mem::size_of::<f32>() * vertices.len(),
    )?;
    let ptr = vertex_buffer.map::<f32>();
    ptr.copy_from_slice(&vertices);
    vertex_buffer.unmap();
    Ok(vertex_buffer)
}
