#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::must_use_candidate)]
extern crate alloc;
extern crate core;
extern crate gl_bindings as gl;
extern crate renderer;
extern crate sdl2;

use std::path::Path;

use anyhow::{anyhow, Context, Result};
use sdl2::mouse::MouseButton;

use gl::types::{GLintptr, GLsizei};
use renderer::{
    Buffer, BufferUsage, RenderPass, Shader, ShaderKind, VertexAttribute, VertexBinding,
};
use renderer::imgui_wrapper;
use renderer::key_codes::KeyCodes;
use renderer::mouse_buttons::MouseButtons;
use renderer::renderer_context::{OpenGLVersion, RendererContext, WindowDimension};
use renderer::resources::Resources;

fn main() -> Result<()> {
    // Initialize render-context
    let context = RendererContext::init(
        "Hello Triangle",
        &WindowDimension::default(),
        &OpenGLVersion::default(),
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

    let mut event_pump = context.sdl().event_pump().map_err(|e| anyhow!(e))?;

    let mut chars: Vec<char> = Vec::new();
    let mut gamma = 1f32;

    let mut imgui_context = imgui_wrapper::Imgui::init();

    'main: loop {
        for event in event_pump.poll_iter() {
            use sdl2::event::Event;
            match event {
                Event::MouseMotion { x, y, .. } => {
                    mouse_pos = (
                        // This is ok - Mouse coordinates shouldn't reach numbers which overflow 16bit
                        i16::try_from(x).unwrap_or(0),
                        i16::try_from(y).unwrap_or(0),
                    );
                }
                Event::MouseButtonDown { mouse_btn, .. } => mouse_buttons[mouse_btn] = true,
                Event::MouseButtonUp { mouse_btn, .. } => mouse_buttons[mouse_btn] = false,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    key_codes[keycode] = true;

                    let keycode = keycode as u32;
                    if (32..512).contains(&keycode) {
                        chars.push(char::from_u32(keycode).unwrap());
                    }
                }
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => key_codes[keycode] = false,
                Event::Quit { .. } => break 'main Ok(()),
                _ => {}
            }
        }

        imgui_context.prepare(
            [900f32, 700f32],
            [mouse_pos.0.into(), mouse_pos.1.into()],
            [
                mouse_buttons[MouseButton::Left],
                mouse_buttons[MouseButton::Right],
            ],
            &mut chars,
        );

        unsafe {
            main_render_pass.display();

            let gamma_ptr = gamma_buffer.map::<f32>();
            gamma_ptr.copy_from_slice(&[gamma]);
            gamma_buffer.unmap();

            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::Viewport(0, 0, 900, 700);
            gl::BindVertexBuffer(
                0,
                vertex_buffer.handle(),
                0 as GLintptr,
                GLsizei::try_from(std::mem::size_of::<f32>() * 5).unwrap(),
            );
            gl::BindVertexBuffer(
                1,
                vertex_buffer.handle(),
                GLintptr::try_from(std::mem::size_of::<f32>() * 2).unwrap(),
                GLsizei::try_from(std::mem::size_of::<f32>() * 5).unwrap(),
            );
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        imgui_context.render(|ui| {
            ui.window("Settings")
                .save_settings(false)
                .always_auto_resize(true)
                .build(|| {
                    ui.slider("Gamma", 0.5f32, 2.5f32, &mut gamma);
                    if ui.button("Reset (1.0)") {
                        gamma = 1f32;
                    }
                    ui.same_line();
                    if ui.button("Reset (2.2)") {
                        gamma = 2.2f32;
                    }
                });
        });

        context.window().gl_swap_window();
    }
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
