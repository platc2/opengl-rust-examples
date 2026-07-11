use sdl2::video::{GLContext, GLProfile, Window, WindowBuildError};
use sdl2::{EventPump, Sdl};
use thiserror::Error;

use crate::event;
use crate::event::{DomainEvent, EventHandler, Sdl2EventHandlers};
use crate::renderer_context::Error::{
    ContextInit, EventPumpInit, SdlInit, VideoSubsystemInit, WindowTooLarge,
};
use gl_bindings as gl;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to initialize SDL: {0}")]
    SdlInit(String),

    #[error("Failed to initialize video subsystem: {0}")]
    VideoSubsystemInit(String),

    #[error("Failed to initialize event pump: {0}")]
    EventPumpInit(String),

    #[error("Failed to initialize window: {0}")]
    WindowInit(#[from] WindowBuildError),

    #[error("Failed to initialize context: {0}")]
    ContextInit(String),

    #[error("Window dimension are too large")]
    WindowTooLarge,
}

type Result<T> = std::result::Result<T, Error>;

pub struct RendererContext {
    sdl: Sdl,
    event_pump: EventPump,
    window: Window,
    _gl_context: GLContext,
    sdl2_event_translator: Sdl2EventHandlers<Vec<DomainEvent>>,
}

pub struct WindowDimension {
    pub width: usize,
    pub height: usize,
}

impl WindowDimension {
    #[must_use]
    pub const fn of(width: usize, height: usize) -> Self {
        Self { width, height }
    }
}

impl Default for WindowDimension {
    fn default() -> Self {
        Self {
            width: 900,
            height: 700,
        }
    }
}

pub struct OpenGLVersion {
    major: u8,
    minor: u8,
}

impl Default for OpenGLVersion {
    fn default() -> Self {
        Self::of(4, 1)
    }
}

impl OpenGLVersion {
    #[must_use]
    pub const fn of(major: u8, minor: u8) -> Self {
        Self { major, minor }
    }
}

fn create_sdl_event_translator() -> Sdl2EventHandlers<Vec<DomainEvent>> {
    let mut event_translator = event::Sdl2EventHandlers::new();
    event_translator.add_quit_handler(|| vec![DomainEvent::QuitRequested]);
    event_translator.add_all_keydown_handler(|scancode, _| {
        scancode
            .try_into()
            .map_or_else(|_| vec![], |key| vec![DomainEvent::KeyDown(key)])
    });
    event_translator.add_all_keyup_handler(|scancode, _| {
        scancode
            .try_into()
            .map_or_else(|_| vec![], |key| vec![DomainEvent::KeyUp(key)])
    });
    event_translator.add_mouse_motion_handler(|motion| {
        vec![DomainEvent::MouseMotion {
            x: motion.x,
            y: motion.y,
            delta_x: motion.delta_x,
            delta_y: motion.delta_y,
        }]
    });
    event_translator
        .add_all_mouse_button_down_handler(|button| vec![DomainEvent::MouseButtonDown(button)]);
    event_translator
        .add_all_mouse_button_up_handler(|button| vec![DomainEvent::MouseButtonUp(button)]);
    event_translator.add_mouse_wheel_handler(|wheel| {
        vec![DomainEvent::MouseWheel {
            x: wheel.x,
            y: wheel.y,
        }]
    });
    event_translator.add_text_input_handler(|text| vec![DomainEvent::TextInput(text)]);

    event_translator
}

impl RendererContext {
    /// # Errors
    /// - SDL failed to initialise
    /// - SDL video subsystem failed to initialise
    /// - Failed to create SDL window
    /// - Failed to initialise OpenGL context
    pub fn init(
        window_title: &str,
        window_dimension: &WindowDimension,
        opengl_version: &OpenGLVersion,
    ) -> Result<Self> {
        let sdl = sdl2::init().map_err(SdlInit)?;
        let video_subsystem = sdl.video().map_err(VideoSubsystemInit)?;
        let event_pump = sdl.event_pump().map_err(EventPumpInit)?;
        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(GLProfile::Core);
        gl_attr.set_context_major_version(opengl_version.major);
        gl_attr.set_context_minor_version(opengl_version.minor);
        gl_attr.set_context_flags().debug().set();
        let window = video_subsystem
            .window(
                window_title,
                u32::try_from(window_dimension.width).map_err(|_| WindowTooLarge)?,
                u32::try_from(window_dimension.height).map_err(|_| WindowTooLarge)?,
            )
            .opengl()
            .resizable()
            .build()?;
        let gl_context = window.gl_create_context().map_err(ContextInit)?;
/*
        gl::load_with(|s| {
            video_subsystem
                .gl_get_proc_address(s)
                .cast::<std::ffi::c_void>()
        });
*/

/*
        unsafe {
            gl::sys::Enable(gl::sys::DEBUG_OUTPUT);
            gl::sys::Enable(gl::sys::DEBUG_OUTPUT_SYNCHRONOUS);
            gl::sys::DebugMessageCallback(Some(debug_msg), core::ptr::null());
            gl::sys::DebugMessageControl(
                gl::sys::DONT_CARE,
                gl::sys::DONT_CARE,
                gl::sys::DONT_CARE,
                0,
                core::ptr::null(),
                gl::sys::TRUE,
            );
        }
*/

        sdl.mouse().set_relative_mouse_mode(true);

        Ok(Self {
            sdl,
            event_pump,
            window,
            _gl_context: gl_context,
            sdl2_event_translator: create_sdl_event_translator(),
        })
    }

    pub fn events(&mut self) -> Vec<DomainEvent> {
        self.event_pump
            .poll_iter()
            .flat_map(|event| self.sdl2_event_translator.handle_event(event))
            .collect()
    }

    #[must_use]
    pub fn window_dimension(&self) -> WindowDimension {
        let (width, height) = self.window.size();
        WindowDimension::of(width as usize, height as usize)
    }

    pub fn swap_buffers(&self) {
        self.window.gl_swap_window();
    }

    pub fn set_relative_mouse_mode(&self, mode: bool) {
        self.sdl.mouse().set_relative_mouse_mode(mode);
    }
}

extern "system" fn debug_msg(
    source: gl::sys::types::GLenum,
    _gltype: gl::sys::types::GLenum,
    id: gl::sys::types::GLuint,
    _severity: gl::sys::types::GLenum,
    _length: gl::sys::types::GLsizei,
    message: *const gl::sys::types::GLchar,
    _user_param: *mut std::ffi::c_void,
) {
    if id == 131169 || id == 131185 || id == 131218 || id == 131204 {
        return;
    };

    println!("---------------");
    let message = unsafe { core::ffi::CStr::from_ptr(message) };
    let message = message.to_str().unwrap();
    println!("Debug message ({id}): {message}");
    match source {
        gl::sys::DEBUG_SOURCE_API => println!("Source: API"),
        gl::sys::DEBUG_SOURCE_WINDOW_SYSTEM => println!("Source: Window system"),
        gl::sys::DEBUG_SOURCE_SHADER_COMPILER => println!("Source: Shader compiler"),
        gl::sys::DEBUG_SOURCE_THIRD_PARTY => println!("Source: Third party"),
        gl::sys::DEBUG_SOURCE_APPLICATION => println!("Source: Application"),
        gl::sys::DEBUG_SOURCE_OTHER => println!("Source: Other"),
        _ => panic!("Unknown source"),
    }
}
