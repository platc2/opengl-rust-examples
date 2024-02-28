use gl_bindings_raw_handle_derive::RawHandle;

use crate::{define_gl_constants, gl};

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, RawHandle)]
pub struct FramebufferTarget(pub(crate) gl::GLenum);

impl FramebufferTarget {
    define_gl_constants!(FramebufferTarget ::
        DRAW_FRAMEBUFFER,
        READ_FRAMEBUFFER,
        FRAMEBUFFER
    );
}
