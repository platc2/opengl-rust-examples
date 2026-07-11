#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::must_use_candidate)]

extern crate alloc;
extern crate core;
extern crate gl_bindings as gl;
extern crate imgui;
extern crate renderer;

use anyhow::Result;
use renderer::{application, SDLOpenGLGraphicsDevice, Platform, SdlPlatform, WindowOptions};
use renderer::renderer_context::{OpenGLVersion, RendererContext, WindowDimension};
use crate::hello_triangle::HelloTriangle;

mod gamma_window;
mod hello_triangle;

fn main() -> Result<()> {
    let mut platform = SdlPlatform::init()?;
    let window = platform.create_window(WindowOptions::new("Hello Triangle", 900, 700))?;
    let graphics_device = window.graphics_device();
/*
        let context = RendererContext::init(
            "Hello Triangle",
            &WindowDimension::of(900, 700),
            &OpenGLVersion::of(4, 5),
        )?;

    application::start::<HelloTriangle>(context)
 */
    Ok(())
}
