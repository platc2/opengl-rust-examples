#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::must_use_candidate)]
extern crate alloc;
extern crate core;
extern crate gl;
extern crate sdl2;

use std::path::Path;

use gl::types::{GLintptr, GLsizei};

use hello_triangle_rust::{imgui_wrapper, renderer};
use hello_triangle_rust::renderer::{Buffer, BufferUsage, RenderPass, Shader, ShaderKind, VertexAttribute, VertexBinding};
use hello_triangle_rust::renderer_context::{OpenGLVersion, RendererContext, WindowDimension};
use hello_triangle_rust::resources::Resources;

fn main() -> Result<(), String> {
    // Initialize render-context
    let context = RendererContext::init("Hello Triangle", WindowDimension::default(), OpenGLVersion::default())
        .map_err(|e| format!("{e}"))?;

    let res = Resources::from_relative_exe_path(Path::new("../../assets/hello_triangle"))
        .map_err(|e| format!("{e}"))?;

    let vertex_buffer = initialize_vertices()
        .map_err(|e| format!("{e}"))?;

    let vertex_shader = Shader::from_source(
        &res.load_string("/shaders/triangle.vert").map_err(|e| format!("{e}"))?,
        ShaderKind::Vertex)
        .map_err(|e| format!("{e}"))?;
    let fragment_shader = Shader::from_source(
        &res.load_string("/shaders/triangle.frag").map_err(|e| format!("{e}"))?,
        ShaderKind::Fragment)
        .map_err(|e| format!("{e}"))?;

    let mut gamma_buffer = Buffer::allocate(BufferUsage::Uniform, std::mem::size_of::<f32>())
        .map_err(|e| format!("{e}"))?;

    let vertex_bindings = [
        VertexBinding::new(0, VertexAttribute::new(renderer::VertexAttributeFormat::RG32F, 0)),
        VertexBinding::new(1, VertexAttribute::new(renderer::VertexAttributeFormat::RGB32F, 0)),
    ];

    let main_render_pass = RenderPass::new(&vertex_shader, &fragment_shader, &vertex_bindings,
                                           &[&gamma_buffer], &[], &[])
        .map_err(|e| format!("{e}"))?;

    let mut mouse_pos = (0, 0);
    let mut mouse_left = false;
    let mut mouse_right = false;

    let mut event_pump = context.sdl().event_pump().expect("Failed to get event pump");

    let mut chars: Vec<char> = Vec::new();
    let mut gamma = 1f32;

    let mut imgui_context = imgui_wrapper::Imgui::init();

    'main: loop {
        for event in event_pump.poll_iter() {
            use sdl2::event::Event;
            use sdl2::mouse::MouseButton;
            use sdl2::keyboard::Keycode;
            match event {
                Event::MouseMotion { x, y, .. } => mouse_pos = (x, y),
                Event::MouseButtonDown { mouse_btn: MouseButton::Left, .. } => mouse_left = true,
                Event::MouseButtonUp { mouse_btn: MouseButton::Left, .. } => mouse_left = false,
                Event::MouseButtonDown { mouse_btn: MouseButton::Right, .. } => mouse_right = true,
                Event::MouseButtonUp { mouse_btn: MouseButton::Right, .. } => mouse_right = false,
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } =>
                    break 'main Ok(()),
                Event::KeyDown { keycode: Some(key_code), .. } => {
                    let key_code = key_code as u32;
                    if 32 <= key_code && key_code < 512 { chars.push(char::from_u32(key_code).unwrap()); }
                }
                _ => {}
            }
        }

        imgui_context.prepare(900f32, 700f32, mouse_pos.0 as f32, mouse_pos.1 as f32, mouse_left, mouse_right, &mut chars);

        unsafe {
            main_render_pass.display();

            let gamma_ptr = gamma_buffer.map::<f32>();
            gamma_ptr.copy_from_slice(&[gamma]);
            gamma_buffer.unmap();

            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::Viewport(0, 0, 900, 700);
            gl::BindVertexBuffer(0, vertex_buffer.handle(), 0 as GLintptr,
                                 GLsizei::try_from(std::mem::size_of::<f32>() * 5).unwrap());
            gl::BindVertexBuffer(1, vertex_buffer.handle(),
                                 GLintptr::try_from(std::mem::size_of::<f32>() * 2).unwrap(),
                                 GLsizei::try_from(std::mem::size_of::<f32>() * 5).unwrap());
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        imgui_context.render(|ui| {
            imgui::Window::new("Settings")
                .save_settings(false)
                .always_auto_resize(true)
                .build(ui, || {
                    imgui::Slider::new("Gamma", 0.5f32, 2.5f32)
                        .build(ui, &mut gamma);
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
fn initialize_vertices() -> Result<Buffer, String> {
    let vertices = vec![
        -0.5f32, -0.5f32, 1f32, 0f32, 0f32,
        0.5f32, -0.5f32, 0f32, 1f32, 0f32,
        0f32, 0.5f32, 0f32, 0f32, 1f32,
    ];
    let mut vertex_buffer = Buffer::allocate(BufferUsage::Vertex, std::mem::size_of::<f32>() * vertices.len())
        .map_err(|e| format!("{:?}", e))?;
    let ptr = vertex_buffer.map::<f32>();
    ptr.copy_from_slice(&vertices);
    vertex_buffer.unmap();
    Ok(vertex_buffer)
}
