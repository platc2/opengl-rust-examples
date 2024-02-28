use gl_bindings_raw_handle_derive::RawHandle;

use crate::{define_gl_constants, gl};

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, RawHandle)]
pub struct RenderbufferTarget(pub(crate) gl::GLenum);

impl RenderbufferTarget {
    define_gl_constants!(RenderbufferTarget ::
        RENDERBUFFER
    );
}
