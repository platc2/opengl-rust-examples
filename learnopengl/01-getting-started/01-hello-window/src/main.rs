extern crate anyhow;
extern crate learnopengl_utils as utils;
extern crate renderer;

use std::time::Instant;

use anyhow::Result;
use renderer::application;
use renderer::application::{App, Application};
use renderer::input::InputManager;
use renderer::renderer_context::{OpenGLVersion, RendererContext, WindowDimension};
use renderer::time::Time;
use utils::gl;

#[derive(Default)]
struct State;

impl App for State {
    fn new() -> Result<Self>
    where
        Self: Sized,
    {
        Ok(Self)
    }
}

impl Application for State {
    fn tick(&mut self, _: &Time<Instant>, _: &dyn InputManager) {
        gl::viewport((0, 0), (800, 600));

        gl::clear_color(0xFF334C4C);
        gl::clear(gl::ClearMask::COLOR_BUFFER_BIT);
    }
}

pub fn main() -> anyhow::Result<()> {
    let context = RendererContext::init(
        "LearnOpenGL",
        &WindowDimension::of(800, 600),
        &OpenGLVersion::of(3, 3),
    )?;

    application::start::<State>(context)
}
