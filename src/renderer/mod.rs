use std::ffi::CString;

pub use self::buffer::Buffer;
pub use self::program::Program;
pub use self::shader::Shader;
pub use self::render_pass::RenderPass;
pub use self::vertex_attribute::VertexAttribute;

pub mod buffer;
pub mod shader;
pub mod program;
pub mod render_pass;
pub mod vertex_attribute;

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    buffer.extend([b' '].iter().cycle().take(len));
    unsafe { CString::from_vec_unchecked(buffer) }
}
