use std::ffi::{CStr, CString};

use gl::types::{GLchar, GLenum, GLint, GLuint};
use thiserror::Error;

use crate::renderer::create_whitespace_cstring_with_len;
use crate::renderer::shader::Error::UnsupportedFileExtension;
use crate::resources::Resources;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Shader type could not be guessed from file extension: {0}")]
    UnsupportedFileExtension(String),

    #[error("Resource error: {0}")]
    ResourceError(#[from] crate::resources::Error),

    #[error("UTF-8 Error: {0}")]
    Utf8Error(#[from] core::str::Utf8Error),

    #[error("Shader failed to compile: {0}")]
    ShaderCompilation(String),
}
type Result<T> = std::result::Result<T, Error>;

#[derive(Copy, Clone)]
pub enum Kind {
    Vertex,
    Fragment,
}

pub struct Shader {
    handle: GLuint,
    kind: Kind,
}

impl Shader {
    /// # Errors
    /// - Shader compilation error
    pub fn from_res(res: &Resources, name: &str) -> Result<Self> {
        const POSSIBLE_EXT: [(&str, Kind); 2] = [
            (".vert", Kind::Vertex),
            (".frag", Kind::Fragment),
        ];

        let shader_kind = POSSIBLE_EXT.iter()
            .find(|&&(file_extension, _)| name.ends_with(file_extension))
            .map(|&(_, kind)| kind)
            .ok_or_else(|| UnsupportedFileExtension(String::from(name)))?;
        let source = res.load_cstring(name)?;

        Self::from_source(source.to_str()?, shader_kind)
    }

    /// # Errors
    /// - Shader compilation error
    pub fn from_source(source: &str, kind: Kind) -> Result<Self> {
        let gl_type = match kind {
            Kind::Vertex => gl::VERTEX_SHADER,
            Kind::Fragment => gl::FRAGMENT_SHADER,
        };

        let source = &CString::new(source)
            .expect("Shader source contains invalid characters");
        let handle = shader_from_source(source, gl_type)?;
        Ok(Self { handle, kind })
    }

    pub const fn handle(&self) -> GLuint { self.handle }

    pub const fn kind(&self) -> Kind { self.kind }
}

fn shader_from_source(source: &CStr, kind: GLenum) -> Result<GLuint> {
    let handle = unsafe { gl::CreateShader(kind) };

    unsafe {
        gl::ShaderSource(handle, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(handle);
    }

    let mut success: GLint = 1;
    unsafe {
        gl::GetShaderiv(handle, gl::COMPILE_STATUS, &mut success);
    }

    if success == 0 {
        let mut len: GLint = 0;
        unsafe {
            gl::GetShaderiv(handle, gl::INFO_LOG_LENGTH, &mut len);
        }

        let error = create_whitespace_cstring_with_len(usize::try_from(len)
            .expect("Error string too long for display!"));
        unsafe {
            gl::GetShaderInfoLog(handle, len, std::ptr::null_mut(), error.as_ptr() as *mut GLchar);
        }

        return Err(Error::ShaderCompilation(error.to_string_lossy().into_owned()));
    }

    Ok(handle)
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { gl::DeleteShader(self.handle); }
    }
}
