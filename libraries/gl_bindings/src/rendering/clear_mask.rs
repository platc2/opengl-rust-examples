use std::ops::BitOr;

use gl_bindings_raw_handle_derive::RawHandle;

use crate::{define_gl_constants, gl, gl::RawHandle};

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, RawHandle)]
pub struct ClearMask(pub(crate) gl::GLenum);

impl ClearMask {
    define_gl_constants!(ClearMask ::
        COLOR_BUFFER_BIT,
        DEPTH_BUFFER_BIT,
        STENCIL_BUFFER_BIT
    );
}

impl BitOr for ClearMask {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(unsafe { self.raw_handle() | rhs.raw_handle() })
    }
}
