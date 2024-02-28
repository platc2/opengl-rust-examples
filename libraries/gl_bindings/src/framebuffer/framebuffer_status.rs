use gl_bindings_raw_handle_derive::RawHandle;

use crate::{define_gl_constants, gl};

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, RawHandle)]
pub struct FramebufferStatus(pub(crate) gl::GLenum);

impl FramebufferStatus {
    define_gl_constants!(FramebufferStatus ::
        FRAMEBUFFER_COMPLETE,
        FRAMEBUFFER_UNDEFINED,
        FRAMEBUFFER_INCOMPLETE_ATTACHMENT,
        FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT,
        FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER,
        FRAMEBUFFER_INCOMPLETE_READ_BUFFER,
        FRAMEBUFFER_UNSUPPORTED,
        FRAMEBUFFER_INCOMPLETE_MULTISAMPLE,
        FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS
    );
}