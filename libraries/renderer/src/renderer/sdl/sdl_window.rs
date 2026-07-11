use crate::{GraphicsRequirement, SDLOpenGLGraphicsDevice};

pub struct SdlWindow {
    pub(crate) window: sdl2::video::Window,
    pub(crate) graphics_requirement: GraphicsRequirement,
    pub(crate) graphics_device: SDLOpenGLGraphicsDevice,
}

impl SdlWindow {
    pub fn graphics_device(&self) -> &SDLOpenGLGraphicsDevice {
        &self.graphics_device
    }

    pub fn graphics_device_mut(&mut self) -> &mut SDLOpenGLGraphicsDevice {
        &mut self.graphics_device
    }
}
