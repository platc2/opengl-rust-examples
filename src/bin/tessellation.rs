#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::must_use_candidate)]
extern crate alloc;
extern crate core;
extern crate gl;
extern crate sdl2;

use std::path::Path;

use gl::types::{GLintptr, GLsizei};

use hello_triangle_rust::renderer::{
    Buffer, BufferUsage, RenderPass, Shader, ShaderKind, VertexAttribute, VertexBinding,
};
use hello_triangle_rust::renderer_context::{OpenGLVersion, RendererContext, WindowDimension};
use hello_triangle_rust::resources::Resources;
use hello_triangle_rust::{imgui_wrapper, renderer};

#[derive(Default, Copy, Clone)]
struct TessellationParameters {
    outer: [u32; 4 * 4],
    inner: [u32; 2 * 4],
}

fn main() -> Result<(), String> {
    // Initialize render-context
    let context = RendererContext::init(
        "Tessellation",
        &WindowDimension::default(),
        &OpenGLVersion::default(),
    )
    .map_err(|e| format!("{e}"))?;

    let res = Resources::from_relative_exe_path(Path::new("assets/tessellation"))
        .map_err(|e| format!("{e}"))?;

    let mut tessellation_parameters = TessellationParameters {
        outer: [1; 4 * 4],
        inner: [1; 2 * 4],
    };

    let mut tessellation_parameters_buffer = Buffer::allocate(
        BufferUsage::Uniform,
        std::mem::size_of::<TessellationParameters>(),
    )
    .map_err(|e| format!("{e}"))?;

    let vertex_buffer = initialize_vertices()?;

    let vertex_shader = Shader::from_source(
        &res.load_string("/shaders/basic.vert")
            .map_err(|e| format!("{e}"))?,
        ShaderKind::Vertex,
    )
    .map_err(|e| format!("{e}"))?;
    let fragment_shader = Shader::from_source(
        &res.load_string("/shaders/basic.frag")
            .map_err(|e| format!("{e}"))?,
        ShaderKind::Fragment,
    )
    .map_err(|e| format!("{e}"))?;
    let tessellation_control_shader = Shader::from_source(
        &res.load_string("/shaders/basic.tesc")
            .map_err(|e| format!("{e}"))?,
        ShaderKind::TessellationControl,
    )
    .map_err(|e| format!("{e}"))?;
    let tessellation_evaluation_shader = Shader::from_source(
        &res.load_string("/shaders/basic.tese")
            .map_err(|e| format!("{e}"))?,
        ShaderKind::TessellationEvaluation,
    )
    .map_err(|e| format!("{e}"))?;

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

    let main_render_pass = RenderPass::new_tess(
        &vertex_shader,
        &fragment_shader,
        &tessellation_control_shader,
        &tessellation_evaluation_shader,
        &vertex_bindings,
        &[&tessellation_parameters_buffer],
        &[],
        &[],
    )
    .map_err(|e| format!("{e}"))?;

    let max_tessellation = std::cmp::min(gl::MAX_TESS_GEN_LEVEL, 64);

    let mut mouse_pos = (0, 0);
    let mut mouse_left = false;
    let mut mouse_right = false;

    let mut event_pump = context
        .sdl()
        .event_pump()
        .expect("Failed to get event pump");

    let mut chars: Vec<char> = Vec::new();

    let mut imgui_context = imgui_wrapper::Imgui::init();

    'main: loop {
        for event in event_pump.poll_iter() {
            use sdl2::event::Event;
            use sdl2::keyboard::Keycode;
            use sdl2::mouse::MouseButton;
            match event {
                Event::MouseMotion { x, y, .. } => {
                    mouse_pos = (
                        // This is ok - Mouse coordinates shouldn't reach numbers which overflow 16bit
                        i16::try_from(x).unwrap_or(0),
                        i16::try_from(y).unwrap_or(0),
                    )
                }
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Left,
                    ..
                } => mouse_left = true,
                Event::MouseButtonUp {
                    mouse_btn: MouseButton::Left,
                    ..
                } => mouse_left = false,
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Right,
                    ..
                } => mouse_right = true,
                Event::MouseButtonUp {
                    mouse_btn: MouseButton::Right,
                    ..
                } => mouse_right = false,
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main Ok(()),
                Event::KeyDown {
                    keycode: Some(key_code),
                    ..
                } => {
                    let key_code = key_code as u32;
                    if (32..512).contains(&key_code) {
                        chars.push(char::from_u32(key_code).unwrap());
                    }
                }
                _ => {}
            }
        }

        imgui_context.prepare(
            [900f32, 700f32],
            [mouse_pos.0.into(), mouse_pos.1.into()],
            [mouse_left, mouse_right],
            &mut chars,
        );

        unsafe {
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

            main_render_pass.display();

            let tessellation_parameters_ptr =
                tessellation_parameters_buffer.map::<TessellationParameters>();
            tessellation_parameters_ptr.copy_from_slice(&[tessellation_parameters]);
            tessellation_parameters_buffer.unmap();

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
            gl::PatchParameteri(gl::PATCH_VERTICES, 3);
            gl::DrawArrays(gl::PATCHES, 0, 3);
        }

        unsafe {
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
        }
        imgui_context.render(|ui| {
            ui.window("Settings")
                .save_settings(false)
                .always_auto_resize(true)
                .build(|| {
                    ui.text("Tessellation parameters");
                    ui.separator();
                    ui.slider(
                        "Outer 0",
                        1,
                        max_tessellation,
                        &mut tessellation_parameters.outer[0 * 4],
                    );
                    ui.slider(
                        "Outer 1",
                        1,
                        max_tessellation,
                        &mut tessellation_parameters.outer[1 * 4],
                    );
                    ui.slider(
                        "Outer 2",
                        1,
                        max_tessellation,
                        &mut tessellation_parameters.outer[2 * 4],
                    );
                    ui.slider(
                        "Outer 3",
                        1,
                        max_tessellation,
                        &mut tessellation_parameters.outer[3 * 4],
                    );
                    ui.separator();
                    ui.slider(
                        "Inner 0",
                        1,
                        max_tessellation,
                        &mut tessellation_parameters.inner[0 * 4],
                    );
                    ui.slider(
                        "Inner 1",
                        1,
                        max_tessellation,
                        &mut tessellation_parameters.inner[1 * 4],
                    );
                    ui.separator();
                    ui.separator();
                    let mut tessellation_value = 0;
                    tessellation_value += tessellation_parameters.outer[0 * 4];
                    tessellation_value += tessellation_parameters.outer[1 * 4];
                    tessellation_value += tessellation_parameters.outer[2 * 4];
                    tessellation_value += tessellation_parameters.outer[3 * 4];
                    tessellation_value += tessellation_parameters.inner[0 * 4];
                    tessellation_value += tessellation_parameters.inner[1 * 4];
                    tessellation_value /= 6;
                    if ui.slider("All", 1, max_tessellation, &mut tessellation_value) {
                        for i in 0..4 {
                            tessellation_parameters.outer[i * 4] = tessellation_value;
                        }
                        for i in 0..2 {
                            tessellation_parameters.inner[i * 4] = tessellation_value;
                        }
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
        -0.5f32, -0.5f32, 1f32, 0f32, 0f32, 0.5f32, -0.5f32, 0f32, 1f32, 0f32, 0f32, 0.5f32, 0f32,
        0f32, 1f32,
    ];
    let mut vertex_buffer = Buffer::allocate(
        BufferUsage::Vertex,
        std::mem::size_of::<f32>() * vertices.len(),
    )
    .map_err(|e| format!("{:?}", e))?;
    let ptr = vertex_buffer.map::<f32>();
    ptr.copy_from_slice(&vertices);
    vertex_buffer.unmap();
    Ok(vertex_buffer)
}
