pub struct WindowOptions {
    pub(crate) title: String,
    pub(crate) width: u16,
    pub(crate) height: u16,
    pub(crate) vsync: bool,
    pub(crate) graphics_requirement: GraphicsRequirement,
}

impl WindowOptions {
    pub fn new<S: Into<String>>(title: S, width: u16, height: u16) -> Self {
        Self {
            title: title.into(),
            width,
            height,
            vsync: false,
            graphics_requirement: GraphicsRequirement::OpenGL(4, 5),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum GraphicsRequirement {
    OpenGL(u8, u8),
}
