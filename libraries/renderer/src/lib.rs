#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

pub extern crate gl_bindings as gl;
extern crate nalgebra_glm as glm;
extern crate sdl2;
extern crate stb_image as stbi;
extern crate thiserror;

pub use renderer::*;

mod renderer;

#[cfg(feature = "imgui")]
mod imgui_impl;

pub mod renderer_context;
pub mod resources;
pub mod application;
pub mod time;
pub mod input_manager;
