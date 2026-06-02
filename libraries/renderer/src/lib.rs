#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

pub extern crate gl_bindings as gl;
extern crate nalgebra_glm as glm;
extern crate sdl2;
extern crate stb_image as stbi;
extern crate thiserror;

pub use renderer::*;

mod renderer;

mod imgui_impl;

pub mod application;
mod event;
pub mod input;
pub mod renderer_context;
pub mod resources;
pub mod time;
