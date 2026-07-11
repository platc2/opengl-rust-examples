mod buffer_binding;
mod mesh_builder;
mod vertex_attribute;
mod vertex_attribute_binding;

use crate::Buffer;
use gl::sys::types::{GLenum, GLint, GLuint};
use gl::sys::RawHandle;
use gl::vertex_array::VertexArrayId;
use std::rc::Rc;

pub use buffer_binding::BufferBinding;
pub use mesh_builder::MeshBuilder;
pub use vertex_attribute::Format;
pub use vertex_attribute::VertexAttribute;
pub use vertex_attribute_binding::VertexAttributeBinding;

pub struct Mesh {
    vertex_array: VertexArrayId,
    _buffers: Vec<Rc<Buffer>>,
}

impl Mesh {
    #[must_use]
    pub fn new(bindings: &[VertexAttributeBinding], buffer_bindings: &[BufferBinding]) -> Self {
/*
        let vertex_array = gl::vertex_array::create_vertex_array();

        for (
            index,
            VertexAttributeBinding {
                binding_index,
                vertex_attribute,
            },
        ) in bindings.iter().enumerate()
        {
            let index = index as _;
            let (format_size, format_type) = convert_format(vertex_attribute.format());
            unsafe {
                gl::sys::EnableVertexArrayAttrib(vertex_array.raw_handle(), index);
                gl::sys::VertexArrayAttribFormat(
                    vertex_array.raw_handle(),
                    index,
                    format_size,
                    format_type,
                    gl::sys::FALSE,
                    GLuint::from(vertex_attribute.offset()),
                );
                gl::sys::VertexArrayAttribBinding(vertex_array.raw_handle(), index, *binding_index);
            }
        }

        for (
            index,
            BufferBinding {
                buffer,
                offset,
                stride,
            },
        ) in buffer_bindings.iter().enumerate()
        {
            unsafe {
                gl::sys::VertexArrayVertexBuffer(
                    vertex_array.raw_handle(),
                    index as _,
                    buffer.handle(),
                    *offset,
                    *stride,
                );
            }
        }

        let buffers = buffer_bindings
            .iter()
            .map(|binding| binding.buffer.clone())
            .collect();

        Self {
            vertex_array,
            _buffers: buffers,
        }
*/
        unimplemented!()
    }

    pub fn bind(&self) {
/*
        gl::vertex_array::bind_vertex_array(self.vertex_array);
*/
    }
}

const fn convert_format(format: Format) -> (GLint, GLenum) {
    match format {
        Format::R32F => (1, gl::sys::FLOAT),
        Format::RG32F => (2, gl::sys::FLOAT),
        Format::RGB32F => (3, gl::sys::FLOAT),
        Format::RGBA32F => (4, gl::sys::FLOAT),
        Format::R8 => (1, gl::sys::BYTE),
        Format::RG8 => (2, gl::sys::BYTE),
        Format::RGB8 => (3, gl::sys::BYTE),
        Format::RGBA8 => (4, gl::sys::BYTE),
    }
}
