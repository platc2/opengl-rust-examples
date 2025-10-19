extern crate anyhow;
extern crate nalgebra_glm as glm;

mod component;
mod game_object;
mod polyhedron;
mod scene_graph;
mod state;
mod transform;

use crate::state::State;
use anyhow::Result;
use renderer::application;
use renderer::application::Application;
use renderer::input_manager::InputManager;
use renderer::renderer_context::{OpenGLVersion, RendererContext, WindowDimension};
use renderer::resources::Resources;
use std::path::Path;

pub fn main() -> Result<()> {
    let window_dimension = WindowDimension::default();
    // Initialize render-context
    let context =
        RendererContext::init("Scene Graph", &window_dimension, &OpenGLVersion::default())?;

    let res = Resources::from_relative_exe_path(Path::new("assets"))?;

    let _state = State::new(res)?;
    application::main_loop(context, _state)
}
