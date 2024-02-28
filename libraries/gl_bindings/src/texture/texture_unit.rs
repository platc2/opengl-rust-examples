use gl_bindings_raw_handle_derive::RawHandle;
use crate::gl;

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, RawHandle)]
pub struct TextureUnit(pub(crate) gl::GLenum);

impl TextureUnit {
    pub fn fixed(value: usize) -> Self {
        let mut max_combine_texture_image_units = 0;
        unsafe { gl::GetIntegerv(gl::MAX_COMBINED_TEXTURE_IMAGE_UNITS, &mut max_combine_texture_image_units); }

        if value < (max_combine_texture_image_units as _) {
            Self(gl::TEXTURE0 + (value as u32))
        } else {
            panic!("Not enough texture units available for {value}!")
        }
    }
}
