use gl_bindings_raw_handle_derive::RawHandle;

use crate::{define_gl_constants, gl};

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, RawHandle)]
pub struct PixelFormat(gl::GLenum);

impl PixelFormat {
    define_gl_constants!(PixelFormat ::
        RED,
        RG,
        RGB, BGR,
        RGBA, BGRA,
        RED_INTEGER,
        RG_INTEGER,
        RGB_INTEGER, BGR_INTEGER,
        RGBA_INTEGER, BGRA_INTEGER,
        STENCIL_INDEX,
        DEPTH_COMPONENT,
        DEPTH_STENCIL
    );
}
