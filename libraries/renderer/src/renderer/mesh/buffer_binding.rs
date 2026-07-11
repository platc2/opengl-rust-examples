use crate::Buffer;
use std::rc::Rc;

pub struct BufferBinding {
    pub(super) buffer: Rc<Buffer>,
    pub(super) offset: isize,
    pub(super) stride: i32,
}

impl BufferBinding {
    #[must_use]
    pub const fn new(buffer: Rc<Buffer>, offset: isize, stride: i32) -> Self {
        Self {
            buffer,
            offset,
            stride,
        }
    }
}
