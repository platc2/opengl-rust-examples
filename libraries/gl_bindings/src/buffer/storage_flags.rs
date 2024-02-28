use std::ops::BitOr;

use gl_bindings_raw_handle_derive::RawHandle;

use crate::{define_gl_constants, gl};

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, RawHandle)]
pub struct StorageFlags(gl::GLbitfield);

impl StorageFlags {
    pub const NO_FLAGS: StorageFlags = StorageFlags(0);

    define_gl_constants!(StorageFlags ::
        DYNAMIC_STORAGE_BIT,
        MAP_READ_BIT,
        MAP_WRITE_BIT,
        MAP_PERSISTENT_BIT,
        MAP_COHERENT_BIT,
        CLIENT_STORAGE_BIT
    );
}

impl BitOr for StorageFlags {
    type Output = StorageFlags;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}
