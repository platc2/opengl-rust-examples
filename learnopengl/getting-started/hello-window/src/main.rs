extern crate anyhow;
extern crate renderer;

use std::time::Instant;

use anyhow::Result;

use renderer::application;
use renderer::application::Application;
use renderer::input_manager::{InputManager, Key};
use renderer::renderer_context::{OpenGLVersion, RendererContext, WindowDimension};
use renderer::time::Time;

#[derive(Default)]
struct State {
    quit: bool,
}

mod gl {
    pub use renderer::gl::rendering::*;
    pub use renderer::gl::state::*;
}

impl Application for State {
    fn tick(&mut self, _: &Time<Instant>, input_manager: &dyn InputManager) {
        if input_manager.key_down(Key::ESCAPE) {
            self.quit = true;
        }

        gl::viewport((0, 0), (800, 600));

        gl::clear_color(0xFF334C4C);
        gl::clear(gl::ClearMask::COLOR_BUFFER_BIT);
    }

    fn quit(&self) -> bool { self.quit }
}

pub fn main() -> Result<()> {
    let context = RendererContext::init(
        "LearnOpenGL",
        &WindowDimension::of(800, 600),
        &OpenGLVersion::of(3, 3),
    )?;

    let state = State::default();

    application::main_loop(context, state)
}
