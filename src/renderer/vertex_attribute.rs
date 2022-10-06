
#[derive(Copy, Clone)]
pub enum Format {
    R32F,
    RG32F,
    RGB32F,
    RGBA32F,
    R8,
    RG8,
    RGB8,
    RGBA8,
}

pub struct VertexAttribute {
    format: Format,
    offset: u16,
}

impl VertexAttribute {
    pub const fn new(format: Format, offset: u16) -> Self {
        Self { format, offset }
    }

    pub const fn format(&self) -> Format { self.format }

    pub const fn offset(&self) -> u16 { self.offset }
}
