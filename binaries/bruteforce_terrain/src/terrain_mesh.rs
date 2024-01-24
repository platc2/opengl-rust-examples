use anyhow::Context;

use renderer::{Buffer, BufferUsage};

pub struct TerrainMesh {
    width: usize,
    height: usize,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
}

impl TerrainMesh {
    pub fn of_size(width: usize, height: usize) -> anyhow::Result<Self> {
        let vertex_data = terrain_vertices(width, height);
        let mut vertex_buffer = Buffer::allocate(
            BufferUsage::Vertex,
            vertex_data.len() * std::mem::size_of::<f32>(),
        )
            .context("Failed to initialize terrain mesh vertex buffer")?;
        upload_data(&mut vertex_buffer, &vertex_data[..]);

        let index_data = terrain_indices(width, height);
        let mut index_buffer = Buffer::allocate(
            BufferUsage::Index,
            index_data.len() * std::mem::size_of::<u16>(),
        )
            .context("Failed to initialize terrain mesh index buffer")?;
        upload_data(&mut index_buffer, &index_data[..]);

        Ok(Self {
            width,
            height,
            vertex_buffer,
            index_buffer,
        })
    }

    pub const fn width(&self) -> usize {
        self.width
    }
    pub const fn height(&self) -> usize {
        self.height
    }

    pub const fn vertex_buffer_handle(&self) -> gl::types::GLuint {
        self.vertex_buffer.handle()
    }

    pub const fn index_buffer_handle(&self) -> gl::types::GLuint {
        self.index_buffer.handle()
    }
}

fn upload_data<T: Copy>(buffer: &mut Buffer, data: &[T]) {
    let ptr = buffer.map();
    ptr.copy_from_slice(data);
    buffer.unmap();
}

/**
 * Generates vertices on the XZ-plane in the coordinate range [-0.5, 0.5]
 */
fn terrain_vertices(width: usize, height: usize) -> Vec<f32> {
    let num_vertices = (width + 1) * (height + 1) * 3;
    let mut vertices = Vec::with_capacity(num_vertices);

    for y in 0..=height {
        let pos_y = 0f32;
        let pos_z = (y as f32 / height as f32) as f32 - 0.5f32;
        for x in 0..=width {
            let pos_x = (x as f32 / width as f32) - 0.5f32;
            vertices.push(pos_x);
            vertices.push(pos_y);
            vertices.push(pos_z);
        }
    }

    vertices
}

fn terrain_indices(width: usize, height: usize) -> Vec<u16> {
    let num_indices = 2 * (width + 2) * height;
    let mut indices = Vec::with_capacity(num_indices);

    for y in 0..height {
        for x in 0..=width {
            let row_index = (y * (width + 1) + x) as u16;
            let next_row_index = ((y + 1) * (width + 1) + x) as u16;
            if x == 0 {
                indices.push(row_index);
            }
            indices.push(row_index);
            indices.push(next_row_index);
            if x == width {
                indices.push(next_row_index);
            }
        }
    }

    indices
}
