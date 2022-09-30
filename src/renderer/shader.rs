use std::ffi::{CStr, CString};
use gl::types::{GLchar, GLenum, GLint, GLuint};

use crate::renderer::create_whitespace_cstring_with_len;

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
    pub fn from_source(source: &str, kind: Kind) -> Result<Self, String> {
        let gl_type = match kind {
            Kind::Vertex => gl::VERTEX_SHADER,
            Kind::Fragment => gl::FRAGMENT_SHADER,
        };

        let source = &CString::new(source)
            .expect("Shader source contains invalid characters");
        let handle = shader_from_source(source, gl_type)?;
        Ok(Self { handle, kind })
    }

    pub fn handle(&self) -> GLuint { self.handle }

    pub fn kind(&self) -> Kind { self.kind }
}

fn shader_from_source(source: &CStr, kind: GLenum) -> Result<GLuint, String> {
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

        return Err(error.to_string_lossy().into_owned());
    }

    Ok(handle)
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { gl::DeleteShader(self.handle); }
    }
}
