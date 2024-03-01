extern crate anyhow;
extern crate gl_bindings;
extern crate stb_image;

use anyhow::{anyhow, Result};
use stb_image::image::LoadResult;

pub mod gl {
    pub use gl_bindings::access_type::*;
    pub use gl_bindings::buffer::*;
    pub use gl_bindings::capabilities::*;
    pub use gl_bindings::debug::*;
    pub use gl_bindings::error::*;
    pub use gl_bindings::framebuffer::*;
    pub use gl_bindings::image_format::*;
    pub use gl_bindings::pixel_format::*;
    pub use gl_bindings::pixel_type::*;
    pub use gl_bindings::program::*;
    pub use gl_bindings::rendering::*;
    pub use gl_bindings::shader::*;
    pub use gl_bindings::state::*;
    pub use gl_bindings::sys;
    pub use gl_bindings::texture::*;
    pub use gl_bindings::vertex_array::*;
    pub use gl_bindings::vertex_attrib::*;
}

pub fn program(vertex_shader_source: &str, fragment_shader_source: &str) -> Result<gl::ProgramId> {
    let program = gl::create_program();
    let mut vertex_shader = shader(gl::ShaderKind::VERTEX_SHADER, vertex_shader_source)?;
    let mut fragment_shader = shader(gl::ShaderKind::FRAGMENT_SHADER, fragment_shader_source)?;
    gl::attach_shader(program, vertex_shader);
    gl::attach_shader(program, fragment_shader);
    gl::link_program(program);

    gl::delete_shader(&mut vertex_shader);
    gl::delete_shader(&mut fragment_shader);

    if gl::program_link_status(program) {
        Ok(program)
    } else {
        let info_log = gl::program_info_log(program);
        Err(anyhow!("Failed to link shader program: {}", info_log.unwrap_or("Unknown error".to_owned())))
    }
}

fn shader(shader_kind: gl::ShaderKind, shader_source: &str) -> Result<gl::ShaderId> {
    let shader = gl::create_shader(shader_kind);
    gl::shader_source(shader, shader_source);
    gl::compile_shader(shader);
    if gl::shader_compile_status(shader) {
        Ok(shader)
    } else {
        let info_log = gl::shader_info_log(shader);
        Err(anyhow!("Error compiling {shader_kind:?} shader: {}", info_log.unwrap_or("Unknown error".to_owned())))
    }
}

pub fn load_texture_2d(texture_data: &[u8]) -> Result<gl::TextureId> {
    let texture = gl::create_texture(gl::TextureTarget::TEXTURE_2D);
    gl::bind_texture(gl::TextureTarget::TEXTURE_2D, texture);

    unsafe {
        // Hack because the wrapper library does not support setting this parameter (yet)
        stb_image::stb_image::stbi_set_flip_vertically_on_load(1);
    }

    let image_data = stb_image::image::load_from_memory(texture_data);
    match image_data {
        LoadResult::Error(e) => Err(anyhow!(e)),
        image_data => {
            match image_data {
                LoadResult::ImageU8(image_data) => gl::tex_image_2d(
                    gl::TextureTarget::TEXTURE_2D,
                    0,
                    gl::ImageFormat::RGB,
                    (image_data.width, image_data.height),
                    0,
                    pixel_format_from_depth(image_data.depth),
                    gl::PixelType::UNSIGNED_BYTE,
                    &image_data.data[..],
                ),
                LoadResult::ImageF32(image_data) => gl::tex_image_2d(
                    gl::TextureTarget::TEXTURE_2D,
                    0,
                    gl::ImageFormat::RGB,
                    (image_data.width, image_data.height),
                    0,
                    pixel_format_from_depth(image_data.depth),
                    gl::PixelType::FLOAT,
                    image_data.data.as_slice(),
                ),
                _ => panic!("Impossible, checked above!"),
            }

            gl::tex_parameter_iuiv(gl::TextureTarget::TEXTURE_2D, gl::TextureParameter::TEXTURE_MIN_FILTER, &[gl::sys::NEAREST]);
            gl::tex_parameter_iuiv(gl::TextureTarget::TEXTURE_2D, gl::TextureParameter::TEXTURE_MAG_FILTER, &[gl::sys::NEAREST]);
            gl::tex_parameter_iuiv(gl::TextureTarget::TEXTURE_2D, gl::TextureParameter::TEXTURE_WRAP_S, &[gl::sys::CLAMP_TO_EDGE]);
            gl::tex_parameter_iuiv(gl::TextureTarget::TEXTURE_2D, gl::TextureParameter::TEXTURE_WRAP_T, &[gl::sys::CLAMP_TO_EDGE]);
            gl::generate_mipmap(gl::TextureTarget::TEXTURE_2D);

            Ok(texture)
        }
    }
}

fn pixel_format_from_depth(depth: usize) -> gl::PixelFormat {
    match depth {
        1 => gl::PixelFormat::RED,
        2 => gl::PixelFormat::RG,
        3 => gl::PixelFormat::RGB,
        4 => gl::PixelFormat::RGBA,
        _ => panic!("Texture has too many channels: {}", depth)
    }
}
