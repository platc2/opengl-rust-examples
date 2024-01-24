use thiserror::Error;

use gl::types::{GLchar, GLint, GLuint};

use crate::renderer::Shader;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Program failed to link: {0}")]
    ProgramLink(String),
}

type Result<T> = std::result::Result<T, Error>;

pub struct Program {
    handle: GLuint,
}

impl Program {
    /// # Errors
    /// - Program failed to link
    pub fn from_shaders(shaders: &[&Shader]) -> Result<Self> {
        let handle = unsafe { gl::CreateProgram() };

        unsafe {
            for shader in shaders {
                gl::AttachShader(handle, shader.handle());
            }
            gl::LinkProgram(handle);
            for shader in shaders {
                gl::DetachShader(handle, shader.handle());
            }
        }

        let mut success: GLint = 1;
        unsafe {
            gl::GetProgramiv(handle, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: GLint = 0;
            unsafe {
                gl::GetProgramiv(handle, gl::INFO_LOG_LENGTH, &mut len);
            }

            // GL_INFO_LOG_LENGTH contains a positive number or 0 if no information is available
            let error_string_length = usize::try_from(len).unwrap_or(0);
            let mut error_string = String::with_capacity(error_string_length);
            error_string.extend([' '].iter().cycle().take(error_string_length));
            unsafe {
                gl::GetProgramInfoLog(
                    handle,
                    len,
                    std::ptr::null_mut(),
                    error_string.as_mut_ptr().cast::<GLchar>(),
                );
            }

            return Err(Error::ProgramLink(error_string));
        }

        Ok(Self { handle })
    }

    pub fn set_used(&self) {
        unsafe {
            gl::UseProgram(self.handle);
        }
    }

    #[must_use]
    pub const fn handle(&self) -> GLuint {
        self.handle
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.handle);
        }
    }
}
