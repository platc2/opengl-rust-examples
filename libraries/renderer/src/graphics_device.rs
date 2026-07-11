use crate::{Buffer, BufferUsage};

pub trait GraphicsDevice {

    fn create_buffer(&mut self, usage: BufferUsage, size: usize) -> Result<Buffer, ()>;
}
