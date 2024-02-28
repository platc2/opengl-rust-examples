use gl_bindings_raw_handle_derive::RawHandle;

use crate::{gl, gl::RawHandle};

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, RawHandle)]
pub struct VertexArrayId(pub(crate) gl::GLuint);

impl VertexArrayId {
    pub const NO_VERTEX_ARRAY: VertexArrayId = VertexArrayId(0);
}

#[must_use]
#[cfg(feature = "GL45")]
pub fn create_vertex_arrays(count: usize) -> Vec<VertexArrayId> {
    let mut vertex_arrayids = Vec::with_capacity(count);

    unsafe {
        let ptr = vertex_arrayids.spare_capacity_mut().as_mut_ptr().cast();
        gl::CreateVertexArrays(count as _, ptr);
        vertex_arrayids.set_len(count);
    }

    vertex_arrayids
}

#[must_use]
#[cfg(feature = "GL45")]
pub fn create_vertex_array() -> VertexArrayId {
    let mut vertex_array_id: gl::GLuint = 0;
    unsafe { gl::CreateVertexArrays(1, &mut vertex_array_id); }
    VertexArrayId(vertex_array_id)
}

pub fn enable_vertex_attrib_array(index: usize) {
    unsafe { gl::EnableVertexAttribArray(index as _); }
}

pub fn disable_vertex_attrib_array(index: usize) {
    unsafe { gl::DisableVertexAttribArray(index as _); }
}

#[cfg(feature = "GL45")]
pub fn enable_vertex_array_attrib(vertex_array: VertexArrayId, index: usize) {
    unsafe { gl::EnableVertexArrayAttrib(vertex_array.raw_handle(), index as _); }
}

#[cfg(feature = "GL45")]
pub fn disable_vertex_array_attrib(vertex_array: VertexArrayId, index: usize) {
    unsafe { gl::DisableVertexArrayAttrib(vertex_array.raw_handle(), index as _); }
}





pub fn delete_vertex_array(vertex_array_id: &mut VertexArrayId) {
    let id = unsafe { vertex_array_id.raw_handle() };
    unsafe { gl::DeleteVertexArrays(1, &id); }
    vertex_array_id.0 = VertexArrayId::NO_VERTEX_ARRAY.0;
}

pub fn bind_vertex_array(vertex_array_id: VertexArrayId) {
    unsafe { gl::BindVertexArray(vertex_array_id.raw_handle()); }
}
