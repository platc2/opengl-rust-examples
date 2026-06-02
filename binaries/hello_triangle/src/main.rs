#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::must_use_candidate)]

extern crate alloc;
extern crate core;
extern crate gl_bindings as gl;
extern crate imgui;
extern crate renderer;

use anyhow::Result;

use renderer::application;
use renderer::renderer_context::{OpenGLVersion, RendererContext, WindowDimension};

use hello_triangle::HelloTriangle;

mod gamma_window;
mod hello_triangle;

fn main() -> Result<()> {
    let context = RendererContext::init(
        "Hello Triangle",
        &WindowDimension::of(900, 700),
        &OpenGLVersion::of(4, 5),
    )?;

    application::start::<HelloTriangle>(context)
}
