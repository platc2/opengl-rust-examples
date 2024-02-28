use stb_image::image::LoadResult;
use thiserror::Error;

use ::gl::sys::RawHandle;

use crate::renderer::texture::ImageLoadingError::{InvalidImage, UnsupportedFormat};

mod gl {
    pub use gl::image_format::*;
    pub use gl::pixel_format::*;
    pub use gl::pixel_type::*;
    pub use gl::sys;
    pub use gl::texture::*;
}

#[derive(Debug, Error)]
pub enum ImageLoadingError {
    #[error("Image data invalid: {0}")]
    InvalidImage(String),

    #[error("Image format unsupported")]
    UnsupportedFormat,

    #[error("Resource error: {0}")]
    Resource(#[from] crate::resources::Error),

    #[error("Image is too large")]
    TooLarge,
}

type Result<T> = std::result::Result<T, ImageLoadingError>;

pub struct Texture {
    id: gl::TextureId,
    width: usize,
    height: usize,
}

impl Texture {
    pub fn from_raw_1(image_data: &[u8], width: usize, height: usize) -> Result<Self> {
        let id = gl::create_texture(gl::TextureTarget::TEXTURE_2D);
        gl::texture_parameter_i(id, gl::TextureParameter::TEXTURE_MIN_FILTER, gl::sys::LINEAR as _);
        gl::texture_parameter_i(id, gl::TextureParameter::TEXTURE_MAG_FILTER, gl::sys::LINEAR as _);
        gl::texture_parameter_i(id, gl::TextureParameter::TEXTURE_WRAP_S, gl::sys::CLAMP_TO_EDGE as _);
        gl::texture_parameter_i(id, gl::TextureParameter::TEXTURE_WRAP_T, gl::sys::CLAMP_TO_EDGE as _);

        gl::bind_texture(gl::TextureTarget::TEXTURE_2D, id);
        gl::tex_image_2d(
            gl::TextureTarget::TEXTURE_2D,
            0,
            gl::ImageFormat::R8,
            (width, height),
            0,
            gl::PixelFormat::RED,
            gl::PixelType::UNSIGNED_BYTE,
            image_data,
        );

        unsafe {
            gl::sys::GenerateTextureMipmap(id.raw_handle());
        }
        Ok(Self {
            id,
            width,
            height,
        })
    }

    /// # Errors
    pub fn from_raw(image_data: &[u8], width: usize, height: usize) -> Result<Self> {
        let id = gl::create_texture(gl::TextureTarget::TEXTURE_2D);

        gl::texture_parameter_i(id, gl::TextureParameter::TEXTURE_MIN_FILTER, gl::sys::LINEAR as _);
        gl::texture_parameter_i(id, gl::TextureParameter::TEXTURE_MAG_FILTER, gl::sys::LINEAR as _);

        gl::bind_texture(gl::TextureTarget::TEXTURE_2D, id);
        gl::tex_image_2d(
            gl::TextureTarget::TEXTURE_2D,
            0,
            gl::ImageFormat::RGBA32F,
            (width, height),
            0,
            gl::PixelFormat::RGBA,
            gl::PixelType::UNSIGNED_BYTE,
            image_data);

        unsafe { gl::sys::GenerateTextureMipmap(id.raw_handle()); }
        Ok(Self {
            id,
            width,
            height,
        })
    }
    /// # Errors
    /// - [`Error::InvalidImage`]
    /// - [`Error::UnsupportedFormat`]
    pub fn from(image_data: &[u8]) -> Result<Self> {
        let id = gl::create_texture(gl::TextureTarget::TEXTURE_2D);

        gl::texture_parameter_i(id, gl::TextureParameter::TEXTURE_MIN_FILTER, gl::sys::LINEAR_MIPMAP_LINEAR as _);
        gl::texture_parameter_i(id, gl::TextureParameter::TEXTURE_MAG_FILTER, gl::sys::LINEAR as _);

        gl::bind_texture(gl::TextureTarget::TEXTURE_2D, id);
        let stb_image = stb_image::image::load_from_memory(image_data);
        let (width, height) = match &stb_image {
            LoadResult::Error(error) => Err(InvalidImage(error.to_string())),
            LoadResult::ImageU8(image_data) => {
                gl::tex_image_2d(
                    gl::TextureTarget::TEXTURE_2D,
                    0,
                    gl::ImageFormat::RGBA32F,
                    (image_data.width, image_data.height),
                    0,
                    format_from_depth(image_data.depth)?,
                    gl::PixelType::UNSIGNED_BYTE,
                    image_data.data.as_slice(),
                );
                Ok((image_data.width, image_data.height))
            }
            LoadResult::ImageF32(image_data) => {
                gl::tex_image_2d(
                    gl::TextureTarget::TEXTURE_2D,
                    0,
                    gl::ImageFormat::RGBA32F,
                    (image_data.width, image_data.height),
                    0,
                    format_from_depth(image_data.depth)?,
                    gl::PixelType::FLOAT,
                    image_data.data.as_slice(),
                );
                Ok((image_data.width, image_data.height))
            }
        }?;

        unsafe {
            gl::sys::GenerateTextureMipmap(id.raw_handle());
        }
        // We don't require to check width & height as they've been validated above
        #[allow(clippy::cast_sign_loss)]
        Ok(Self {
            id,
            width,
            height,
        })
    }

    #[must_use]
    pub fn blank(width: usize, height: usize) -> Self {
        let id = gl::create_texture(gl::TextureTarget::TEXTURE_2D);

        gl::texture_parameter_i(id, gl::TextureParameter::TEXTURE_MIN_FILTER, gl::sys::LINEAR as _);
        gl::texture_parameter_i(id, gl::TextureParameter::TEXTURE_MAG_FILTER, gl::sys::LINEAR as _);

        gl::bind_texture(gl::TextureTarget::TEXTURE_2D, id);
        gl::tex_image_2d::<u8>(
            gl::TextureTarget::TEXTURE_2D,
            0,
            gl::ImageFormat::RGBA32F,
            (width, height),
            0,
            gl::PixelFormat::RGBA,
            gl::PixelType::UNSIGNED_BYTE,
            &[],
        );

        Self {
            id,
            width,
            height,
        }
    }

    #[must_use]
    pub fn handle(&self) -> gl::sys::types::GLuint {
        unsafe { self.id.raw_handle() }
    }

    #[must_use]
    pub const fn width(&self) -> usize {
        self.width
    }

    #[must_use]
    pub const fn height(&self) -> usize {
        self.height
    }
}

const fn format_from_depth(depth: usize) -> Result<gl::PixelFormat> {
    match depth {
        1 => Ok(gl::PixelFormat::RED),
        2 => Ok(gl::PixelFormat::RG),
        3 => Ok(gl::PixelFormat::RGB),
        4 => Ok(gl::PixelFormat::RGBA),
        _ => Err(UnsupportedFormat),
    }
}
