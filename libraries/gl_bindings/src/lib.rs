extern crate gl_bindings_raw_handle_derive;

pub use sys::load_with;

pub mod buffer;
pub mod shader;
pub mod program;
pub mod capabilities;
pub mod rendering;
pub mod state;
pub mod vertex_array;
pub mod vertex_attrib;
pub mod debug;
pub mod texture;
pub mod framebuffer;
pub mod image_format;
pub mod access_type;
pub mod pixel_format;
pub mod pixel_type;
pub mod error;

mod gl {
    pub use crate::sys::*;
    pub use crate::sys::types::*;
}

macro_rules! define_gl_constants {
    ($t:ident :: $($name:ident),+) => {
        $(
            pub const $name: $t = $t(gl::$name);
        )+
    };
}

pub(crate) use define_gl_constants;

pub mod sys {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

    pub trait RawHandle<T> {
        /// # Safety
        /// The raw types should not be handled manually, unless not possible
        unsafe fn raw_handle(&self) -> T;

        /// # Safety
        /// The raw types should not be handled manually, unless not possible
        unsafe fn from_raw(value: T) -> Self;
    }
}
