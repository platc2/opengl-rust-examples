pub use texture_id::*;
pub use texture_parameter::*;
pub use texture_target::*;
pub use texture_unit::*;

use crate::{gl, gl::RawHandle};
use crate::access_type::AccessType;
use crate::buffer::BufferId;
use crate::image_format::ImageFormat;
use crate::pixel_format::PixelFormat;
use crate::pixel_type::PixelType;

mod texture_unit;
mod texture_id;
mod texture_target;
mod texture_parameter;

pub fn active_texture(texture_unit: TextureUnit) {
    unsafe { gl::ActiveTexture(texture_unit.raw_handle()); }
}

#[cfg(feature = "GL42")]
pub fn bind_image_texture(texture_unit: TextureUnit,
                          texture: TextureId,
                          level: usize,
                          layered: bool,
                          layer: usize,
                          access: AccessType,
                          format: ImageFormat) {
    unsafe {
        gl::BindImageTexture(
            texture_unit.raw_handle(),
            texture.raw_handle(),
            level as _,
            layered as _,
            layer as _,
            access.raw_handle(),
            format.raw_handle(),
        );
    }
}

#[cfg(feature = "GL44")]
pub fn bind_image_textures(first: usize, textures: &[TextureId]) {
    unsafe { gl::BindImageTextures(first as _, textures.len() as _, textures.as_ptr().cast()); }
}

pub fn bind_texture(target: TextureTarget, texture: TextureId) {
    unsafe { gl::BindTexture(target.raw_handle(), texture.raw_handle()); }
}

#[cfg(feature = "GL45")]
pub fn bin_texture_unit(texture_unit: TextureUnit, texture: TextureId) {
    unsafe { gl::BindTextureUnit(texture_unit.raw_handle(), texture.raw_handle()); }
}

#[cfg(feature = "GL44")]
pub fn bind_textures(first: usize, textures: &[TextureId]) {
    unsafe { gl::BindTextures(first as _, textures.len() as _, textures.as_ptr().cast()); }
}

#[cfg(feature = "GL44")]
pub fn clear_tex_image<T>(texture: TextureId, level: usize, format: PixelFormat, pixel_type: PixelType, data: &[T]) {
    unsafe {
        gl::ClearTexImage(
            texture.raw_handle(),
            level as _,
            format.raw_handle(),
            pixel_type.raw_handle(),
            data.as_ptr().cast());
    }
}

#[cfg(feature = "GL44")]
pub fn clear_tex_sub_image<T>(texture: TextureId,
                              level: usize,
                              offset: (usize, usize, usize),
                              size: (usize, usize, usize),
                              format: PixelFormat,
                              pixel_type: PixelType,
                              data: &[T]) {
    unsafe {
        gl::ClearTexSubImage(
            texture.raw_handle(),
            level as _,
            offset.0 as _,
            offset.1 as _,
            offset.2 as _,
            size.0 as _,
            size.1 as _,
            size.2 as _,
            format.raw_handle(),
            pixel_type.raw_handle(),
            data.as_ptr().cast(),
        );
    }
}

pub fn compressed_tex_image_1d<T>(texture_target: TextureTarget, level: usize, format: ImageFormat, dimension: (usize, ), data: &[T]) {
    unsafe {
        gl::CompressedTexImage1D(
            texture_target.raw_handle(),
            level as _,
            format.raw_handle(),
            dimension.0 as _,
            0,
            data.len() as _,
            data.as_ptr().cast(),
        );
    }
}

pub fn compressed_tex_image_2d<T>(texture_target: TextureTarget, level: usize, format: ImageFormat, dimension: (usize, usize), data: &[T]) {
    unsafe {
        gl::CompressedTexImage2D(
            texture_target.raw_handle(),
            level as _,
            format.raw_handle(),
            dimension.0 as _,
            dimension.1 as _,
            0,
            data.len() as _,
            data.as_ptr().cast(),
        );
    }
}

pub fn compressed_tex_image_3d<T>(texture_target: TextureTarget, level: usize, format: ImageFormat, dimension: (usize, usize, usize), data: &[T]) {
    unsafe {
        gl::CompressedTexImage3D(
            texture_target.raw_handle(),
            level as _,
            format.raw_handle(),
            dimension.0 as _,
            dimension.1 as _,
            dimension.2 as _,
            0,
            data.len() as _,
            data.as_ptr().cast(),
        );
    }
}

pub fn compressed_tex_sub_image_1d<T>(target: TextureTarget,
                                      level: usize,
                                      offset: (usize, ),
                                      dimension: (usize, ),
                                      format: PixelFormat,
                                      data: &[T]) {
    unsafe {
        gl::CompressedTexSubImage1D(
            target.raw_handle(),
            level as _,
            offset.0 as _,
            dimension.0 as _,
            format.raw_handle(),
            data.len() as _,
            data.as_ptr().cast(),
        );
    }
}

#[cfg(feature = "GL45")]
pub fn compressed_texture_sub_image_1d<T>(texture: TextureId,
                                          level: usize,
                                          offset: (usize, ),
                                          dimension: (usize, ),
                                          format: PixelFormat,
                                          data: &[T]) {
    unsafe {
        gl::CompressedTextureSubImage1D(
            texture.raw_handle(),
            level as _,
            offset.0 as _,
            dimension.0 as _,
            format.raw_handle(),
            data.len() as _,
            data.as_ptr().cast(),
        );
    }
}

pub fn compressed_tex_sub_image_2d<T>(target: TextureTarget,
                                      level: usize,
                                      offset: (usize, usize),
                                      dimension: (usize, usize),
                                      format: PixelFormat,
                                      data: &[T]) {
    unsafe {
        gl::CompressedTexSubImage2D(
            target.raw_handle(),
            level as _,
            offset.0 as _,
            offset.1 as _,
            dimension.0 as _,
            dimension.1 as _,
            format.raw_handle(),
            data.len() as _,
            data.as_ptr().cast(),
        );
    }
}

#[cfg(feature = "GL45")]
pub fn compressed_texture_sub_image_2d<T>(texture: TextureId,
                                          level: usize,
                                          offset: (usize, usize),
                                          dimension: (usize, usize),
                                          format: PixelFormat,
                                          data: &[T]) {
    unsafe {
        gl::CompressedTextureSubImage2D(
            texture.raw_handle(),
            level as _,
            offset.0 as _,
            offset.1 as _,
            dimension.0 as _,
            dimension.1 as _,
            format.raw_handle(),
            data.len() as _,
            data.as_ptr().cast(),
        );
    }
}

pub fn compressed_tex_sub_image_3d<T>(target: TextureTarget,
                                      level: usize,
                                      offset: (usize, usize, usize),
                                      dimension: (usize, usize, usize),
                                      format: PixelFormat,
                                      data: &[T]) {
    unsafe {
        gl::CompressedTexSubImage3D(
            target.raw_handle(),
            level as _,
            offset.0 as _,
            offset.1 as _,
            offset.2 as _,
            dimension.0 as _,
            dimension.1 as _,
            dimension.2 as _,
            format.raw_handle(),
            data.len() as _,
            data.as_ptr().cast(),
        );
    }
}

#[cfg(feature = "GL45")]
pub fn compressed_texture_sub_image_3d<T>(texture: TextureId,
                                          level: usize,
                                          offset: (usize, usize, usize),
                                          dimension: (usize, usize, usize),
                                          format: PixelFormat,
                                          data: &[T]) {
    unsafe {
        gl::CompressedTextureSubImage3D(
            texture.raw_handle(),
            level as _,
            offset.0 as _,
            offset.1 as _,
            offset.2 as _,
            dimension.0 as _,
            dimension.1 as _,
            dimension.2 as _,
            format.raw_handle(),
            data.len() as _,
            data.as_ptr().cast(),
        );
    }
}

#[cfg(feature = "GL43")]
pub fn copy_image_sub_data(source: TextureId,
                           source_target: TextureTarget,
                           source_level: usize,
                           source_position: (usize, usize, usize),
                           destination: TextureId,
                           destination_target: TextureTarget,
                           destination_level: usize,
                           destination_position: (usize, usize, usize),
                           size: (usize, usize, usize)) {
    unsafe {
        gl::CopyImageSubData(
            source.raw_handle(),
            source_target.raw_handle(),
            source_level as _,
            source_position.0 as _,
            source_position.1 as _,
            source_position.2 as _,
            destination.raw_handle(),
            destination_target.raw_handle(),
            destination_level as _,
            destination_position.0 as _,
            destination_position.1 as _,
            destination_position.2 as _,
            size.0 as _,
            size.1 as _,
            size.2 as _,
        );
    }
}

pub fn copy_tex_image_1d(target: TextureTarget,
                         level: usize,
                         format: ImageFormat,
                         position: (usize, usize),
                         size: (usize, )) {
    unsafe {
        gl::CopyTexImage1D(
            target.raw_handle(),
            level as _,
            format.raw_handle(),
            position.0 as _,
            position.1 as _,
            size.0 as _,
            0,
        );
    }
}

pub fn copy_tex_image_2d(target: TextureTarget,
                         level: usize,
                         format: ImageFormat,
                         position: (usize, usize),
                         size: (usize, usize)) {
    unsafe {
        gl::CopyTexImage2D(
            target.raw_handle(),
            level as _,
            format.raw_handle(),
            position.0 as _,
            position.1 as _,
            size.0 as _,
            size.1 as _,
            0,
        );
    }
}

pub fn copy_tex_sub_image_1d(target: TextureTarget,
                             level: usize,
                             offset: (usize, ),
                             position: (usize, usize),
                             size: (usize, )) {
    unsafe {
        gl::CopyTexSubImage1D(
            target.raw_handle(),
            level as _,
            offset.0 as _,
            position.0 as _,
            position.1 as _,
            size.0 as _,
        );
    }
}

#[cfg(feature = "GL45")]
pub fn copy_texture_sub_image_1d(texture: TextureId,
                                 level: usize,
                                 offset: (usize, ),
                                 position: (usize, usize),
                                 size: (usize, )) {
    unsafe {
        gl::CopyTextureSubImage1D(
            texture.raw_handle(),
            level as _,
            offset.0 as _,
            position.0 as _,
            position.1 as _,
            size.0 as _,
        );
    }
}

pub fn copy_tex_sub_image_2d(target: TextureTarget,
                             level: usize,
                             offset: (usize, usize),
                             position: (usize, usize),
                             size: (usize, usize)) {
    unsafe {
        gl::CopyTexSubImage2D(
            target.raw_handle(),
            level as _,
            offset.0 as _,
            offset.1 as _,
            position.0 as _,
            position.1 as _,
            size.0 as _,
            size.1 as _,
        );
    }
}

#[cfg(feature = "GL45")]
pub fn copy_texture_sub_image_2d(texture: TextureId,
                                 level: usize,
                                 offset: (usize, usize),
                                 position: (usize, usize),
                                 size: (usize, usize)) {
    unsafe {
        gl::CopyTextureSubImage2D(
            texture.raw_handle(),
            level as _,
            offset.0 as _,
            offset.1 as _,
            position.0 as _,
            position.1 as _,
            size.0 as _,
            size.1 as _,
        );
    }
}

pub fn copy_tex_sub_image_3d(target: TextureTarget,
                             level: usize,
                             offset: (usize, usize, usize),
                             position: (usize, usize),
                             size: (usize, usize)) {
    unsafe {
        gl::CopyTexSubImage3D(
            target.raw_handle(),
            level as _,
            offset.0 as _,
            offset.1 as _,
            offset.2 as _,
            position.0 as _,
            position.1 as _,
            size.0 as _,
            size.1 as _,
        );
    }
}

#[cfg(feature = "GL45")]
pub fn copy_texture_sub_image_3d(texture: TextureId,
                                 level: usize,
                                 offset: (usize, usize, usize),
                                 position: (usize, usize),
                                 size: (usize, usize)) {
    unsafe {
        gl::CopyTextureSubImage3D(
            texture.raw_handle(),
            level as _,
            offset.0 as _,
            offset.1 as _,
            offset.2 as _,
            position.0 as _,
            position.1 as _,
            size.0 as _,
            size.1 as _,
        );
    }
}

#[cfg(feature = "GL45")]
pub fn create_textures(target: TextureTarget, count: usize) -> Vec<TextureId> {
    let mut texture_ids = Vec::with_capacity(count);

    unsafe {
        let raw_texture_ids_ptr = texture_ids.spare_capacity_mut().as_mut_ptr().cast();
        gl::CreateTextures(target.raw_handle(), count as _, raw_texture_ids_ptr);
        texture_ids.set_len(count);
    }

    texture_ids
}

#[cfg(feature = "GL45")]
pub fn create_texture(target: TextureTarget) -> TextureId {
    let mut texture_id: gl::GLuint = 0;
    unsafe { gl::CreateTextures(target.raw_handle(), 1, &mut texture_id); }
    TextureId(texture_id)
}

pub fn delete_textures(textures: &mut [TextureId]) {
    unsafe { gl::DeleteTextures(textures.len() as _, textures.as_ptr().cast()); }
    for texture in textures {
        *texture = TextureId::NO_TEXTURE;
    }
}

pub fn delete_texture(texture: &mut TextureId) {
    delete_textures(core::slice::from_mut(texture));
}

pub fn gen_textures(count: usize) -> Vec<TextureId> {
    let mut textures = Vec::with_capacity(count);

    unsafe {
        let ptr = textures.spare_capacity_mut().as_ptr() as _;
        gl::GenTextures(count as _, ptr);
        textures.set_len(count);
    }

    textures
}

pub fn get_compressed_tex_image<T>(target: TextureTarget, level: usize, data: &mut [T]) {
    unsafe { gl::GetCompressedTexImage(target.raw_handle(), level as _, data.as_mut_ptr() as _); }
}

#[cfg(feature = "GL45")]
pub fn get_n_compressed_tex_image<T>(target: TextureTarget, level: usize, data: &mut [T]) {
    unsafe { gl::GetnCompressedTexImage(target.raw_handle(), level as _, data.len() as _, data.as_mut_ptr() as _); }
}

#[cfg(feature = "GL45")]
pub fn get_compressed_texture_image<T>(texture: TextureId, level: usize, data: &mut [T]) {
    unsafe { gl::GetCompressedTextureImage(texture.raw_handle(), level as _, data.len() as _, data.as_mut_ptr() as _); }
}

#[cfg(feature = "GL45")]
pub fn get_compressed_texture_sub_image<T>(texture: TextureId,
                                           level: usize,
                                           offset: (usize, usize, usize),
                                           size: (usize, usize, usize),
                                           data: &mut [T]) {
    unsafe {
        gl::GetCompressedTextureSubImage(
            texture.raw_handle(),
            level as _,
            offset.0 as _,
            offset.1 as _,
            offset.2 as _,
            size.0 as _,
            size.1 as _,
            size.2 as _,
            data.len() as _,
            data.as_mut_ptr() as _,
        );
    }
}

pub fn get_tex_image<T>(target: TextureTarget, level: usize, format: PixelFormat, pixel_type: PixelType, pixels: &mut [T]) {
    unsafe {
        gl::GetTexImage(
            target.raw_handle(),
            level as _,
            format.raw_handle(),
            pixel_type.raw_handle(),
            pixels.as_mut_ptr() as _);
    }
}

#[cfg(feature = "GL45")]
pub fn get_n_tex_image<T>(target: TextureTarget, level: usize, format: PixelFormat, pixel_type: PixelType, pixels: &mut [T]) {
    unsafe {
        gl::GetnTexImage(
            target.raw_handle(),
            level as _,
            format.raw_handle(),
            pixel_type.raw_handle(),
            pixels.len() as _,
            pixels.as_mut_ptr() as _);
    }
}

#[cfg(feature = "GL45")]
pub fn get_texture_image<T>(texture: TextureId, level: usize, format: PixelFormat, pixel_type: PixelType, pixels: &mut [T]) {
    unsafe {
        gl::GetnTexImage(
            texture.raw_handle(),
            level as _,
            format.raw_handle(),
            pixel_type.raw_handle(),
            pixels.len() as _,
            pixels.as_mut_ptr() as _);
    }
}

pub fn get_tex_level_parameter_fv(target: TextureTarget, level: usize, parameter: TextureParameter, res: &mut [f32]) {
    unsafe {
        gl::GetTexLevelParameterfv(
            target.raw_handle(),
            level as _,
            parameter.raw_handle(),
            res.as_mut_ptr(),
        );
    }
}

pub fn get_tex_level_parameter_iv(target: TextureTarget, level: usize, parameter: TextureParameter, res: &mut [i32]) {
    unsafe {
        gl::GetTexLevelParameteriv(
            target.raw_handle(),
            level as _,
            parameter.raw_handle(),
            res.as_mut_ptr(),
        );
    }
}

#[cfg(feature = "GL45")]
pub fn get_texture_level_parameter_fv(texture: TextureId, level: usize, parameter: TextureParameter, res: &mut [f32]) {
    unsafe {
        gl::GetTextureLevelParameterfv(
            texture.raw_handle(),
            level as _,
            parameter.raw_handle(),
            res.as_mut_ptr(),
        );
    }
}

#[cfg(feature = "GL45")]
pub fn get_texture_level_parameter_iv(texture: TextureId, level: usize, parameter: TextureParameter, res: &mut [i32]) {
    unsafe {
        gl::GetTextureLevelParameteriv(
            texture.raw_handle(),
            level as _,
            parameter.raw_handle(),
            res.as_mut_ptr(),
        );
    }
}

pub fn get_tex_parameter_fv(target: TextureTarget, parameter: TextureParameter, res: &mut [f32]) {
    unsafe { gl::GetTexParameterfv(target.raw_handle(), parameter.raw_handle(), res.as_mut_ptr()); }
}

pub fn get_tex_parameter_iv(target: TextureTarget, parameter: TextureParameter, res: &mut [i32]) {
    unsafe { gl::GetTexParameteriv(target.raw_handle(), parameter.raw_handle(), res.as_mut_ptr()); }
}

pub fn get_tex_parameter_iiv(target: TextureTarget, parameter: TextureParameter, res: &mut [i32]) {
    unsafe { gl::GetTexParameterIiv(target.raw_handle(), parameter.raw_handle(), res.as_mut_ptr()); }
}

pub fn get_tex_parameter_iuiv(target: TextureTarget, parameter: TextureParameter, res: &mut [u32]) {
    unsafe { gl::GetTexParameterIuiv(target.raw_handle(), parameter.raw_handle(), res.as_mut_ptr()); }
}

#[cfg(feature = "GL45")]
pub fn get_texture_parameter_fv(texture: TextureId, parameter: TextureParameter, res: &mut [f32]) {
    unsafe { gl::GetTextureParameterfv(texture.raw_handle(), parameter.raw_handle(), res.as_mut_ptr()); }
}

#[cfg(feature = "GL45")]
pub fn get_texture_parameter_iv(texture: TextureId, parameter: TextureParameter, res: &mut [i32]) {
    unsafe { gl::GetTextureParameteriv(texture.raw_handle(), parameter.raw_handle(), res.as_mut_ptr()); }
}

#[cfg(feature = "GL45")]
pub fn get_texture_parameter_iiv(texture: TextureId, parameter: TextureParameter, res: &mut [i32]) {
    unsafe { gl::GetTextureParameterIiv(texture.raw_handle(), parameter.raw_handle(), res.as_mut_ptr()); }
}

#[cfg(feature = "GL45")]
pub fn get_texture_parameter_iuiv(texture: TextureId, parameter: TextureParameter, res: &mut [u32]) {
    unsafe { gl::GetTextureParameterIuiv(texture.raw_handle(), parameter.raw_handle(), res.as_mut_ptr()); }
}

#[cfg(feature = "GL45")]
pub fn get_texture_sub_image<T>(texture: TextureId,
                                level: usize,
                                offset: (usize, usize, usize),
                                size: (usize, usize, usize),
                                format: PixelFormat,
                                pixel_type: PixelType,
                                data: &mut [T]) {
    unsafe {
        gl::GetTextureSubImage(
            texture.raw_handle(),
            level as _,
            offset.0 as _,
            offset.1 as _,
            offset.2 as _,
            size.0 as _,
            size.1 as _,
            size.2 as _,
            format.raw_handle(),
            pixel_type.raw_handle(),
            data.len() as _,
            data.as_mut_ptr().cast(),
        );
    }
}

#[cfg(feature = "GL43")]
pub fn invalidate_tex_image(texture: TextureId, level: usize) {
    unsafe { gl::InvalidateTexImage(texture.raw_handle(), level as _); }
}

#[cfg(feature = "GL43")]
pub fn invalidate_tex_sub_image(texture: TextureId,
                                level: usize,
                                offset: (usize, usize, usize),
                                size: (usize, usize, usize)) {
    unsafe {
        gl::InvalidateTexSubImage(
            texture.raw_handle(),
            level as _,
            offset.0 as _,
            offset.1 as _,
            offset.2 as _,
            size.0 as _,
            size.1 as _,
            size.2 as _,
        );
    }
}

pub fn is_texture(texture: TextureId) -> bool {
    let res = unsafe { gl::IsTexture(texture.raw_handle()) };
    res == gl::TRUE
}

pub fn tex_buffer(target: TextureTarget, format: ImageFormat, buffer: BufferId) {
    unsafe { gl::TexBuffer(target.raw_handle(), format.raw_handle(), buffer.raw_handle()); }
}

#[cfg(feature = "GL45")]
pub fn texture_buffer(texture: TextureId, format: ImageFormat, buffer: BufferId) {
    unsafe { gl::TextureBuffer(texture.raw_handle(), format.raw_handle(), buffer.raw_handle()); }
}

#[cfg(feature = "GL43")]
pub fn tex_buffer_range(target: TextureTarget, format: ImageFormat, buffer: BufferId, offset: usize, size: usize) {
    unsafe {
        gl::TexBufferRange(
            target.raw_handle(),
            format.raw_handle(),
            buffer.raw_handle(),
            offset as _,
            size as _,
        );
    }
}

#[cfg(feature = "GL45")]
pub fn texture_buffer_range(texture: TextureId, format: ImageFormat, buffer: BufferId, offset: usize, size: usize) {
    unsafe {
        gl::TextureBufferRange(
            texture.raw_handle(),
            format.raw_handle(),
            buffer.raw_handle(),
            offset as _,
            size as _,
        );
    }
}

pub fn tex_image_1d<T>(target: TextureTarget,
                       level: usize,
                       internal_format: ImageFormat,
                       size: (usize, ),
                       border: usize,
                       format: PixelFormat,
                       pixel_type: PixelType,
                       data: &[T]) {
    unsafe {
        gl::TexImage1D(
            target.raw_handle(),
            level as _,
            internal_format.raw_handle() as _,
            size.0 as _,
            border as _,
            format.raw_handle(),
            pixel_type.raw_handle(),
            data.as_ptr() as _,
        );
    }
}

pub fn tex_image_2d<T>(target: TextureTarget,
                       level: usize,
                       internal_format: ImageFormat,
                       size: (usize, usize),
                       border: usize,
                       format: PixelFormat,
                       pixel_type: PixelType,
                       data: &[T]) {
    unsafe {
        gl::TexImage2D(
            target.raw_handle(),
            level as _,
            internal_format.raw_handle() as _,
            size.0 as _,
            size.1 as _,
            border as _,
            format.raw_handle(),
            pixel_type.raw_handle(),
            data.as_ptr() as _,
        );
    }
}

pub fn tex_image_2d_multisample(target: TextureTarget,
                                samples: usize,
                                internal_format: ImageFormat,
                                size: (usize, usize),
                                fixed_sampled_locations: bool) {
    unsafe {
        gl::TexImage2DMultisample(
            target.raw_handle(),
            samples as _,
            internal_format.raw_handle(),
            size.0 as _,
            size.1 as _,
            fixed_sampled_locations as _,
        );
    }
}

pub fn tex_image_3d<T>(target: TextureTarget,
                       level: usize,
                       internal_format: ImageFormat,
                       size: (usize, usize, usize),
                       border: usize,
                       format: PixelFormat,
                       pixel_type: PixelType,
                       data: &[T]) {
    unsafe {
        gl::TexImage3D(
            target.raw_handle(),
            level as _,
            internal_format.raw_handle() as _,
            size.0 as _,
            size.1 as _,
            size.2 as _,
            border as _,
            format.raw_handle(),
            pixel_type.raw_handle(),
            data.as_ptr() as _,
        );
    }
}

pub fn tex_image_3d_multisample(target: TextureTarget,
                                samples: usize,
                                internal_format: ImageFormat,
                                size: (usize, usize, usize),
                                fixed_sampled_locations: bool) {
    unsafe {
        gl::TexImage3DMultisample(
            target.raw_handle(),
            samples as _,
            internal_format.raw_handle(),
            size.0 as _,
            size.1 as _,
            size.2 as _,
            fixed_sampled_locations as _,
        );
    }
}

pub fn tex_parameter_f(target: TextureTarget, parameter: TextureParameter, param: f32) {
    unsafe { gl::TexParameterf(target.raw_handle(), parameter.raw_handle(), param); }
}

pub fn tex_parameter_fv(target: TextureTarget, parameter: TextureParameter, params: &[f32]) {
    unsafe { gl::TexParameterfv(target.raw_handle(), parameter.raw_handle(), params.as_ptr()); }
}

pub fn tex_parameter_i(target: TextureTarget, parameter: TextureParameter, param: i32) {
    unsafe { gl::TexParameteri(target.raw_handle(), parameter.raw_handle(), param); }
}

pub fn tex_parameter_iv(target: TextureTarget, parameter: TextureParameter, params: &[i32]) {
    unsafe { gl::TexParameteriv(target.raw_handle(), parameter.raw_handle(), params.as_ptr()); }
}

pub fn tex_paramater_iiv(target: TextureTarget, parameter: TextureParameter, params: &[i32]) {
    unsafe { gl::TexParameterIiv(target.raw_handle(), parameter.raw_handle(), params.as_ptr()); }
}

pub fn tex_parameter_iuiv(target: TextureTarget, parameter: TextureParameter, params: &[u32]) {
    unsafe { gl::TexParameterIuiv(target.raw_handle(), parameter.raw_handle(), params.as_ptr()); }
}

#[cfg(feature = "GL45")]
pub fn texture_parameter_f(texture: TextureId, parameter: TextureParameter, param: f32) {
    unsafe { gl::TextureParameterf(texture.raw_handle(), parameter.raw_handle(), param); }
}

#[cfg(feature = "GL45")]
pub fn texture_parameter_fv(texture: TextureId, parameter: TextureParameter, params: &[f32]) {
    unsafe { gl::TextureParameterfv(texture.raw_handle(), parameter.raw_handle(), params.as_ptr()); }
}

#[cfg(feature = "GL45")]
pub fn texture_parameter_i(texture: TextureId, parameter: TextureParameter, param: i32) {
    unsafe { gl::TextureParameteri(texture.raw_handle(), parameter.raw_handle(), param); }
}

#[cfg(feature = "GL45")]
pub fn texture_parameter_iv(texture: TextureId, parameter: TextureParameter, params: &[i32]) {
    unsafe { gl::TextureParameteriv(texture.raw_handle(), parameter.raw_handle(), params.as_ptr()); }
}

#[cfg(feature = "GL45")]
pub fn texture_paramater_iiv(texture: TextureId, parameter: TextureParameter, params: &[i32]) {
    unsafe { gl::TextureParameterIiv(texture.raw_handle(), parameter.raw_handle(), params.as_ptr()); }
}

#[cfg(feature = "GL45")]
pub fn texture_parameter_iuiv(texture: TextureId, parameter: TextureParameter, params: &[u32]) {
    unsafe { gl::TextureParameterIuiv(texture.raw_handle(), parameter.raw_handle(), params.as_ptr()); }
}

#[cfg(feature = "GL42")]
pub fn tex_storage_1d(target: TextureTarget, levels: usize, format: ImageFormat, size: (usize, )) {
    unsafe { gl::TexStorage1D(target.raw_handle(), levels as _, format.raw_handle(), size.0 as _); }
}

#[cfg(feature = "GL45")]
pub fn texture_storage_1d(texture: TextureId, levels: usize, format: ImageFormat, size: (usize, )) {
    unsafe { gl::TextureStorage1D(texture.raw_handle(), levels as _, format.raw_handle(), size.0 as _); }
}

#[cfg(feature = "GL42")]
pub fn tex_storage_2d(target: TextureTarget, levels: usize, format: ImageFormat, size: (usize, usize)) {
    unsafe { gl::TexStorage2D(target.raw_handle(), levels as _, format.raw_handle(), size.0 as _, size.1 as _); }
}

#[cfg(feature = "GL45")]
pub fn texture_storage_2d(texture: TextureId, levels: usize, format: ImageFormat, size: (usize, usize)) {
    unsafe { gl::TextureStorage2D(texture.raw_handle(), levels as _, format.raw_handle(), size.0 as _, size.1 as _); }
}

#[cfg(feature = "GL43")]
pub fn tex_storage_2d_multisample(target: TextureTarget,
                                  samples: usize,
                                  format: ImageFormat,
                                  size: (usize, usize),
                                  fixed_sample_locations: bool) {
    unsafe {
        gl::TexStorage2DMultisample(
            target.raw_handle(),
            samples as _,
            format.raw_handle(),
            size.0 as _,
            size.1 as _,
            fixed_sample_locations as _,
        );
    }
}

#[cfg(feature = "GL44")]
pub fn texture_storage_2d_multisample(texture: TextureId,
                                      samples: usize,
                                      format: ImageFormat,
                                      size: (usize, usize),
                                      fixed_sample_locations: bool) {
    unsafe {
        gl::TextureStorage2DMultisample(
            texture.raw_handle(),
            samples as _,
            format.raw_handle(),
            size.0 as _,
            size.1 as _,
            fixed_sample_locations as _,
        );
    }
}

#[cfg(feature = "GL42")]
pub fn tex_storage_3d(target: TextureTarget, levels: usize, format: ImageFormat, size: (usize, usize, usize)) {
    unsafe { gl::TexStorage3D(target.raw_handle(), levels as _, format.raw_handle(), size.0 as _, size.1 as _, size.2 as _); }
}

#[cfg(feature = "GL45")]
pub fn texture_storage_3d(texture: TextureId, levels: usize, format: ImageFormat, size: (usize, usize, usize)) {
    unsafe { gl::TextureStorage3D(texture.raw_handle(), levels as _, format.raw_handle(), size.0 as _, size.1 as _, size.2 as _); }
}

#[cfg(feature = "GL43")]
pub fn tex_storage_3d_multisample(target: TextureTarget,
                                  samples: usize,
                                  format: ImageFormat,
                                  size: (usize, usize, usize),
                                  fixed_sample_locations: bool) {
    unsafe {
        gl::TexStorage3DMultisample(
            target.raw_handle(),
            samples as _,
            format.raw_handle(),
            size.0 as _,
            size.1 as _,
            size.2 as _,
            fixed_sample_locations as _,
        );
    }
}

#[cfg(feature = "GL44")]
pub fn texture_storage_3d_multisample(texture: TextureId,
                                      samples: usize,
                                      format: ImageFormat,
                                      size: (usize, usize, usize),
                                      fixed_sample_locations: bool) {
    unsafe {
        gl::TextureStorage3DMultisample(
            texture.raw_handle(),
            samples as _,
            format.raw_handle(),
            size.0 as _,
            size.1 as _,
            size.2 as _,
            fixed_sample_locations as _,
        );
    }
}

pub fn tex_sub_image_1d<T>(target: TextureTarget,
                           level: usize,
                           offset: (usize, ),
                           size: (usize, ),
                           format: PixelFormat,
                           pixel_type: PixelType,
                           data: &[T]) {
    unsafe {
        gl::TexSubImage1D(
            target.raw_handle(),
            level as _,
            offset.0 as _,
            size.0 as _,
            format.raw_handle(),
            pixel_type.raw_handle(),
            data.as_ptr().cast(),
        );
    }
}

#[cfg(feature = "GL45")]
pub fn texture_sub_image_1d<T>(texture: TextureId,
                               level: usize,
                               offset: (usize, ),
                               size: (usize, ),
                               format: PixelFormat,
                               pixel_type: PixelType,
                               data: &[T]) {
    unsafe {
        gl::TextureSubImage1D(
            texture.raw_handle(),
            level as _,
            offset.0 as _,
            size.0 as _,
            format.raw_handle(),
            pixel_type.raw_handle(),
            data.as_ptr().cast(),
        );
    }
}

pub fn tex_sub_image_2d<T>(target: TextureTarget,
                           level: usize,
                           offset: (usize, usize),
                           size: (usize, usize),
                           format: PixelFormat,
                           pixel_type: PixelType,
                           data: &[T]) {
    unsafe {
        gl::TexSubImage2D(
            target.raw_handle(),
            level as _,
            offset.0 as _,
            offset.1 as _,
            size.0 as _,
            size.1 as _,
            format.raw_handle(),
            pixel_type.raw_handle(),
            data.as_ptr().cast(),
        );
    }
}

#[cfg(feature = "GL45")]
pub fn texture_sub_image_2d<T>(texture: TextureId,
                               level: usize,
                               offset: (usize, usize),
                               size: (usize, usize),
                               format: PixelFormat,
                               pixel_type: PixelType,
                               data: &[T]) {
    unsafe {
        gl::TextureSubImage2D(
            texture.raw_handle(),
            level as _,
            offset.0 as _,
            offset.1 as _,
            size.0 as _,
            size.1 as _,
            format.raw_handle(),
            pixel_type.raw_handle(),
            data.as_ptr().cast(),
        );
    }
}

pub fn tex_sub_image_3d<T>(target: TextureTarget,
                           level: usize,
                           offset: (usize, usize, usize),
                           size: (usize, usize, usize),
                           format: PixelFormat,
                           pixel_type: PixelType,
                           data: &[T]) {
    unsafe {
        gl::TexSubImage3D(
            target.raw_handle(),
            level as _,
            offset.0 as _,
            offset.1 as _,
            offset.2 as _,
            size.0 as _,
            size.1 as _,
            size.2 as _,
            format.raw_handle(),
            pixel_type.raw_handle(),
            data.as_ptr().cast(),
        );
    }
}

#[cfg(feature = "GL45")]
pub fn texture_sub_image_3d<T>(texture: TextureId,
                               level: usize,
                               offset: (usize, usize, usize),
                               size: (usize, usize, usize),
                               format: PixelFormat,
                               pixel_type: PixelType,
                               data: &[T]) {
    unsafe {
        gl::TextureSubImage3D(
            texture.raw_handle(),
            level as _,
            offset.0 as _,
            offset.1 as _,
            offset.2 as _,
            size.0 as _,
            size.1 as _,
            size.2 as _,
            format.raw_handle(),
            pixel_type.raw_handle(),
            data.as_ptr().cast(),
        );
    }
}

#[cfg(feature = "GL43")]
pub fn texture_view(texture: TextureId,
                    target: TextureTarget,
                    original_texture: TextureId,
                    format: ImageFormat,
                    min_level: usize,
                    num_levels: usize,
                    min_layer: usize,
                    num_layers: usize) {
    unsafe {
        gl::TextureView(
            texture.raw_handle(),
            target.raw_handle(),
            original_texture.raw_handle(),
            format.raw_handle(),
            min_level as _,
            num_levels as _,
            min_layer as _,
            num_layers as _,
        );
    }
}
