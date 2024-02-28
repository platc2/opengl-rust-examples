#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::must_use_candidate)]

extern crate alloc;
extern crate core;
extern crate gl_bindings as gl;
extern crate imgui;
extern crate sdl2;

use core::fmt::{Display, Formatter};
use std::path::Path;

use anyhow::{Context, Result};

use renderer::{application, Buffer, BufferUsage, RenderPass, Shader, ShaderKind, Texture, VertexAttribute, VertexAttributeFormat, VertexBinding};
use renderer::renderer_context::{OpenGLVersion, RendererContext, WindowDimension};
use renderer::resources::Resources;

use crate::state::State;

mod state;


#[derive(Default)]
struct WGS84Coordinate {
    longitude: f32,
    latitude: f32,
}

impl WGS84Coordinate {
    pub fn offset(&mut self, longitude: f32, latitude: f32) {
        self.longitude = (self.longitude + longitude + 180f32).rem_euclid(360f32) - 180f32;
        self.latitude = (self.latitude + latitude + 90f32).rem_euclid(180f32) - 90f32;
    }
}

impl Display for WGS84Coordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let longitude = self.longitude.abs();
        let latitude = self.latitude.abs();
        write!(
            f,
            "{:3}°{:5.2}'{} {:3}°{:5.2}'{}",
            longitude.trunc(),
            longitude.fract() * 60f32,
            if self.longitude < 0f32 { 'W' } else { 'E' },
            latitude.trunc(),
            latitude.fract() * 60f32,
            if self.latitude > 0f32 { 'N' } else { 'S' }
        )
    }
}

#[derive(Default)]
struct Position {
    pub pos: WGS84Coordinate,
    pub altitude: f32,
    pub bearing: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
struct CameraSettings {
    pub position: nalgebra_glm::TVec3<f32>,
    pub direction: nalgebra_glm::TVec3<f32>,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct WorldSettings {
    pub time: f32,
    pub planet_radius: f32,
    pub atmosphere_height: f32,
    pub inscatter_points: u32,
    pub optical_depth_points: u32,
    pub g: f32,
    pub intensity: f32,
    pub rayleigh_scale_height: f32,
    pub mie_scale_height: f32,
}

fn main() -> Result<()> {
    // Planet looks better if screen is a square
    let window_dimension = WindowDimension::of(800, 800);

    let context = RendererContext::init(
        "Atmospheric Scattering",
        &window_dimension,
        &OpenGLVersion::default(),
    )
        .context("Failed to initialize renderer context")?;

    let res = Resources::from_relative_exe_path(Path::new("assets"))
        .context("Failed to initialize resources")?;

    let vertices = vec![
        -1f32, 1f32, -1f32, -1f32, 1f32, -1f32, 1f32, -1f32, 1f32, 1f32, -1f32, 1f32,
    ];
    let mut vertex_buffer = Buffer::allocate(
        BufferUsage::Vertex,
        std::mem::size_of::<f32>() * vertices.len(),
    )
        .context("Failed to allocate vertex buffer")?;
    let vertex_ptr = vertex_buffer.map();
    vertex_ptr.copy_from_slice(&vertices);
    vertex_buffer.unmap();

    let camera_settings_buffer =
        Buffer::allocate(BufferUsage::Uniform, std::mem::size_of::<CameraSettings>())
            .context("Failed to allocate camera settings buffer")?;

    let world_settings_buffer =
        Buffer::allocate(BufferUsage::Uniform, std::mem::size_of::<WorldSettings>())
            .context("Failed to allocate world settings buffer")?;

    let vertex = res.load_string("/shaders/cube.vert")
        .map_err(Into::into)
        .and_then(|source| Shader::from_source(&source, ShaderKind::Vertex))
        .context("Failed to initialize cube vertex shader")?;
    let planet_fragment = res.load_string("/shaders/planet.frag")
        .map_err(Into::into)
        .and_then(|source| Shader::from_source(&source, ShaderKind::Fragment))
        .context("Failed to initialize planet fragment shader")?;

    let planet_vertex_bindings = [VertexBinding::new(
        0,
        VertexAttribute::new(VertexAttributeFormat::RG32F, 0),
    )];
    let planet_texture = Texture::blank(window_dimension.width, window_dimension.height);
    let planet_render_pass = RenderPass::new(
        &vertex,
        &planet_fragment,
        &planet_vertex_bindings,
        &[&camera_settings_buffer, &world_settings_buffer],
        &[],
        &[&planet_texture],
    )
        .context("Failed to initialize planet render pass")?;

    let fragment = res.load_string("/shaders/sky.frag")
        .map_err(Into::into)
        .and_then(|source| Shader::from_source(&source, ShaderKind::Fragment))
        .context("Failed to initialize sky fragment shader")?;

    let vertex_bindings = [VertexBinding::new(
        0,
        VertexAttribute::new(VertexAttributeFormat::RG32F, 0),
    )];

    let render_pass = RenderPass::new(
        &vertex,
        &fragment,
        &vertex_bindings,
        &[&camera_settings_buffer, &world_settings_buffer],
        &[&planet_texture],
        &[],
    )
        .context("Failed to initialize render pass")?;

    let state = State::new(
        world_settings_buffer,
        camera_settings_buffer,
        planet_render_pass,
        render_pass,
        vertex_buffer,
    );

    application::main_loop(context, state)
}
