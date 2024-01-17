use anyhow::Context;
use anyhow::Result;

use hello_triangle_rust::renderer::{Buffer, BufferUsage};

use crate::polyhedron::Polyhedron;

pub struct Planet {
    pub vertex_buffer: Buffer,
    pub size: u32,
    icosahedron: Polyhedron,
}

impl Planet {
    pub fn new() -> Result<Planet> {
        let icosahedron: Polyhedron = Polyhedron::regular_icosahedron();
        let vertex_data = generate_mesh_data(0, &nalgebra_glm::vec3(0., 0., 0.), None, &icosahedron);
        let mut vertex_buffer = Buffer::allocate(
            BufferUsage::Vertex,
            vertex_data.len() * std::mem::size_of::<f32>(),
        )
            .context("Failed to initialize terrain mesh vertex buffer")?;
        upload_data(&mut vertex_buffer, &vertex_data[..]);

        Ok(Self { vertex_buffer, size: vertex_data.len() as u32, icosahedron })
    }

    pub fn recalculate(&mut self, max_level: u16, camera: &nalgebra_glm::Vec3, forward: Option<&nalgebra_glm::Vec3>) {
        let vertex_data = generate_mesh_data(max_level, camera, forward, &self.icosahedron);
        let mut vertex_buffer = Buffer::allocate(
            BufferUsage::Vertex,
            vertex_data.len() * std::mem::size_of::<f32>(),
        ).unwrap();
        upload_data(&mut vertex_buffer, &vertex_data[..]);

        self.vertex_buffer = vertex_buffer;
        self.size = vertex_data.len() as u32;
    }
}

fn upload_data<T: Copy>(buffer: &mut Buffer, data: &[T]) {
    let ptr = buffer.map();
    ptr.copy_from_slice(data);
    buffer.unmap();
}

fn generate_mesh_data(max_level: u16, camera: &nalgebra_glm::Vec3, forward: Option<&nalgebra_glm::Vec3>, icosahedron: &Polyhedron) -> Vec<f32> {
    let mut result = Vec::new();
    /*
        let triangles = icosahedron::triangles();

        triangles.chunks_exact(3)
            .map(|x| [&x[0], &x[1], &x[2]])
            .for_each(|[a, b, c]| recursive_triangle(&mut result, &camera, a, b, c, 0, max_level));
        result
    */

    icosahedron.triangles
        .iter()
/*
        .filter(|(a, b, c)| {
            if let Some(forward) = forward {
                let x = nalgebra_glm::vec3(b.x - a.x, b.y - a.y, b.z - a.z).normalize();
                let y = nalgebra_glm::vec3(c.x - a.x, c.y - a.y, c.z - a.z).normalize();
                let n = nalgebra_glm::cross(&x, &y).normalize();
                nalgebra_glm::dot(forward, &n) >= 0.
            } else {
                true
            }
        })
 */
        .for_each(|(a, b, c)| recursive_triangle(&mut result, &camera, a, b, c, 0, max_level));
    result
}

fn recursive_triangle(vertices: &mut Vec<f32>, camera: &nalgebra_glm::Vec3, a: &nalgebra_glm::Vec3, b: &nalgebra_glm::Vec3, c: &nalgebra_glm::Vec3, level: u16, max_level: u16) {
    let subdivide_level = level < max_level;
    // Already in world coordinates
    let center: Vec3 = (a + b + c) / 3.;
    let distance_vector: Vec3 = center - camera;
    let subdivide = subdivide_level && (nalgebra_glm::length(&distance_vector) * f32::from(level) < 2.);

    use nalgebra_glm::Vec3;

    if (subdivide) {
        let A: Vec3 = (b + ((c - b) * 0.5))
            .normalize();
        let B: Vec3 = (c + ((a - c) * 0.5))
            .normalize();
        let C: Vec3 = (a + ((b - a) * 0.5))
            .normalize();

        let level = level + 1;
        recursive_triangle(vertices, camera, a, &B, &C, level, max_level);
        recursive_triangle(vertices, camera, &A, b, &C, level, max_level);
        recursive_triangle(vertices, camera, &A, &B, c, level, max_level);
        recursive_triangle(vertices, camera, &A, &B, &C, level, max_level);
    } else {
        vertices.push(a.x);
        vertices.push(a.y);
        vertices.push(a.z);

        vertices.push(b.x);
        vertices.push(b.y);
        vertices.push(b.z);

        vertices.push(c.x);
        vertices.push(c.y);
        vertices.push(c.z);
    }
}
