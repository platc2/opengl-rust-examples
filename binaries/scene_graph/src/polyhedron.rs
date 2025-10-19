use std::rc::Rc;

pub type Vertex = Rc<glm::Vec3>;
pub type Triangle = (Vertex, Vertex, Vertex);

#[derive(Debug)]
pub struct Polyhedron {
    vertices: Vec<Vertex>,
    triangles: Vec<Triangle>,
}

impl Polyhedron {
    #[must_use]
    pub fn cube() -> Self {
        let vertices = generate_cube_vertices();
        let triangles = generate_cube_triangles(&vertices);

        Self { vertices, triangles }
    }

    #[must_use]
    pub fn vertices(&self) -> &Vec<Vertex> { &self.vertices }

    #[must_use]
    pub fn triangles(&self) -> &Vec<Triangle> { &self.triangles }
}

fn generate_cube_vertices() -> Vec<Vertex> {
    let unit: f32 = 0.5;
    vec![
        // z == -unit
        glm::vec3(-unit, unit, -unit),
        glm::vec3(-unit, -unit, -unit),
        glm::vec3(unit, -unit, -unit),
        glm::vec3(unit, unit, -unit),

        // z == unit
        glm::vec3(unit, unit, unit),
        glm::vec3(unit, -unit, unit),
        glm::vec3(-unit, -unit, unit),
        glm::vec3(-unit, unit, unit),
    ]
        .into_iter()
        .map(Rc::new)
        .collect()
}

fn generate_cube_triangles(vertices: &Vec<Vertex>) -> Vec<Triangle> {
    let indices = vec![
        // Front face
        (0, 1, 2),
        (2, 3, 0),

        // Right face
        (3, 2, 5),
        (5, 4, 3),

        // Back face
        (4, 5, 6),
        (6, 7, 4),

        // Left face
        (7, 6, 1),
        (1, 0, 7),

        // Top face
        (7, 0, 3),
        (3, 4, 7),

        // Bottom face
        (1, 6, 5),
        (5, 2, 1),
    ];

    indices
        .into_iter()
        .map(|(a, b, c)| (vertices[a].clone(), vertices[b].clone(), vertices[c].clone()))
        .collect()
}
