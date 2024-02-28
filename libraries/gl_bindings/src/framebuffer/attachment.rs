use gl_bindings_raw_handle_derive::RawHandle;

use crate::{define_gl_constants, gl};

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, RawHandle)]
pub struct Attachment(gl::GLenum);

impl Attachment {
    define_gl_constants!(Attachment ::
        COLOR_ATTACHMENT0,
        COLOR_ATTACHMENT1,
        COLOR_ATTACHMENT2,
        COLOR_ATTACHMENT3,
        COLOR_ATTACHMENT4,
        COLOR_ATTACHMENT5,
        COLOR_ATTACHMENT6,
        COLOR_ATTACHMENT7,
        COLOR_ATTACHMENT8,
        COLOR_ATTACHMENT9,
        COLOR_ATTACHMENT10,
        COLOR_ATTACHMENT11,
        COLOR_ATTACHMENT12,
        COLOR_ATTACHMENT13,
        COLOR_ATTACHMENT14,
        COLOR_ATTACHMENT15,
        COLOR_ATTACHMENT16,
        COLOR_ATTACHMENT17,
        COLOR_ATTACHMENT18,
        COLOR_ATTACHMENT19,
        COLOR_ATTACHMENT20,
        COLOR_ATTACHMENT21,
        COLOR_ATTACHMENT22,
        COLOR_ATTACHMENT23,
        COLOR_ATTACHMENT24,
        COLOR_ATTACHMENT25,
        COLOR_ATTACHMENT26,
        COLOR_ATTACHMENT27,
        COLOR_ATTACHMENT28,
        COLOR_ATTACHMENT29,
        COLOR_ATTACHMENT30,
        COLOR_ATTACHMENT31,
        DEPTH_ATTACHMENT,
        STENCIL_ATTACHMENT,
        DEPTH_STENCIL_ATTACHMENT
    );
}
