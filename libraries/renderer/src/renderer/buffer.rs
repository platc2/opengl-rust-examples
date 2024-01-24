use gl::types::{GLenum, GLsizeiptr, GLuint};
use thiserror::Error;

use crate::renderer::buffer::Error::TooLarge;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Buffer too large")]
    TooLarge,
}
type Result<T> = std::result::Result<T, Error>;

#[derive(Copy, Clone)]
pub enum Usage {
    Vertex,
    Index,
    Uniform,
}

const UNIFORM_BUFFER_FLAGS: GLenum = gl::MAP_PERSISTENT_BIT | gl::MAP_COHERENT_BIT;

pub struct Buffer {
    handle: GLuint,
    target: GLenum,

    size: usize,
    usage: Usage,
}

impl Buffer {
    /// # Errors
    /// - Allocated buffer size too large
    pub fn allocate(usage: Usage, size: usize) -> Result<Self> {
        let target = match usage {
            Usage::Vertex => gl::ARRAY_BUFFER,
            Usage::Index => gl::ELEMENT_ARRAY_BUFFER,
            Usage::Uniform => gl::UNIFORM_BUFFER,
        };

        let bit_flags = match usage {
            Usage::Vertex | Usage::Index => 0,
            Usage::Uniform => UNIFORM_BUFFER_FLAGS,
        };

        let mut handle: GLuint = 0;
        let gl_size = GLsizeiptr::try_from(size).map_err(|_| TooLarge)?;
        unsafe {
            gl::CreateBuffers(1, &mut handle);
            gl::BindBuffer(target, handle);
            gl::BufferStorage(
                target,
                gl_size,
                std::ptr::null(),
                gl::MAP_WRITE_BIT | bit_flags,
            );
        }

        Ok(Self {
            handle,
            target,
            size,
            usage,
        })
    }

    #[must_use]
    pub const fn handle(&self) -> GLuint {
        self.handle
    }

    #[must_use]
    pub const fn size(&self) -> usize {
        self.size
    }

    #[must_use]
    pub const fn usage(&self) -> Usage {
        self.usage
    }

    pub fn map<Type>(&mut self) -> &mut [Type]
    where
        Type: Sized,
    {
        unsafe {
            gl::BindBuffer(self.target, self.handle);
            let memory_pointer = gl::MapBuffer(self.target, gl::WRITE_ONLY).cast::<Type>();
            std::slice::from_raw_parts_mut(memory_pointer, self.size / std::mem::size_of::<Type>())
        }
    }

    pub fn unmap(&self) {
        unsafe {
            gl::UnmapBuffer(self.target);
        }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
//            gl_bindings::DeleteBuffers(1, &self.handle);
        }
    }
}
