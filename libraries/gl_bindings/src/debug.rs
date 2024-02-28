use gl_bindings_raw_handle_derive::RawHandle;

use crate::{define_gl_constants, gl, gl::RawHandle};

#[derive(Debug, Copy, Clone, PartialEq, Eq, RawHandle)]
pub struct DebugGroupSource(gl::GLenum);

impl DebugGroupSource {
    define_gl_constants!(DebugGroupSource ::
        DEBUG_SOURCE_APPLICATION,
        DEBUG_SOURCE_THIRD_PARTY
    );
}

pub fn pop_debug_group() {}

pub fn push_debug_group(debug_group_source: DebugGroupSource, id: usize, message: &str) {
    unsafe {
        gl::PushDebugGroup(
            debug_group_source.raw_handle(),
            id as _,
            message.len() as _,
            message.as_ptr().cast(),
        );
    }
}
