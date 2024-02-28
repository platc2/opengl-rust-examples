pub use buffer_id::*;
pub use buffer_target::*;
pub use buffer_usage::*;
pub use storage_flags::*;

use crate::{gl, gl::RawHandle};
use crate::vertex_array::VertexArrayId;

mod buffer_target;
mod buffer_usage;
mod buffer_id;
mod storage_flags;

pub fn bind_buffer(target: BufferTarget, buffer: BufferId) {
    unsafe { gl::BindBuffer(target.raw_handle(), buffer.raw_handle()); }
}

pub fn bind_buffer_base(target: BufferTarget, index: usize, buffer: BufferId) {
    unsafe { gl::BindBufferBase(target.raw_handle(), index as _, buffer.raw_handle()); }
}

pub fn bind_buffer_range(target: BufferTarget, index: usize, buffer: BufferId, offset: usize, size: usize) {
    unsafe { gl::BindBufferRange(target.raw_handle(), index as _, buffer.raw_handle(), offset as _, size as _); }
}

#[cfg(feature = "GL44")]
pub fn bind_buffers_base(target: BufferTarget, first: usize, buffers: &[BufferId]) {
    unsafe { gl::BindBuffersBase(target.raw_handle(), first as _, buffers.len() as _, buffers.as_ptr().cast()); }
}

#[cfg(feature = "GL44")]
pub fn bind_buffers_range(
    target: BufferTarget,
    first: usize,
    buffers: &[BufferId],
    offsets: &[usize],
    sizes: &[usize]) {
    unsafe {
        gl::BindBuffersRange(
            target.raw_handle(),
            first as _,
            buffers.len() as _,
            buffers.as_ptr().cast(),
            offsets.as_ptr().cast(),
            sizes.as_ptr().cast());
    }
}

#[cfg(feature = "GL43")]
pub fn bind_vertex_buffer(binding: usize, buffer: BufferId, offset: usize, stride: usize) {
    unsafe { gl::BindVertexBuffer(binding as _, buffer.raw_handle(), offset as _, stride as _); }
}

#[cfg(feature = "GL45")]
pub fn vertex_array_vertex_buffer(vertex_array: VertexArrayId, binding: usize, buffer: BufferId, offset: usize, stride: usize) {
    unsafe { gl::VertexArrayVertexBuffer(vertex_array.raw_handle(), binding as _, buffer.raw_handle(), offset as _, stride as _); }
}

#[cfg(feature = "GL44")]
pub fn bind_vertex_buffers(first: usize, buffers: &[BufferId], offsets: &[usize], strides: &[usize]) {
    unsafe { gl::BindVertexBuffers(first as _, buffers.len() as _, buffers.as_ptr().cast(), offsets.as_ptr().cast(), strides.as_ptr().cast()); }
}

#[cfg(feature = "GL45")]
pub fn vertex_array_vertex_buffers(vertex_array: VertexArrayId, first: usize, buffers: &[BufferId], offsets: &[usize], strides: &[usize]) {
    unsafe { gl::VertexArrayVertexBuffers(vertex_array.raw_handle(), first as _, buffers.len() as _, buffers.as_ptr().cast(), offsets.as_ptr().cast(), strides.as_ptr().cast()); }
}

pub fn buffer_data<T>(target: BufferTarget, data: &[T], usage: BufferUsage) {
    unsafe { gl::BufferData(target.raw_handle(), core::mem::size_of_val(data) as _, data.as_ptr().cast(), usage.raw_handle()); }
}

#[cfg(feature = "GL45")]
pub fn named_buffer_data<T>(buffer: BufferId, data: &[T], usage: BufferUsage) {
    unsafe { gl::NamedBufferData(buffer.raw_handle(), core::mem::size_of_val(data) as _, data.as_ptr().cast(), usage.raw_handle()); }
}

#[cfg(feature = "GL44")]
pub fn buffer_storage_empty(target: BufferTarget, size: usize, flags: StorageFlags) {
    unsafe { gl::BufferStorage(target.raw_handle(), size as _, core::ptr::null(), flags.raw_handle()); }
}

#[cfg(feature = "GL44")]
pub fn buffer_storage<T>(target: BufferTarget, data: &[T], flags: StorageFlags) {
    unsafe { gl::BufferStorage(target.raw_handle(), core::mem::size_of_val(data) as _, data.as_ptr().cast(), flags.raw_handle()); }
}

#[cfg(feature = "GL45")]
pub fn named_buffer_storage<T>(buffer: BufferId, data: &[T], flags: StorageFlags) {
    unsafe { gl::NamedBufferStorage(buffer.raw_handle(), core::mem::size_of_val(data) as _, data.as_ptr().cast(), flags.raw_handle()); }
}

pub fn buffer_sub_data<T>(target: BufferTarget, offset: usize, data: &[T]) {
    unsafe { gl::BufferSubData(target.raw_handle(), offset as _, core::mem::size_of_val(data) as _, data.as_ptr().cast()); }
}

#[cfg(feature = "GL45")]
pub fn named_buffer_sub_data<T>(buffer: BufferId, offset: usize, data: &[T]) {
    unsafe { gl::NamedBufferSubData(buffer.raw_handle(), offset as _, core::mem::size_of_val(data) as _, data.as_ptr().cast()); }
}

/*
#[cfg(feature = "GL43")]
pub fn clear_buffer_data(target: BufferTarget, ...)
*/

/*
#[cfg(feature = "GL45")]
pub fn clear_named_buffer_data(buffer: BufferId, ...)
*/

/*
#[cfg(feature = "GL43")]
pub fn clear_buffer_sub_data(target: BufferTarget, ...)
*/

/*
#[cfg(feature = "GL45")]
pub fn clear_named_buffer_sub_data(buffer: BufferId, ...)
*/

pub fn copy_buffer_sub_data(read_target: BufferTarget, write_target: BufferTarget, read_offset: usize, write_offset: usize, size: usize) {
    unsafe { gl::CopyBufferSubData(read_target.raw_handle(), write_target.raw_handle(), read_offset as _, write_offset as _, size as _); }
}

#[cfg(feature = "GL45")]
pub fn copy_named_buffer_sub_data(read_buffer: BufferId, write_buffer: BufferId, read_offset: usize, write_offset: usize, size: usize) {
    unsafe { gl::CopyNamedBufferSubData(read_buffer.raw_handle(), write_buffer.raw_handle(), read_offset as _, write_offset as _, size as _); }
}

#[must_use]
#[cfg(feature = "GL45")]
pub fn create_buffers(count: usize) -> Vec<BufferId> {
    let mut raw_buffer_ids = Vec::with_capacity(count);

    unsafe {
        let raw_buffer_ids_ptr = raw_buffer_ids.spare_capacity_mut().as_mut_ptr().cast();
        gl::CreateBuffers(count as _, raw_buffer_ids_ptr);
        raw_buffer_ids.set_len(count);
    }

    raw_buffer_ids
}

#[must_use]
#[cfg(feature = "GL45")]
pub fn create_buffer() -> BufferId {
    let mut buffer_id: gl::GLuint = 0;
    unsafe { gl::CreateBuffers(1, &mut buffer_id); }
    BufferId(buffer_id)
}

pub fn delete_buffers(buffers: &mut [BufferId]) {
    unsafe { gl::DeleteBuffers(buffers.len() as _, buffers.as_ptr().cast()); }
    for buffer in buffers {
        *buffer = BufferId::NO_BUFFER;
    }
}

pub fn delete_buffer(buffer: &mut BufferId) {
    delete_buffers(core::slice::from_mut(buffer));
}
