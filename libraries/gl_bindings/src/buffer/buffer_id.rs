use gl_bindings_raw_handle_derive::RawHandle;

use crate::gl;

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, RawHandle)]
pub struct BufferId(pub(crate) gl::GLuint);

impl BufferId {
    pub const NO_BUFFER: BufferId = BufferId(0);
}
