use gl_bindings_raw_handle_derive::RawHandle;

use crate::{define_gl_constants, gl};

#[derive(Debug, Copy, Clone, PartialEq, Eq, RawHandle)]
pub struct TextureTarget(pub(crate) gl::GLenum);

impl TextureTarget {
    define_gl_constants!(TextureTarget ::
        TEXTURE_1D,
        TEXTURE_2D,
        TEXTURE_3D,
        TEXTURE_1D_ARRAY,
        TEXTURE_2D_ARRAY,
        TEXTURE_RECTANGLE,
        TEXTURE_CUBE_MAP,
        TEXTURE_CUBE_MAP_ARRAY,
        TEXTURE_BUFFER,
        TEXTURE_2D_MULTISAMPLE,
        TEXTURE_2D_MULTISAMPLE_ARRAY
    );
}
