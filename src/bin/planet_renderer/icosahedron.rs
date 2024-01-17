use nalgebra_glm as glm;

pub fn vertices() -> Vec<glm::Vec3> {
    let golden_ratio: f32 = (1. + 5f32.sqrt()) / 2.;
    // Endpoints will have 3 coordinates, GOLDEN_RATIO, 1, and 0. Thus the vertices
    // must be normalised using a SCALE
    let scale: f32 = 1. / (golden_ratio * golden_ratio + 1.).sqrt();

    let vertex_a: f32 = golden_ratio * scale;
    let vertex_b: f32 = scale;

    use glm::vec3;
    vec![
        // x plane
        vec3(-vertex_a, 0., vertex_b),
        vec3(vertex_a, 0., vertex_b),
        vec3(vertex_a, 0., -vertex_b),
        vec3(-vertex_a, 0., -vertex_b),

        // y plane
        vec3(-vertex_b, vertex_a, 0.),
        vec3(vertex_b, vertex_a, 0.),
        vec3(vertex_b, -vertex_a, 0.),
        vec3(-vertex_b, -vertex_a, 0.),

        // z plane
        vec3(0., vertex_b, vertex_a),
        vec3(0., vertex_b, -vertex_a),
        vec3(0., -vertex_b, -vertex_a),
        vec3(0., -vertex_b, vertex_a),
    ]
}

pub fn triangles() -> Vec<glm::Vec3> {
    let vertices = vertices();
    vec![
        vertices[0], vertices[3], vertices[4],
        vertices[0], vertices[7], vertices[3],
        vertices[0], vertices[11], vertices[7],
        vertices[0], vertices[8], vertices[11],
        vertices[0], vertices[4], vertices[8],

        vertices[2], vertices[1], vertices[5],
        vertices[2], vertices[6], vertices[1],
        vertices[2], vertices[10], vertices[6],
        vertices[2], vertices[9], vertices[10],
        vertices[2], vertices[5], vertices[9],

        // First: 3, 4, 8, 11, 7
        // Second: 10, 9, 5, 1, 6
        vertices[10], vertices[3], vertices[7],
        vertices[9], vertices[4], vertices[3],
        vertices[5], vertices[8], vertices[4],
        vertices[1], vertices[11], vertices[8],
        vertices[6], vertices[7], vertices[11],

        vertices[9], vertices[3], vertices[10],
        vertices[5], vertices[4], vertices[9],
        vertices[1], vertices[8], vertices[5],
        vertices[6], vertices[11], vertices[1],
        vertices[10], vertices[7], vertices[6],
    ]
}
