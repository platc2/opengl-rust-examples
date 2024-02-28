use gl_bindings_raw_handle_derive::RawHandle;

use crate::{define_gl_constants, gl};

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, RawHandle)]
pub struct PixelType(gl::GLenum);

impl PixelType {
    define_gl_constants!(PixelType ::
        UNSIGNED_BYTE,
        BYTE,
        UNSIGNED_SHORT,
        SHORT,
        UNSIGNED_INT,
        INT,
        HALF_FLOAT,
        FLOAT,
        UNSIGNED_BYTE_3_3_2,
        UNSIGNED_BYTE_2_3_3_REV,
        UNSIGNED_SHORT_5_6_5,
        UNSIGNED_SHORT_5_6_5_REV,
        UNSIGNED_SHORT_4_4_4_4,
        UNSIGNED_SHORT_4_4_4_4_REV,
        UNSIGNED_SHORT_5_5_5_1,
        UNSIGNED_SHORT_1_5_5_5_REV,
        UNSIGNED_INT_8_8_8_8,
        UNSIGNED_INT_8_8_8_8_REV,
        UNSIGNED_INT_10_10_10_2,
        UNSIGNED_INT_2_10_10_10_REV
    );
}
