use gl::types::{GLint, GLuint};
use crate::renderer::{create_whitespace_cstring_with_len, Shader};

pub struct Program {
    handle: GLuint,
}

impl Program {
    /// # Errors
    /// - Program failed to link
    pub fn from_shaders(shaders: &[&Shader]) -> Result<Self, String> {
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

            let error = create_whitespace_cstring_with_len(usize::try_from(len)
                .expect("Error string too long for display!"));
            unsafe {
                gl::GetProgramInfoLog(handle, len, std::ptr::null_mut(), error.as_ptr() as *mut gl::types::GLchar);
            }

            return Err(error.to_string_lossy().into_owned());
        }

        Ok(Self { handle })
    }

    pub fn set_used(&self) {
        unsafe { gl::UseProgram(self.handle); }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.handle); }
    }
}
