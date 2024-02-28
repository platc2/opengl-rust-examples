pub use clear_mask::*;
pub use draw_mode::*;
use gl_bindings_raw_handle_derive::RawHandle;

use crate::{define_gl_constants, gl, gl::RawHandle};

mod clear_mask;
mod draw_mode;

pub fn clear(clear_mask: ClearMask) {
    unsafe { gl::Clear(clear_mask.raw_handle()); }
}

pub fn clear_color(color_rgb: u32) {
    let a = ((color_rgb >> 24) & 0xFF) as f32 / 255.;
    let r = ((color_rgb >> 16) & 0xFF) as f32 / 255.;
    let g = ((color_rgb >> 8) & 0xFF) as f32 / 255.;
    let b = (color_rgb & 0xFF) as f32 / 255.;
    unsafe { gl::ClearColor(r, g, b, a); }
}

pub fn clear_depth(depth: f64) {
    unsafe { gl::ClearDepth(depth as _); }
}

#[cfg(feature = "GL41")]
pub fn clear_depth_f(depth: f32) {
    unsafe { gl::ClearDepth(depth as _); }
}

pub fn clear_stencil(value: i32) {
    unsafe { gl::ClearStencil(value as _); }
}

pub fn finish() {
    unsafe { gl::Finish(); }
}

pub fn flush() {
    unsafe { gl::Flush(); }
}

pub fn draw_arrays(draw_mode: DrawMode, start_index: usize, count: usize) {
    unsafe { gl::DrawArrays(draw_mode.raw_handle(), start_index as _, count as _); }
}

pub fn draw_elements<T>(draw_mode: DrawMode, count: usize, index_type: IndexType, indices: Option<Vec<T>>) {
    let indices = indices.map(|vec| vec.as_ptr().cast())
        .unwrap_or(core::ptr::null());
    unsafe { gl::DrawElements(draw_mode.raw_handle(), count as _, index_type.raw_handle(), indices); }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, RawHandle)]
pub struct IndexType(gl::GLenum);

impl IndexType {
    define_gl_constants!(IndexType ::
        UNSIGNED_BYTE,
        UNSIGNED_SHORT,
        UNSIGNED_INT
    );
}

pub fn draw_elements_base_vertex(draw_mode: DrawMode, count: usize, index_type: IndexType, index_offset: usize, base_vertex: usize) {
    unsafe {
        gl::DrawElementsBaseVertex(
            draw_mode.raw_handle(),
            count as _,
            index_type.raw_handle(),
            index_offset as _,
            base_vertex as _);
    }
}
