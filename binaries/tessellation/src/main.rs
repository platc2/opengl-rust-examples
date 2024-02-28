#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::must_use_candidate)]

extern crate alloc;
extern crate core;
extern crate gl_bindings as gl;
extern crate sdl2;

use std::path::Path;

use anyhow::{Context, Result};

use renderer::{application, Buffer, BufferUsage, RenderPass, Shader, ShaderKind, VertexAttribute, VertexBinding};
use renderer::renderer_context::{OpenGLVersion, RendererContext, WindowDimension};
use renderer::resources::Resources;

use crate::state::State;

mod state;

#[derive(Default, Copy, Clone)]
pub struct TessellationParameters {
    outer: [u32; 4 * 4],
    inner: [u32; 2 * 4],
}

fn main() -> Result<()> {
    // Initialize render-context
    let context = RendererContext::init(
        "Tessellation",
        &WindowDimension::default(),
        &OpenGLVersion::default(),
    )?;

    let res = Resources::from_relative_exe_path(Path::new("assets"))?;

    let tessellation_parameters_buffer = Buffer::allocate(
        BufferUsage::Uniform,
        std::mem::size_of::<TessellationParameters>(),
    )?;

    let vertex_buffer = initialize_vertices()?;

    let vertex_shader = Shader::from_source(
        &res.load_string("/shaders/basic.vert")?,
        ShaderKind::Vertex)
        .context("Failed to initialize vertex shader")?;
    let fragment_shader = Shader::from_source(
        &res.load_string("/shaders/basic.frag")?,
        ShaderKind::Fragment)
        .context("Failed to initialize fragment shader")?;
    let tessellation_control_shader = Shader::from_source(
        &res.load_string("/shaders/basic.tesc")?,
        ShaderKind::TessellationControl)
        .context("Failed to initialize tessellation control shader")?;
    let tessellation_evaluation_shader = Shader::from_source(
        &res.load_string("/shaders/basic.tese")?,
        ShaderKind::TessellationEvaluation)
        .context("Failed to initialize tessellation evaluation shader")?;

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

    let main_render_pass = RenderPass::new_tess(
        &vertex_shader,
        &fragment_shader,
        &tessellation_control_shader,
        &tessellation_evaluation_shader,
        &vertex_bindings,
        &[&tessellation_parameters_buffer],
        &[],
        &[],
    )?;

    let state = State::new(
        main_render_pass,
        tessellation_parameters_buffer,
        vertex_buffer,
    );

    application::main_loop(context, state)
}

/// # Errors
/// - Fail to initialize vertex buffer
fn initialize_vertices() -> Result<Buffer> {
    let vertices = vec![
        -0.5, -0.5,
        1., 0., 0.,
        0.5, -0.5,
        0., 1., 0.,
        0., -0.5 + (3f32.sqrt() / 2.),
        0., 0., 1.,
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
