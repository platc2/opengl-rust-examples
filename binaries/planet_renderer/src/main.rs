#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::must_use_candidate)]

extern crate gl_bindings as gl;

use std::path::Path;

use anyhow::{Context, Result};
use nalgebra_glm as glm;

use renderer::{application, Buffer, BufferUsage, RenderPass, Shader, ShaderKind, VertexAttribute, VertexBinding};
use renderer::renderer_context::{OpenGLVersion, RendererContext, WindowDimension};
use renderer::resources::Resources;

use crate::matrix_uniform::MatrixUniform;
use crate::state::State;

mod matrix_uniform;
mod camera;
mod planet;
mod icosahedron;
mod polyhedron;
mod transform;
mod movable;
mod frustum;
mod state;

pub fn main() -> Result<()> {
    let window_dimension = WindowDimension::default();
    // Initialize render-context
    let context = RendererContext::init(
        "Planet Renderer",
        &window_dimension,
        &OpenGLVersion::default(),
    )?;

    let res = Resources::from_relative_exe_path(Path::new("assets"))?;

    let matrix_uniform_buffer =
        Buffer::allocate(BufferUsage::Uniform, std::mem::size_of::<MatrixUniform>())?;

    let planet_vertex_shader = res
        .load_string("/shaders/planet.vert")
        .map_err(Into::into)
        .and_then(|source| Shader::from_source(&source, ShaderKind::Vertex))
        .context("Failed to initialize terrain vertex shader")?;
    let planet_fragment_shader = res
        .load_string("/shaders/planet.frag")
        .map_err(Into::into)
        .and_then(|source| Shader::from_source(&source, ShaderKind::Fragment))
        .context("Failed to initialize terrain fragment shader")?;

    let vertex_bindings = [VertexBinding::new(
        0,
        VertexAttribute::new(renderer::VertexAttributeFormat::RGB32F, 0),
    )];

    let main_render_pass = RenderPass::new(
        &planet_vertex_shader,
        &planet_fragment_shader,
        &vertex_bindings,
        &[&matrix_uniform_buffer],
        &[],
        &[],
    )?;

    let frustum_vbx = Buffer::allocate(BufferUsage::Vertex, std::mem::size_of::<glm::Vec3>() * 8)?;
    let frustum_idx = Buffer::allocate(BufferUsage::Index, std::mem::size_of::<u16>() * 24)?;

    let state = State::new(
        frustum_vbx,
        frustum_idx,
        matrix_uniform_buffer,
        main_render_pass,
    );

    application::main_loop(context, state)
}
