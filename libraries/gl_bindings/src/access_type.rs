use gl_bindings_raw_handle_derive::RawHandle;

use crate::{define_gl_constants, gl};

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, RawHandle)]
pub struct AccessType(pub(crate) gl::GLenum);

impl AccessType {
    define_gl_constants!(AccessType ::
        READ_ONLY,
        WRITE_ONLY,
        READ_WRITE
    );
}
