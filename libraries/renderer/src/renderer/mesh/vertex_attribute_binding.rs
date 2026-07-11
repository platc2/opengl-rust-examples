use super::VertexAttribute;
use gl::sys::types::GLuint;

pub struct VertexAttributeBinding {
    pub(super) vertex_attribute: VertexAttribute,
    pub(super) binding_index: GLuint,
}

impl VertexAttributeBinding {
    #[must_use]
    pub fn new(binding_index: u8, vertex_attribute: VertexAttribute) -> Self {
        Self {
            binding_index: GLuint::from(binding_index),
            vertex_attribute,
        }
    }
}
