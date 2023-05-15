type Mat4 = nalgebra_glm::TMat4<f32>;

#[derive(Default, Copy, Clone)]
pub struct MatrixUniform {
    pub model: Mat4,
    pub view: Mat4,
    pub projection: Mat4,
}
