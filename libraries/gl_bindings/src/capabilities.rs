use gl_bindings_raw_handle_derive::RawHandle;

use crate::{define_gl_constants, gl, gl::RawHandle};

#[derive(Debug, Copy, Clone, PartialEq, Eq, RawHandle)]
pub struct Capability(gl::GLenum);

impl Capability {
    define_gl_constants!(Capability ::
        BLEND,
        CULL_FACE,
        DEPTH_TEST,
        STENCIL_TEST,
        SCISSOR_TEST
    );
}

pub fn enable(capability: Capability) {
    unsafe { gl::Enable(capability.raw_handle()); }
}

pub fn disable(capability: Capability) {
    unsafe { gl::Disable(capability.raw_handle()); }
}

pub fn is_enabled(capability: Capability) -> bool {
    let enabled = unsafe { gl::IsEnabled(capability.raw_handle()) };
    enabled == gl::TRUE
}
