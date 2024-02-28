#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::must_use_candidate)]
extern crate core;
extern crate gl_bindings as gl;
extern crate imgui;
extern crate nalgebra_glm as glm;
extern crate noise;
extern crate sdl2;

use std::path::Path;

use anyhow::{Context, Result};

use renderer::{application, Buffer, BufferUsage, RenderPass, Shader, ShaderKind, Texture, VertexAttribute, VertexBinding};
use renderer::renderer_context::{OpenGLVersion, RendererContext, WindowDimension};
use renderer::resources::Resources;

use crate::matrix_uniform::MatrixUniform;
use crate::state::State;

mod chunk;
mod terrain_mesh;
mod matrix_uniform;
mod camera;
mod state;

fn main() -> Result<()> {
    let window_dimension = WindowDimension::default();
    // Initialize render-context
    let context = RendererContext::init(
        "Bruteforce Terrain",
        &window_dimension,
        &OpenGLVersion::default(),
    )?;

    let res = Resources::from_relative_exe_path(Path::new("assets"))?;

    let matrix_uniform_buffer =
        Buffer::allocate(BufferUsage::Uniform, std::mem::size_of::<MatrixUniform>())?;
    let mut matrix_uniforms = MatrixUniform::default();
    matrix_uniforms.model = nalgebra_glm::TMat4::identity();
    matrix_uniforms.projection = nalgebra_glm::perspective(
        window_dimension.width as f32 / window_dimension.height as f32,
        60f32.to_radians(),
        0.01f32,
        100f32,
    );

    let terrain_texture = Texture::blank(0, 0);

    let terrain_vertex_shader = res
        .load_string("/shaders/terrain.vert")
        .map_err(Into::into)
        .and_then(|source| Shader::from_source(&source, ShaderKind::Vertex))
        .context("Failed to initialize terrain vertex shader")?;
    let terrain_fragment_shader = res
        .load_string("/shaders/terrain.frag")
        .map_err(Into::into)
        .and_then(|source| Shader::from_source(&source, ShaderKind::Fragment))
        .context("Failed to initialize terrain fragment shader")?;
    let terrain_geometry_shader = res
        .load_string("/shaders/terrain.geom")
        .map_err(Into::into)
        .and_then(|source| Shader::from_source(&source, ShaderKind::Geometry))
        .context("Failed to initialize terrain geometry shader")?;

    let vertex_bindings = [VertexBinding::new(
        0,
        VertexAttribute::new(renderer::VertexAttributeFormat::RGB32F, 0),
    )];

    let grass_texture = res
        .load_image("/textures/grass.jpg")
        .map_err(Into::into)
        .and_then(|mut image_data| Texture::from(image_data.as_mut_slice()))
        .context("Failed to load grass texture")?;
    let sand_texture = res
        .load_image("/textures/sand.jpg")
        .map_err(Into::into)
        .and_then(|mut image_data| Texture::from(image_data.as_mut_slice()))
        .context("Failed to load sand texture")?;
    let stone_texture = res
        .load_image("/textures/stone.jpg")
        .map_err(Into::into)
        .and_then(|mut image_data| Texture::from(image_data.as_mut_slice()))
        .context("Failed to load stone texture")?;
    let snow_texture = res
        .load_image("/textures/snow.jpg")
        .map_err(Into::into)
        .and_then(|mut image_data| Texture::from(image_data.as_mut_slice()))
        .context("Failed to load snow texture")?;

    let main_render_pass = RenderPass::new_geom(
        &terrain_vertex_shader,
        &terrain_fragment_shader,
        &terrain_geometry_shader,
        &vertex_bindings,
        &[&matrix_uniform_buffer],
        &[
            &terrain_texture,
            &grass_texture,
            &sand_texture,
            &stone_texture,
            &snow_texture,
        ],
        &[],
    )?;

    let state = State::new(
        matrix_uniforms,
        matrix_uniform_buffer,
        main_render_pass,
    );

    application::main_loop(context, state)
}
