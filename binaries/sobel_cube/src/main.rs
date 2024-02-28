#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::must_use_candidate)]

extern crate alloc;
extern crate core;
extern crate gl_bindings as gl;
extern crate nalgebra_glm as glm;
extern crate sdl2;

use std::path::Path;

use anyhow::{Context, Result};

use renderer::{application, Buffer, BufferUsage, RenderPass, Shader, ShaderKind, Texture, VertexAttribute, VertexBinding};
use renderer::renderer_context::{OpenGLVersion, RendererContext, WindowDimension};
use renderer::resources::Resources;

use crate::state::State;

mod state;

type Mat3 = nalgebra_glm::TMat3<f32>;
type Vec3 = nalgebra_glm::TVec3<f32>;

pub struct KernelMatrix {
    pub label: String,
    pub matrix: Mat3,
}

#[allow(clippy::too_many_lines)]
fn main() -> Result<()> {
    type Mat4 = nalgebra_glm::TMat4<f32>;

    // Initialize render-context
    let context = RendererContext::init(
        "Sobel Cube",
        &WindowDimension::default(),
        &OpenGLVersion::default(),
    )?;

    unsafe {
        gl::sys::Enable(gl::sys::DEBUG_OUTPUT);
        gl::sys::Enable(gl::sys::DEBUG_OUTPUT_SYNCHRONOUS);
    }

    let res = Resources::from_relative_exe_path(Path::new("assets"))?;

    let vertex_buffer = initialize_vertices()?;
    let index_buffer = initialize_indices()?;

    let vertex_shader =
        Shader::from_source(&res.load_string("/shaders/basic.vert")?, ShaderKind::Vertex)
            .context("Failed to initialize basic vertex shader")?;
    let fragment_shader = Shader::from_source(
        &res.load_string("/shaders/basic.frag")?,
        ShaderKind::Fragment,
    )
        .context("Failed to initialize basic fragment shader")?;

    let matrix_buffer =
        Buffer::allocate(BufferUsage::Uniform, std::mem::size_of::<Mat4>() * 2)?;
    let texture_switch_buffer =
        Buffer::allocate(BufferUsage::Uniform, std::mem::size_of::<f32>())?;
    let light_buffer = Buffer::allocate(BufferUsage::Uniform, std::mem::size_of::<Vec3>())?;
    let kernel_buffer = Buffer::allocate(BufferUsage::Uniform, std::mem::size_of::<Mat3>())?;

    let vertex_bindings = [
        VertexBinding::new(
            0,
            VertexAttribute::new(renderer::VertexAttributeFormat::RGB32F, 0),
        ),
        VertexBinding::new(
            1,
            VertexAttribute::new(renderer::VertexAttributeFormat::RG32F, 0),
        ),
        VertexBinding::new(
            2,
            VertexAttribute::new(renderer::VertexAttributeFormat::RGB32F, 0),
        ),
    ];

    let cube_texture = Texture::from(&mut res.load_image("/textures/cube.tga")?)?;
    let floor_texture = Texture::from(&mut res.load_image("/textures/floor.tga")?)?;

    let render_texture = Texture::blank(1024, 1024);

    let main_render_pass = RenderPass::new(
        &vertex_shader,
        &fragment_shader,
        &vertex_bindings,
        &[&matrix_buffer, &texture_switch_buffer, &light_buffer],
        &[&cube_texture, &floor_texture],
        &[&render_texture],
    )?;

    let cube_vertices = initialize_cube_vertices()?;
    let cube_vertex_bindings = [
        VertexBinding::new(
            0,
            VertexAttribute::new(renderer::VertexAttributeFormat::RG32F, 0),
        ),
        VertexBinding::new(
            1,
            VertexAttribute::new(renderer::VertexAttributeFormat::RG32F, 0),
        ),
    ];
    let cube_vertex_shader =
        Shader::from_source(&res.load_string("/shaders/cube.vert")?, ShaderKind::Vertex)
            .context("Failed to initialize cube vertex shader")?;
    let cube_fragment_shader = Shader::from_source(
        &res.load_string("/shaders/cube.frag")?,
        ShaderKind::Fragment,
    )
        .context("Failed to initialize cube fragment shader")?;
    let cube_render_pass = RenderPass::new(
        &cube_vertex_shader,
        &cube_fragment_shader,
        &cube_vertex_bindings,
        &[&kernel_buffer],
        &[&render_texture],
        &[],
    )?;

    unsafe {
        gl::sys::Enable(gl::sys::DEPTH_TEST);
        gl::sys::DepthFunc(gl::sys::LESS);
    }


    unsafe {
        gl::sys::Enable(gl::sys::CULL_FACE);
        gl::sys::FrontFace(gl::sys::CCW);
        gl::sys::CullFace(gl::sys::BACK);

        gl::sys::Enable(gl::sys::DEPTH_TEST);
        gl::sys::DepthFunc(gl::sys::LEQUAL);
    }

    let matrices = [
        KernelMatrix {
            label: String::from("Identity"),
            matrix: nalgebra_glm::mat3(0f32, 0f32, 0f32, 0f32, 1f32, 0f32, 0f32, 0f32, 0f32),
        },
        KernelMatrix {
            label: String::from("Sobel Filter"),
            matrix: nalgebra_glm::mat3(
                -1f32, -1f32, -1f32, -1f32, 8f32, -1f32, -1f32, -1f32, -1f32,
            ),
        },
        KernelMatrix {
            label: String::from("Sharpen"),
            matrix: nalgebra_glm::mat3(0f32, -1f32, 0f32, -1f32, 5f32, -1f32, 0f32, -1f32, 0f32),
        },
        KernelMatrix {
            label: String::from("Box Blur"),
            matrix: nalgebra_glm::mat3(
                1f32 / 9f32,
                1f32 / 9f32,
                1f32 / 9f32,
                1f32 / 9f32,
                1f32 / 9f32,
                1f32 / 9f32,
                1f32 / 9f32,
                1f32 / 9f32,
                1f32 / 9f32,
            ),
        },
        KernelMatrix {
            label: String::from("Gaussian Blur"),
            matrix: nalgebra_glm::mat3(
                1f32 / 16f32,
                2f32 / 16f32,
                1f32 / 16f32,
                2f32 / 16f32,
                4f32 / 16f32,
                2f32 / 16f32,
                1f32 / 16f32,
                2f32 / 16f32,
                1f32 / 16f32,
            ),
        },
    ];

    let state = State::new(
        Vec::from(matrices),
        matrix_buffer,
        kernel_buffer,
        texture_switch_buffer,
        light_buffer,
        vertex_buffer,
        index_buffer,
        cube_vertices,
        render_texture,
        main_render_pass,
        cube_render_pass,
    );

    application::main_loop(context, state)
}

/// # Errors
/// - Fail to initialize vertex buffer
fn initialize_cube_vertices() -> Result<Buffer> {
    let vertices = vec![
        -1f32, 1f32, -1f32, -1f32, 1f32, -1f32, 1f32, -1f32, 1f32, 1f32, -1f32, 1f32, 0f32, 1f32,
        0f32, 0f32, 1f32, 0f32, 1f32, 0f32, 1f32, 1f32, 0f32, 1f32,
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

/// # Errors
/// - Fail to initialize vertex buffer
fn initialize_vertices() -> Result<Buffer> {
    let vertices = vec![
        // Vertices
        // Front face
        -0.5f32, 0.5f32, 0.5f32, -0.5f32, -0.5f32, 0.5f32, 0.5f32, -0.5f32, 0.5f32, 0.5f32, 0.5f32,
        0.5f32, // Right face
        0.5f32, 0.5f32, 0.5f32, 0.5f32, -0.5f32, 0.5f32, 0.5f32, -0.5f32, -0.5f32, 0.5f32, 0.5f32,
        -0.5f32, // Back face
        0.5f32, 0.5f32, -0.5f32, 0.5f32, -0.5f32, -0.5f32, -0.5f32, -0.5f32, -0.5f32, -0.5f32,
        0.5f32, -0.5f32, // Left face
        -0.5f32, 0.5f32, -0.5f32, -0.5f32, -0.5f32, -0.5f32, -0.5f32, -0.5f32, 0.5f32, -0.5f32,
        0.5f32, 0.5f32, // Top face
        -0.5f32, 0.5f32, -0.5f32, -0.5f32, 0.5f32, 0.5f32, 0.5f32, 0.5f32, 0.5f32, 0.5f32, 0.5f32,
        -0.5f32, // Bottom face
        -0.5f32, -0.5f32, 0.5f32, -0.5f32, -0.5f32, -0.5f32, 0.5f32, -0.5f32, -0.5f32, 0.5f32,
        -0.5f32, 0.5f32, // Texture coordinates
        // Front face
        0.0f32, 1.0f32, 0.0f32, 0.0f32, 1.0f32, 0.0f32, 1.0f32, 1.0f32, // Right face
        0.0f32, 1.0f32, 0.0f32, 0.0f32, 1.0f32, 0.0f32, 1.0f32, 1.0f32, // Back face
        0.0f32, 1.0f32, 0.0f32, 0.0f32, 1.0f32, 0.0f32, 1.0f32, 1.0f32, // Left face
        0.0f32, 1.0f32, 0.0f32, 0.0f32, 1.0f32, 0.0f32, 1.0f32, 1.0f32, // Top face
        0.0f32, 1.0f32, 0.0f32, 0.0f32, 1.0f32, 0.0f32, 1.0f32, 1.0f32, // Bottom face
        0.0f32, 1.0f32, 0.0f32, 0.0f32, 1.0f32, 0.0f32, 1.0f32, 1.0f32,
        // Normals
        // Front face
        0f32, 0f32, 1f32, 0f32, 0f32, 1f32, 0f32, 0f32, 1f32, 0f32, 0f32, 1f32,
        // Right face
        1f32, 0f32, 0f32, 1f32, 0f32, 0f32, 1f32, 0f32, 0f32, 1f32, 0f32, 0f32,
        // Back face
        0f32, 0f32, -1f32, 0f32, 0f32, -1f32, 0f32, 0f32, -1f32, 0f32, 0f32, -1f32,
        // Left face
        -1f32, 0f32, 0f32, -1f32, 0f32, 0f32, -1f32, 0f32, 0f32, -1f32, 0f32, 0f32,
        // Top face
        0f32, 1f32, 0f32, 0f32, 1f32, 0f32, 0f32, 1f32, 0f32, 0f32, 1f32, 0f32,
        // Bottom face
        0f32, -1f32, 0f32, 0f32, -1f32, 0f32, 0f32, -1f32, 0f32, 0f32, -1f32, 0f32,
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

/// # Errors
/// - Fail to initialize index buffer
pub fn initialize_indices() -> Result<Buffer> {
    let indices = vec![
        // Front face
        0u16, 1u16, 2u16, 0u16, 2u16, 3u16, // Right face
        4u16, 5u16, 6u16, 4u16, 6u16, 7u16, // Back face
        8u16, 9u16, 10u16, 8u16, 10u16, 11u16, // Left face
        12u16, 13u16, 14u16, 12u16, 14u16, 15u16, // Top face
        16u16, 17u16, 18u16, 16u16, 18u16, 19u16, // Bottom face
        20u16, 21u16, 22u16, 20u16, 22u16, 23u16,
    ];
    let mut index_buffer = Buffer::allocate(
        BufferUsage::Index,
        std::mem::size_of::<u16>() * indices.len(),
    )?;
    let ptr = index_buffer.map::<u16>();
    ptr.copy_from_slice(&indices);
    index_buffer.unmap();
    Ok(index_buffer)
}
