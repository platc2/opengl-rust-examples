use std::fmt::{Debug, Formatter};

use gl_bindings_raw_handle_derive::RawHandle;

use crate::{define_gl_constants, gl};

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, RawHandle)]
pub struct Error(pub(crate) gl::GLenum);

impl Error {
    define_gl_constants!(Error ::
        NO_ERROR,
        INVALID_ENUM,
        INVALID_VALUE,
        INVALID_OPERATION,
        INVALID_FRAMEBUFFER_OPERATION,
        OUT_OF_MEMORY,
        STACK_UNDERFLOW,
        STACK_OVERFLOW
    );
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            Error::NO_ERROR => write!(f, "Error(NO_ERROR)"),
            Error::INVALID_ENUM => write!(f, "Error(INVALID_ENUM)"),
            Error::INVALID_VALUE => write!(f, "Error(INVALID_VALUE)"),
            Error::INVALID_OPERATION => write!(f, "Error(INVALID_OPERATION)"),
            Error::INVALID_FRAMEBUFFER_OPERATION => write!(f, "Error(INVALID_FRAMEBUFFER_OPERATION)"),
            Error::OUT_OF_MEMORY => write!(f, "Error(OUT_OF_MEMORY)"),
            Error::STACK_UNDERFLOW => write!(f, "Error(STACK_UNDERFLOW)"),
            Error::STACK_OVERFLOW => write!(f, "Error(STACK_OVERFLOW)"),
            _ => write!(f, "Error(UNKNOWN)"),
        }
    }
}

pub fn get_error() -> Error {
    Error(unsafe { gl::GetError() })
}
