#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::must_use_candidate)]

use std::collections::VecDeque;
use std::path::Path;

use anyhow::{Context, Result};
use gl::types::{GLintptr, GLsizei};
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

use hello_triangle_rust::{imgui_wrapper, renderer};
use hello_triangle_rust::key_codes::KeyCodes;
use hello_triangle_rust::mouse_buttons::MouseButtons;
use hello_triangle_rust::renderer::{Buffer, BufferUsage, RenderPass, Shader, ShaderKind, VertexAttribute, VertexBinding};
use hello_triangle_rust::renderer_context::{OpenGLVersion, RendererContext, WindowDimension};
use hello_triangle_rust::resources::Resources;

use crate::camera::Camera;
use crate::matrix_uniform::MatrixUniform;
use crate::planet::Planet;
use crate::time::Time;

mod matrix_uniform;
mod camera;
mod planet;
mod icosahedron;
mod polyhedron;
mod transform;
mod time;

pub fn main() -> Result<()> {
    let mut time = Time::new();

    let window_dimension = WindowDimension::default();
    // Initialize render-context
    let context = RendererContext::init(
        "Planet Renderer",
        &window_dimension,
        &OpenGLVersion::default(),
    )?;

    let res = Resources::from_relative_exe_path(Path::new("assets/planet_renderer"))?;

    let mut matrix_uniform_buffer =
        Buffer::allocate(BufferUsage::Uniform, std::mem::size_of::<MatrixUniform>())?;
    let mut matrix_uniforms = MatrixUniform::default();

    matrix_uniforms.model = nalgebra_glm::TMat4::identity();
    matrix_uniforms.projection = nalgebra_glm::perspective(
        window_dimension.width as f32 / window_dimension.height as f32,
        60f32.to_radians(),
        0.01f32,
        100f32,
    );

    let mut planet_mesh = Planet::new()
        .context("Failed to initialize planet mesh")?;

    let planet_vertex_shader = res
        .load_string("/shaders/planet.vert")
        .map_err(Into::into)
        .and_then(|source| Shader::from_source(&source, ShaderKind::Vertex))
        .context("Failed to initialize terrain vertex shader")?;
    let planet_fragment_shader = res
        .load_string("/shaders/planet.frag")
        .map_err(Into::into)
        .and_then(|source| Shader::from_source(&source, ShaderKind::Fragment))
        .context("Failed to initialize terrain fragment shader")?;

    let vertex_bindings = [VertexBinding::new(
        0,
        VertexAttribute::new(renderer::VertexAttributeFormat::RGB32F, 0),
    )];

    let main_render_pass = RenderPass::new(
        &planet_vertex_shader,
        &planet_fragment_shader,
        &vertex_bindings,
        &[&matrix_uniform_buffer],
        &[],
        &[],
    )?;

    let mut camera = Camera::default();
    camera.move_up(1f32);

    let mut mouse_buttons = MouseButtons::default();
    let mut key_codes = KeyCodes::default();
    let mut mouse_pos = (0, 0);

    let mut event_pump = context
        .sdl()
        .event_pump()
        .expect("Failed to get event pump");

    let mut chars: Vec<char> = Vec::new();

    let mut imgui_context = imgui_wrapper::Imgui::init();
    let mut wireframe = false;
    let mut old_level = 0;
    let mut max_level = 0;
    let mut freeze_camera = false;
    let mut last_freeze_state = false;
    let mut camera_pos: nalgebra_glm::Vec3 = camera.position();
    let mut camera_forward: nalgebra_glm::Vec3 = camera.forward();

    let fps_values = 1000;
    let mut fps = VecDeque::new();
    'main: loop {
        time.update();
        for event in event_pump.poll_iter() {
            use sdl2::event::Event;
            use sdl2::keyboard::Keycode;
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
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main Ok(()),
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
                _ => {}
            }
        }

        if last_freeze_state != freeze_camera {
            last_freeze_state = freeze_camera;
        }

        if !freeze_camera {
            camera_pos = camera.position();
            camera_forward = camera.forward();
        }

        let mut moved = false;
        let speed = time.duration().as_secs_f32();
        if key_codes[Keycode::W] {
            camera.move_forward(speed);
            moved = true;
        }
        if key_codes[Keycode::S] {
            camera.move_forward(-speed);
            moved = true;
        }
        if key_codes[Keycode::D] {
            camera.move_right(speed);
            moved = true;
        }
        if key_codes[Keycode::A] {
            camera.move_right(-speed);
            moved = true;
        }
        if key_codes[Keycode::Space] {
            camera.move_up(speed);
            moved = true;
        }
        if key_codes[Keycode::LCtrl] {
            camera.move_up(-speed);
            moved = true;
        }
        if key_codes[Keycode::Up] {
            camera.look_up(speed);
            moved = true;
        }
        if key_codes[Keycode::Down] {
            camera.look_up(-speed);
            moved = true;
        }
        if key_codes[Keycode::Right] {
            camera.look_right(speed);
            moved = true;
        }
        if key_codes[Keycode::Left] {
            camera.look_right(-speed);
            moved = true;
        }

        matrix_uniforms.view = camera.view_matrix();

        let matrix_uniforms_ptr = matrix_uniform_buffer.map::<MatrixUniform>();
        matrix_uniforms_ptr.copy_from_slice(&[matrix_uniforms]);
        matrix_uniform_buffer.unmap();

        imgui_context.prepare(
            [
                window_dimension.width as f32,
                window_dimension.height as f32,
            ],
            [mouse_pos.0.into(), mouse_pos.1.into()],
            [
                mouse_buttons[MouseButton::Left],
                mouse_buttons[MouseButton::Right],
            ],
            &mut chars,
        );

        unsafe {
            gl::PolygonMode(
                gl::FRONT_AND_BACK,
                if wireframe { gl::LINE } else { gl::FILL },
            );

            main_render_pass.display();

            let distance = nalgebra_glm::length(&camera_pos);
            if (old_level != max_level || moved) {
                planet_mesh.recalculate(max_level, &camera_pos, Some(&camera_forward));
                old_level = max_level;
            }

            gl::Disable(gl::CULL_FACE);
            gl::Enable(gl::DEPTH_TEST);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::Viewport(0, 0, 900, 700);

            gl::BindVertexBuffer(
                0,
                planet_mesh.vertex_buffer.handle(),
                0 as GLintptr,
                GLsizei::try_from(std::mem::size_of::<f32>() * 3).unwrap(),
            );

            gl::DrawArrays(
                gl::TRIANGLES,
                0,
                planet_mesh.size as GLsizei,
            );

            gl::BindVertexBuffer(0, 0, 0, 0);
        }

        let current_fps = time.fps();
        if fps.len() >= fps_values {
            fps.pop_back();
        }
        fps.push_front(current_fps);
        imgui_context.render(|ui| {
            ui.window("Settings")
                .save_settings(false)
                .always_auto_resize(true)
                .build(|| {
                    ui.checkbox("Wireframe", &mut wireframe);
                    ui.slider("Max Level", 0, 10, &mut max_level);
                    ui.checkbox("Freeze camera", &mut freeze_camera);
                    ui.plot_lines(format!("FPS: {}", current_fps), fps.make_contiguous())
                        .scale_min(0.)
                        .scale_max(200.)
                        .build();
                    ui.columns(2, "col", false);
                    ui.text("Vertices");
                    ui.next_column();
                    ui.text_colored([1., 0.5, 0.5, 1.], format!("{}", planet_mesh.size));
                });
        });

        context.window().gl_swap_window();
    }
}