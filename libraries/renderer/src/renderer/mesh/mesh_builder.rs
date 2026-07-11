use super::{BufferBinding, Format, Mesh, VertexAttribute, VertexAttributeBinding};
use crate::Buffer;
use std::rc::Rc;

pub struct MeshBuilder {
    attribute_bindings: Vec<VertexAttributeBinding>,
    buffer_bindings: Vec<BufferBinding>,
}

impl Default for MeshBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl MeshBuilder {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            attribute_bindings: vec![],
            buffer_bindings: vec![],
        }
    }

    pub fn vertex_buffer(&mut self, buffer: Rc<Buffer>, offset: isize, stride: i32) -> &mut Self {
        self.buffer_bindings
            .push(BufferBinding::new(buffer, offset, stride));
        self
    }

    pub fn attribute(&mut self, binding: u8, format: Format, offset: u16) -> &mut Self {
        self.attribute_bindings.push(VertexAttributeBinding::new(
            binding,
            VertexAttribute::new(format, offset),
        ));
        self
    }

    #[must_use]
    pub fn build(&self) -> Mesh {
        Mesh::new(&self.attribute_bindings, &self.buffer_bindings)
    }
}
