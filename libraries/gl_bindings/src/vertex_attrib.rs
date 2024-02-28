use gl_bindings_raw_handle_derive::RawHandle;

use crate::{define_gl_constants, gl, gl::RawHandle};

#[derive(Debug, Copy, Clone, PartialEq, Eq, RawHandle)]
pub struct ComponentSize(gl::GLint);

impl ComponentSize {
    pub const SIZE_1: ComponentSize = ComponentSize(1);
    pub const SIZE_2: ComponentSize = ComponentSize(2);
    pub const SIZE_3: ComponentSize = ComponentSize(3);
    pub const SIZE_4: ComponentSize = ComponentSize(4);
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, RawHandle)]
pub struct ComponentType(gl::GLenum);

impl ComponentType {
    define_gl_constants!(ComponentType ::
        UNSIGNED_BYTE,
        BYTE,
        SHORT,
        FLOAT
    );
}

pub fn vertex_attrib_pointer(index: usize, size: ComponentSize, value_type: ComponentType, normalized: bool, stride: usize, offset: usize) {
    unsafe {
        gl::VertexAttribPointer(
            index as _,
            size.raw_handle(),
            value_type.raw_handle(),
            normalized as _,
            stride as _,
            offset as _);
    }
}
