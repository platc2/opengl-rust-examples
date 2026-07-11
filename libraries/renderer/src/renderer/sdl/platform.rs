use crate::window_options::{GraphicsRequirement, WindowOptions};
use crate::{Platform, SDLOpenGLGraphicsDevice, SdlWindow};
use sdl2::video::GLProfile;
use sdl2::{EventPump, Sdl, VideoSubsystem};

pub struct SdlPlatform {
    sdl: Sdl,
    video_subsystem: VideoSubsystem,
    event_pump: EventPump,
}

impl SdlPlatform {
    pub fn init() -> crate::Result<Self> {
        let sdl = sdl2::init().map_err(crate::PlatformError::InitialisationError)?;
        let video_subsystem = sdl
            .video()
            .map_err(crate::PlatformError::InitialisationError)?;
        video_subsystem
            .gl_attr()
            .set_context_profile(sdl2::video::GLProfile::Core);
        let event_pump = sdl
            .event_pump()
            .map_err(crate::PlatformError::InitialisationError)?;
        Ok(Self {
            sdl,
            video_subsystem,
            event_pump,
        })
    }
}

impl Platform for SdlPlatform {
    type Window = SdlWindow;

    fn create_window(&mut self, window_options: WindowOptions) -> crate::Result<Self::Window> {
        let window = self
            .video_subsystem
            .window(
                window_options.title.as_str(),
                window_options.width.into(),
                window_options.height.into(),
            )
            .apply_graphics_requirement(window_options.graphics_requirement)
            .position_centered()
            .build()
            .map_err(|err| crate::PlatformError::WindowCreationError(err.to_string()))?;

        let graphics_device = match window_options.graphics_requirement {
            GraphicsRequirement::OpenGL(major, minor) => {
                let gl_attributes = self.video_subsystem.gl_attr();
                gl_attributes.set_context_major_version(major);
                gl_attributes.set_context_minor_version(minor);
                gl_attributes.set_context_profile(GLProfile::Core);
                let gl_context = window
                    .gl_create_context()
                    .map_err(crate::PlatformError::WindowCreationError)?;
                let gl = gl::Gl::load_with(|s| self.video_subsystem.gl_get_proc_address(s).cast());
                SDLOpenGLGraphicsDevice::new(gl_context, gl)
            }
        };

        Ok(SdlWindow {
            window,
            graphics_requirement: window_options.graphics_requirement,
            graphics_device,
        })
    }
}

trait GraphicsRequirementExt {
    fn apply_graphics_requirement(&mut self, requirement: GraphicsRequirement) -> &mut Self;
}

impl GraphicsRequirementExt for sdl2::video::WindowBuilder {
    fn apply_graphics_requirement(&mut self, requirement: GraphicsRequirement) -> &mut Self {
        match requirement {
            GraphicsRequirement::OpenGL(_, _) => self.opengl(),
        }
    }
}
