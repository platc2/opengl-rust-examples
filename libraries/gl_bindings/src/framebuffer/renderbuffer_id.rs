use gl_bindings_raw_handle_derive::RawHandle;

use crate::gl;

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, RawHandle)]
pub struct RenderbufferId(pub(crate) gl::GLenum);

impl RenderbufferId {
    pub const NO_RENDERBUFFER: RenderbufferId = RenderbufferId(0);
}
