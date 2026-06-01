#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::must_use_candidate)]

extern crate alloc;
extern crate core;
extern crate gl_bindings as gl;
extern crate nalgebra_glm as glm;

use anyhow::{Context, Result};

use renderer::renderer_context::{OpenGLVersion, RendererContext, WindowDimension};
use renderer::{
    application, Buffer, BufferUsage
    ,
};

use crate::sobel_cube::SobelCube;

mod sobel_cube;

type Mat3 = nalgebra_glm::TMat3<f32>;
type Vec3 = nalgebra_glm::TVec3<f32>;

pub struct KernelMatrix {
    pub label: String,
    pub matrix: Mat3,
}

fn main() -> Result<()> {
    // Initialize render-context
    let context = RendererContext::init(
        "Sobel Cube",
        &WindowDimension::default(),
        &OpenGLVersion::default(),
    )?;

    application::start::<SobelCube>(context)
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
