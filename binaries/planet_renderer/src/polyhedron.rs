use std::rc::Rc;

use nalgebra_glm as glm;

pub type Vertex = Rc<glm::Vec3>;

#[derive(Debug)]
pub struct Polyhedron {
    pub vertices: Vec<Vertex>,
    pub triangles: Vec<(Vertex, Vertex, Vertex)>,
}

impl Polyhedron {
    pub fn regular_icosahedron() -> Self {
        let vertices: [Vertex; 12] = generate_icosahedron_vertices();
        let triangles: Vec<(Vertex, Vertex, Vertex)> = generate_icosahedron_triangles(&vertices);

        Self { vertices: Vec::from(vertices), triangles }
    }
}

fn generate_icosahedron_vertices() -> [Vertex; 12] {
    let golden_ratio: f32 = (1. + 5f32.sqrt()) / 2.;
    // Coordinates consist of 0.0, 1.0 and golden_ratio. Thus their length can be scaled
    // down accordingly to be unit vectors
    let scale: f32 = 1. / (golden_ratio * golden_ratio + 1.).sqrt();
    let coordinate_a: f32 = golden_ratio * scale;
    let coordinate_b: f32 = scale;
    [
        // x plane
        glm::vec3(-coordinate_a, 0., coordinate_b),
        glm::vec3(coordinate_a, 0., coordinate_b),
        glm::vec3(coordinate_a, 0., -coordinate_b),
        glm::vec3(-coordinate_a, 0., -coordinate_b),

        // y plane
        glm::vec3(-coordinate_b, coordinate_a, 0.),
        glm::vec3(coordinate_b, coordinate_a, 0.),
        glm::vec3(coordinate_b, -coordinate_a, 0.),
        glm::vec3(-coordinate_b, -coordinate_a, 0.),

        // z plane
        glm::vec3(0., coordinate_b, coordinate_a),
        glm::vec3(0., coordinate_b, -coordinate_a),
        glm::vec3(0., -coordinate_b, -coordinate_a),
        glm::vec3(0., -coordinate_b, coordinate_a),
    ]
        .map(|vector| Rc::new(vector))
}

fn generate_icosahedron_triangles(vertices: &[Vertex; 12]) -> Vec<(Vertex, Vertex, Vertex)> {
    // Rings
    //   First: 3, 4, 8, 11, 7
    //   Second: 10, 9, 5, 1, 6
    let faces = vec![
        &vertices[0], &vertices[3], &vertices[4],
        &vertices[0], &vertices[7], &vertices[3],
        &vertices[0], &vertices[11], &vertices[7],
        &vertices[0], &vertices[8], &vertices[11],
        &vertices[0], &vertices[4], &vertices[8],
        &vertices[2], &vertices[1], &vertices[5],
        &vertices[2], &vertices[6], &vertices[1],
        &vertices[2], &vertices[10], &vertices[6],
        &vertices[2], &vertices[9], &vertices[10],
        &vertices[2], &vertices[5], &vertices[9],
        &vertices[10], &vertices[3], &vertices[7],
        &vertices[9], &vertices[4], &vertices[3],
        &vertices[5], &vertices[8], &vertices[4],
        &vertices[1], &vertices[11], &vertices[8],
        &vertices[6], &vertices[7], &vertices[11],
        &vertices[9], &vertices[3], &vertices[10],
        &vertices[5], &vertices[4], &vertices[9],
        &vertices[1], &vertices[8], &vertices[5],
        &vertices[6], &vertices[11], &vertices[1],
        &vertices[10], &vertices[7], &vertices[6],
    ];

    faces
        .chunks_exact(3)
        .map(|chunk| (chunk[0].clone(), chunk[1].clone(), chunk[2].clone()))
        .collect()
}
