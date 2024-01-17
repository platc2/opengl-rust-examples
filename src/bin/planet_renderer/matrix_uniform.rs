use nalgebra_glm::Mat4;

#[derive(Default, Copy, Clone)]
pub struct MatrixUniform {
    pub model: Mat4,
    pub view: Mat4,
    pub projection: Mat4,
}