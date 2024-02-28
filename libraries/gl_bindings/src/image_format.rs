use crate::{gl, define_gl_constants};
use gl_bindings_raw_handle_derive::RawHandle;

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, RawHandle)]
pub struct ImageFormat(gl::GLenum);

impl ImageFormat {
    // Unsized formats
    define_gl_constants!(ImageFormat ::
        DEPTH_COMPONENT,
        DEPTH_STENCIL,
        RED,
        RG,
        RGB,
        RGBA
    );

    // Color formats
    define_gl_constants!(ImageFormat ::
        // R
        R8,     R8_SNORM,                   R8I,        R8UI,
        R16,    R16_SNORM,      R16F,       R16I,       R16UI,
                                R32F,       R32I,       R32UI,
        // RG
        RG8,    RG8_SNORM,                  RG8I,       RG8UI,
        RG16,   RG16_SNORM,     RG16F,      RG16I,      RG16UI,
                                RG32F,      RG32I,      RG32UI,
        // RGB
        RGB4,
        RGB5,
        RGB8,   RGB8_SNORM,                 RGB8I,      RGB8UI,
        RGB10,
        RGB12,
        RGB16,  RGB16_SNORM,    RGB16F,     RGB16I,     RGB16UI,
                                RGB32F,     RGB32I,     RGB32UI,
        // RGBA
        RGBA2,
        RGBA4,
        RGBA8,  RGBA8_SNORM,                RGBA8I,     RGBA8UI,
        RGBA12,
        RGBA16, RGBA16_SNORM,   RGBA16F,    RGBA16I,    RGBA16UI,
                                RGBA32F,    RGBA32I,    RGBA32UI,
        // Special
        R3_G3_B2,
        RGB5_A1,
        RGB10_A2,
        RGB10_A2UI,
        R11F_G11F_B10F,
        RGB9_E5,
        RGB565,
        // sRGB
        SRGB8,
        SRGB8_ALPHA8,
        // Compressed
        COMPRESSED_RED,
        COMPRESSED_RG,
        COMPRESSED_RGB,
        COMPRESSED_RGBA,
        COMPRESSED_SRGB,
        COMPRESSED_SRGB_ALPHA,

        COMPRESSED_RED_RGTC1,
        COMPRESSED_SIGNED_RED_RGTC1,
        COMPRESSED_RG_RGTC2,
        COMPRESSED_SIGNED_RG_RGTC2
    );

    #[cfg(feature = "GL42")]
    define_gl_constants!(ImageFormat ::
        COMPRESSED_RGBA_BPTC_UNORM,
        COMPRESSED_SRGB_ALPHA_BPTC_UNORM,
        COMPRESSED_RGB_BPTC_SIGNED_FLOAT,
        COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT
    );

    // Depth formats
    define_gl_constants!(ImageFormat ::
        DEPTH_COMPONENT16,
        DEPTH_COMPONENT24,
        DEPTH_COMPONENT32,
        DEPTH_COMPONENT32F
    );

    // Depth stencil formats
    define_gl_constants!(ImageFormat ::
        DEPTH24_STENCIL8,
        DEPTH32F_STENCIL8
    );

    // Stencil formats
    define_gl_constants!(ImageFormat ::
        STENCIL_INDEX,
        STENCIL_INDEX1,
        STENCIL_INDEX4,
        STENCIL_INDEX8,
        STENCIL_INDEX16
    );
}
