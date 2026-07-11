use sdl2::video::GLContext;
use crate::{Buffer, BufferUsage};
use crate::graphics_device::GraphicsDevice;

pub struct SDLOpenGLGraphicsDevice {
    opengl_context: GLContext,
    gl: gl::Gl,
}

impl SDLOpenGLGraphicsDevice {
    pub const fn new(opengl_context: GLContext, gl: gl::Gl) -> Self {
        Self { opengl_context, gl }
    }
}

impl GraphicsDevice for SDLOpenGLGraphicsDevice {
    fn create_buffer(&mut self, usage: BufferUsage, size: usize) -> Result<Buffer, ()> {
        Buffer::allocate(usage, size).map_err(|_| ())
    }
}
