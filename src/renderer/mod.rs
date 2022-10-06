use std::ffi::CString;

pub use self::buffer::{Buffer, Usage as BufferUsage};
pub use self::program::Program;
pub use self::render_pass::{RenderPass, VertexBinding};
pub use self::shader::{Shader, Kind as ShaderKind};
pub use self::vertex_attribute::{Format as VertexAttributeFormat, VertexAttribute};
pub use self::texture::Texture;

mod buffer;
mod shader;
mod program;
mod render_pass;
mod vertex_attribute;
mod texture;

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    buffer.extend([b' '].iter().cycle().take(len));
    unsafe { CString::from_vec_unchecked(buffer) }
}
