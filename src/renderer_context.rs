use std::ffi::c_void;

use sdl2::Sdl;
use sdl2::video::{GLContext, GLProfile, Window, WindowBuildError};
use thiserror::Error;

use crate::renderer_context::Error::{ContextInit, SdlInit, VideoSubsystemInit};

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to initialize SDL: {0}")]
    SdlInit(String),

    #[error("Failed to initialize video subsystem: {0}")]
    VideoSubsystemInit(String),

    #[error("Failed to initialize window: {0}")]
    WindowInit(#[from] WindowBuildError),

    #[error("Failed to initialize context: {0}")]
    ContextInit(String),
}


type Result<T> = std::result::Result<T, Error>;

pub struct RendererContext {
    sdl: Sdl,
    window: Window,
    _gl_context: GLContext,
}

impl RendererContext {
    pub fn init() -> Result<Self> {
        let sdl = sdl2::init()
            .map_err(|str| SdlInit(str))?;
        let video_subsystem = sdl.video()
            .map_err(|str| VideoSubsystemInit(str))?;
        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(GLProfile::Core);
        gl_attr.set_context_version(4, 3);
        gl_attr.set_context_flags()
            .debug()
            .set();
        let window = video_subsystem
            .window("Hello Cube", 900, 700)
            .opengl()
            .resizable()
            .build()?;
        let _gl_context = window.gl_create_context()
            .map_err(|e| ContextInit(e))?;
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s).cast::<c_void>());

        Ok(Self { sdl, window, _gl_context })
    }

    pub const fn sdl(&self) -> &Sdl { &self.sdl }

    pub const fn window(&self) -> &Window { &self.window }
}
