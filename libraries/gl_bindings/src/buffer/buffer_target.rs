use gl_bindings_raw_handle_derive::RawHandle;

use crate::{define_gl_constants, gl};

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, RawHandle)]
pub struct BufferTarget(gl::GLenum);

impl BufferTarget {
    define_gl_constants!(BufferTarget ::
        ARRAY_BUFFER,
        COPY_READ_BUFFER,
        COPY_WRITE_BUFFER,
        DRAW_INDIRECT_BUFFER,
        ELEMENT_ARRAY_BUFFER,
        PIXEL_PACK_BUFFER,
        PIXEL_UNPACK_BUFFER,
        TEXTURE_BUFFER,
        TRANSFORM_FEEDBACK_BUFFER,
        UNIFORM_BUFFER
    );

    #[cfg(feature = "GL42")]
    define_gl_constants!(BufferTarget ::
        ATOMIC_COUNTER_BUFFER
    );

    #[cfg(feature = "GL43")]
    define_gl_constants!(BufferTarget ::
        DISPATCH_INDIRECT_BUFFER,
        SHADER_STORAGE_BUFFER
    );

    #[cfg(feature = "GL44")]
    define_gl_constants!(BufferTarget ::
        QUERY_BUFFER
    );
}
