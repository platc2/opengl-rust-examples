use gl_bindings_raw_handle_derive::RawHandle;

use crate::{define_gl_constants, gl};

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, RawHandle)]
pub struct TextureParameter(pub(crate) gl::GLenum);

impl TextureParameter {
    define_gl_constants!(TextureParameter ::
        TEXTURE_WIDTH,
        TEXTURE_HEIGHT,
        TEXTURE_DEPTH,
        TEXTURE_INTERNAL_FORMAT,
        TEXTURE_RED_SIZE,
        TEXTURE_GREEN_SIZE,
        TEXTURE_BLUE_SIZE,
        TEXTURE_ALPHA_SIZE,
        TEXTURE_DEPTH_SIZE,
        TEXTURE_COMPRESSED,
        TEXTURE_COMPRESSED_IMAGE_SIZE,

        TEXTURE_BASE_LEVEL,
        TEXTURE_BORDER_COLOR,
        TEXTURE_COMPARE_MODE,
        TEXTURE_COMPARE_FUNC,
        TEXTURE_IMMUTABLE_FORMAT,
        TEXTURE_LOD_BIAS,
        TEXTURE_MAG_FILTER,
        TEXTURE_MAX_LEVEL,
        TEXTURE_MAX_LOD,
        TEXTURE_MIN_FILTER,
        TEXTURE_MIN_LOD,
        TEXTURE_SWIZZLE_R,
        TEXTURE_SWIZZLE_G,
        TEXTURE_SWIZZLE_B,
        TEXTURE_SWIZZLE_A,
        TEXTURE_SWIZZLE_RGBA,
        TEXTURE_WRAP_S,
        TEXTURE_WRAP_T,
        TEXTURE_WRAP_R
    );

    #[cfg(feature = "GL42")]
    define_gl_constants!(TextureParameter ::
        IMAGE_FORMAT_COMPATIBILITY_TYPE
    );

    #[cfg(feature = "GL43")]
    define_gl_constants!(TextureParameter ::
        TEXTURE_BUFFER_OFFSET,
        TEXTURE_BUFFER_SIZE,
        DEPTH_STENCIL_TEXTURE_MODE,
        TEXTURE_VIEW_MIN_LAYER,
        TEXTURE_VIEW_MIN_LEVEL,
        TEXTURE_VIEW_NUM_LAYERS,
        TEXTURE_VIEW_NUM_LEVELS,
        TEXTURE_IMMUTABLE_LEVELS
    );

    #[cfg(feature = "GL45")]
    define_gl_constants!(TextureParameter ::
        TEXTURE_TARGET
    );
}
