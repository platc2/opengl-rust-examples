use gl_bindings_raw_handle_derive::RawHandle;

use crate::{define_gl_constants, gl};

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, RawHandle)]
pub struct BufferUsage(gl::GLenum);

impl BufferUsage {
    define_gl_constants!(BufferUsage ::
        STREAM_DRAW,
        STATIC_DRAW,
        STREAM_READ,
        STATIC_READ,
        STREAM_COPY,
        STATIC_COPY
    );
}
