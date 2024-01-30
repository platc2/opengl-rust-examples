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
use renderer::key_codes::KeyCodes;
use renderer::mouse_buttons::MouseButtons;
use renderer::renderer_context::{OpenGLVersion, RendererContext, WindowDimension};
use renderer::resources::Resources;
use renderer::time::Time;

struct State {
    mouse_pos: (i16, i16),
    mouse_buttons: MouseButtons,
    key_codes: KeyCodes,
    chars: Vec<char>,
    main_render_pass: RenderPass,
    gamma_buffer: Buffer,
    vertex_buffer: Buffer,
    gamma: f32,
    string: String,

    quit: bool,
//    count: u32,
//    updates: u32,
//    lag: Duration,
}

impl Application for State {
    fn tick(&mut self, _: &Time<Instant>, input_manager: &dyn InputManager) {
        /*
                for event in self.event_pump.poll_iter() {
                    use sdl2::event::Event;
                    match event {
                        Event::MouseMotion { x, y, .. } => {
                            self.mouse_pos = (
                                // This is ok - Mouse coordinates shouldn't reach numbers which overflow 16bit
                                i16::try_from(x).unwrap_or(0),
                                i16::try_from(y).unwrap_or(0),
                            );
                        }
                        Event::MouseButtonDown { mouse_btn, .. } => self.mouse_buttons[mouse_btn] = true,
                        Event::MouseButtonUp { mouse_btn, .. } => self.mouse_buttons[mouse_btn] = false,
                        Event::KeyDown {
                            keycode: Some(keycode),
                            ..
                        } => {
                            self.key_codes[keycode] = true;

                            let keycode = keycode as u32;
                            if (32..512).contains(&keycode) {
                                self.chars.push(char::from_u32(keycode).unwrap());
                            }
                        }
                        Event::KeyUp {
                            keycode: Some(keycode),
                            ..
                        } => self.key_codes[keycode] = false,
                        Event::Quit { .. } => self.quit = true,
                        _ => {}
                    }
                }
        */

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
                ui.input_float("Float", &mut self.gamma)
                    .build();
                ui.input_text("Text", &mut self.string)
                    .build();
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

    let mut gamma_buffer = Buffer::allocate(BufferUsage::Uniform, std::mem::size_of::<f32>())?;

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

    let mut mouse_buttons = MouseButtons::default();
    let mut key_codes = KeyCodes::default();
    let mut mouse_pos = (0, 0);

    let mut chars: Vec<char> = Vec::new();
    let mut gamma = 1f32;

    let mut state = State {
        mouse_pos,
        mouse_buttons,
        key_codes,
        chars,
        main_render_pass,
        gamma_buffer,
        vertex_buffer,
        gamma,
        string: String::from("Hello, World!"),

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
