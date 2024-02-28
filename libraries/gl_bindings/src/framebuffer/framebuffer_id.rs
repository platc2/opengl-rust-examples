use gl_bindings_raw_handle_derive::RawHandle;

use crate::gl;

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, RawHandle)]
pub struct FramebufferId(pub(crate) gl::GLenum);

impl FramebufferId {
    pub const DEFAULT_FRAMEBUFFER: FramebufferId = FramebufferId(0);
}
