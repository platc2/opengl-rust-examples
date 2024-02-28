use thiserror::Error;

use ::gl::sys::RawHandle;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Buffer too large")]
    TooLarge,
}

type Result<T> = std::result::Result<T, Error>;

mod gl {
    pub use gl::buffer::*;
    pub use gl::sys;
}

#[derive(Copy, Clone)]
pub enum Usage {
    Vertex,
    Index,
    Uniform,
}

pub struct Buffer {
    id: gl::BufferId,
    target: gl::BufferTarget,

    size: usize,
    usage: Usage,
}

impl Buffer {
    /// # Errors
    /// - Allocated buffer size too large
    pub fn allocate(usage: Usage, size: usize) -> Result<Self> {
        let target = match usage {
            Usage::Vertex => gl::BufferTarget::ARRAY_BUFFER,
            Usage::Index => gl::BufferTarget::ELEMENT_ARRAY_BUFFER,
            Usage::Uniform => gl::BufferTarget::UNIFORM_BUFFER,
        };

        let bit_flags = match usage {
            Usage::Vertex | Usage::Index => gl::StorageFlags::NO_FLAGS,
            Usage::Uniform => gl::StorageFlags::MAP_COHERENT_BIT | gl::StorageFlags::MAP_PERSISTENT_BIT,
        };

        let id = gl::create_buffer();
        gl::bind_buffer(target, id);
        gl::buffer_storage_empty(target, size, gl::StorageFlags::MAP_WRITE_BIT | bit_flags);

        Ok(Self {
            id,
            target,
            size,
            usage,
        })
    }

    #[must_use]
    pub fn handle(&self) -> gl::sys::types::GLuint {
        unsafe { self.id.raw_handle() }
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
        gl::bind_buffer(self.target, self.id);
        unsafe {
            let memory_pointer = gl::sys::MapBuffer(self.target.raw_handle(), gl::sys::WRITE_ONLY).cast::<Type>();
            std::slice::from_raw_parts_mut(memory_pointer, self.size / std::mem::size_of::<Type>())
        }
    }

    pub fn unmap(&self) {
        unsafe {
            gl::sys::UnmapBuffer(self.target.raw_handle());
        }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        gl::delete_buffer(&mut self.id);
    }
}
