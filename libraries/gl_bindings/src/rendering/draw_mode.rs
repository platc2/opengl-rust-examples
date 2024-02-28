use gl_bindings_raw_handle_derive::RawHandle;

use crate::{define_gl_constants, gl};

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, RawHandle)]
pub struct DrawMode(pub(crate) gl::GLenum);

impl DrawMode {
    define_gl_constants!(DrawMode ::
        POINTS,
        LINE_STRIP,
        LINE_LOOP,
        LINES,
        TRIANGLE_STRIP,
        TRIANGLE_FAN,
        TRIANGLES
    );
}
