use crate::gl;
use gl_bindings_raw_handle_derive::RawHandle;

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, RawHandle)]
pub struct TextureId(pub(crate) gl::GLuint);

impl TextureId {
    pub const NO_TEXTURE: TextureId = TextureId(0);
}
