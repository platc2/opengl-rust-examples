#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::must_use_candidate)]

extern crate alloc;
extern crate core;
extern crate gl_bindings as gl;
extern crate imgui;
extern crate renderer;
extern crate sdl2;

use std::path::Path;
use std::time::Instant;

use anyhow::{Context, Result};
use imgui::Ui;

use gl::types::{GLintptr, GLsizei};
use renderer::{application, Buffer, BufferUsage, RenderPass, Shader, ShaderKind, VertexAttribute, VertexBinding};
use renderer::application::Application;
use renderer::input_manager::{InputManager, Key};
use renderer::renderer_context::{OpenGLVersion, RendererContext, WindowDimension};
use renderer::resources::Resources;
use renderer::time::Time;

struct State {
    main_render_pass: RenderPass,
    gamma_buffer: Buffer,
    vertex_buffer: Buffer,
    gamma: f32,

    quit: bool,
}

impl Application for State {
    fn tick(&mut self, _: &Time<Instant>, input_manager: &dyn InputManager) {
        if input_manager.key_down(Key::ESCAPE) {
            self.quit = true;
        }

        unsafe {
            self.main_render_pass.display();

            let gamma_ptr = self.gamma_buffer.map::<f32>();
            gamma_ptr.copy_from_slice(&[self.gamma]);
            self.gamma_buffer.unmap();

            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::Viewport(0, 0, 900, 700);
            gl::BindVertexBuffer(
                0,
                self.vertex_buffer.handle(),
                0 as GLintptr,
                GLsizei::try_from(std::mem::size_of::<f32>() * 5).unwrap(),
            );
            gl::BindVertexBuffer(
                1,
                self.vertex_buffer.handle(),
                GLintptr::try_from(std::mem::size_of::<f32>() * 2).unwrap(),
                GLsizei::try_from(std::mem::size_of::<f32>() * 5).unwrap(),
            );
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }

    fn gui(&mut self, ui: &Ui) {
        ui.window("Settings")
            .save_settings(false)
            .always_auto_resize(true)
            .build(|| {
                ui.slider("Gamma", 0.5f32, 2.5f32, &mut self.gamma);
                if ui.button("Reset (1.0)") {
                    self.gamma = 1f32;
                }
                ui.same_line();
                if ui.button("Reset (2.2)") {
                    self.gamma = 2.2f32;
                }
                ui.text(format!("Delta time: {}", ui.io().delta_time));
            });
    }

    fn quit(&self) -> bool {
        self.quit
    }
}


fn main() -> Result<()> {
    let context = RendererContext::init(
        "Hello Triangle",
        &WindowDimension::of(900, 700),
        &OpenGLVersion::of(4, 5),
    )?;

    let res = Resources::from_relative_exe_path(Path::new("assets"))?;

    let vertex_buffer = initialize_vertices()?;

    let vertex_shader = res
        .load_string("/shaders/triangle.vert")
        .map_err(Into::into)
        .and_then(|source| Shader::from_source(&source, ShaderKind::Vertex))
        .context("Failed to initialize vertex shader")?;
    let fragment_shader = res
        .load_string("/shaders/triangle.frag")
        .map_err(Into::into)
        .and_then(|source| Shader::from_source(&source, ShaderKind::Fragment))
        .context("Failed to initialize fragment shader")?;

    let gamma_buffer = Buffer::allocate(BufferUsage::Uniform, std::mem::size_of::<f32>())?;

    let vertex_bindings = [
        VertexBinding::new(
            0,
            VertexAttribute::new(renderer::VertexAttributeFormat::RG32F, 0),
        ),
        VertexBinding::new(
            1,
            VertexAttribute::new(renderer::VertexAttributeFormat::RGB32F, 0),
        ),
    ];

    let main_render_pass = RenderPass::new(
        &vertex_shader,
        &fragment_shader,
        &vertex_bindings,
        &[&gamma_buffer],
        &[],
        &[],
    )?;

    let state = State {
        main_render_pass,
        gamma_buffer,
        vertex_buffer,
        gamma: 1f32,

        quit: false,
    };

    application::main_loop(context, state)
}

/// # Errors
/// - Fail to initialize vertex buffer
fn initialize_vertices() -> Result<Buffer> {
    let vertices = vec![
        -0.5f32, -0.5f32, 1f32, 0f32, 0f32, 0.5f32, -0.5f32, 0f32, 1f32, 0f32, 0f32, 0.5f32, 0f32,
        0f32, 1f32,
    ];
    let mut vertex_buffer = Buffer::allocate(
        BufferUsage::Vertex,
        std::mem::size_of::<f32>() * vertices.len(),
    )?;
    let ptr = vertex_buffer.map::<f32>();
    ptr.copy_from_slice(&vertices);
    vertex_buffer.unmap();
    Ok(vertex_buffer)
}
