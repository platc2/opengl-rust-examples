#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::must_use_candidate)]
extern crate alloc;
extern crate core;
extern crate gl_bindings as gl;
extern crate sdl2;

use std::path::Path;
use std::time::Duration;

use anyhow::{anyhow, Context, Result};
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

use imgui::{Condition, TextureId};
use renderer::{
    Program, Shader, ShaderKind, Texture
    ,
};
use renderer::imgui_wrapper;
use renderer::key_codes::KeyCodes;
use renderer::mouse_buttons::MouseButtons;
use renderer::renderer_context::{OpenGLVersion, RendererContext, WindowDimension};
use renderer::resources::Resources;

use crate::camera::Camera;

mod camera;

type Vec3 = nalgebra_glm::TVec3<f32>;
type Vec4 = nalgebra_glm::TVec4<f32>;

#[repr(packed)]
struct Sphere {
    centre: Vec3,
    radius: f32,
    color: Vec4,
}

impl Sphere {
    pub fn new(centre: Vec3, radius: f32, color: Vec4) -> Self {
        Self {
            centre,
            radius,
            color,
        }
    }
}

fn main() -> Result<()> {
    // Initialize render-context
    let context = RendererContext::init(
        "Compute Shader",
        &WindowDimension::default(),
        &OpenGLVersion::default(),
    )?;

    let res = Resources::from_relative_exe_path(Path::new("assets"))?;

    let compute_shader = res
        .load_string("/shaders/main.comp")
        .map_err(Into::into)
        .and_then(|source| Shader::from_source(&source, ShaderKind::Compute))
        .context("Failed to initialize compute shader")?;
    let compute_program = Program::from_shaders(&[&compute_shader])?;

    use nalgebra_glm::{vec3, vec4};
    let spheres = vec![
        Sphere::new(vec3(0., 0., -10.), 1., vec4(1., 0., 0., 1.)),
        Sphere::new(vec3(5., 0., -10.), 1., vec4(0., 1., 0., 1.)),
        Sphere::new(vec3(-5., 0., -50.), 10., vec4(0., 0., 1., 1.)),
    ];

    compute_program.set_used();

    let mut ssbo: gl::types::GLuint = 0;
    unsafe {
        gl::CreateBuffers(1, &mut ssbo);
        gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, ssbo);
        gl::BufferData(
            gl::SHADER_STORAGE_BUFFER,
            (std::mem::size_of::<Sphere>() * spheres.len() * std::mem::size_of::<f32>())
                as gl::types::GLsizeiptr,
            spheres.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );
        gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, 5, ssbo);
    }

    let mut texture = Texture::blank(512, 512);
    unsafe {
        gl::BindImageTexture(
            0,
            texture.handle(),
            0,
            gl::FALSE,
            0,
            gl::READ_ONLY,
            gl::RGBA32F,
        );
    }

    let mut mouse_buttons = MouseButtons::default();
    let mut key_codes = KeyCodes::default();
    let mut mouse_pos = (0, 0);

    let mut event_pump = context.sdl().event_pump().map_err(|e| anyhow!(e))?;

    let mut chars: Vec<char> = Vec::new();

    let mut imgui_context = imgui_wrapper::Imgui::init();

    let mut camera = Camera::default();

    let mut t = 0;
    let mut frame_times = vec![60.];
    let target_fps = 60;
    let target_frame_time_ns = 1_000_000_000u32 / target_fps;
    'main: loop {
        let start_time = std::time::Instant::now();
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

        let speed = 0.05f32;
        if key_codes[Keycode::W] {
            camera.move_forward(speed);
        }
        if key_codes[Keycode::S] {
            camera.move_forward(-speed);
        }
        if key_codes[Keycode::D] {
            camera.move_right(speed);
        }
        if key_codes[Keycode::A] {
            camera.move_right(-speed);
        }
        if key_codes[Keycode::Space] {
            camera.move_up(speed);
        }
        if key_codes[Keycode::LCtrl] {
            camera.move_up(-speed);
        }
        if key_codes[Keycode::Up] {
            camera.look_up(speed);
        }
        if key_codes[Keycode::Down] {
            camera.look_up(-speed);
        }
        if key_codes[Keycode::Right] {
            camera.look_right(speed);
        }
        if key_codes[Keycode::Left] {
            camera.look_right(-speed);
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
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::Viewport(0, 0, 900, 700);

            compute_program.set_used();

            gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, 5, ssbo);

            t += 1;
            unsafe {
                gl::Uniform3fv(0, 1, camera.position().as_ptr());
                gl::Uniform3fv(1, 1, camera.forward().as_ptr());
                gl::Uniform3fv(2, 1, camera.up().as_ptr());
            }

            unsafe {
                gl::DispatchCompute(512, 512, 1);
                gl::MemoryBarrier(gl::SHADER_IMAGE_ACCESS_BARRIER_BIT);
            }
        }

        let fps = frame_times.iter().sum::<f32>() / (frame_times.len() as f32);
        imgui_context.render(|ui| {
            ui.window("Result")
                .save_settings(false)
                .resizable(false)
                .position([(900. - 512.) / 2., (700. - 512.) / 2.], Condition::Once)
                .no_decoration()
                .movable(false)
                .bring_to_front_on_focus(false)
                .always_use_window_padding(false)
                .focused(false)
                .build(|| {
                    imgui::Image::new(TextureId::from(texture.handle() as usize), [512f32, 512f32])
                        .build(ui);
                });
            ui.window("Main")
                .save_settings(false)
                .always_auto_resize(true)
                .build(|| {
                    ui.text(format!("FPS: {}", fps));
                    ui.plot_lines("Frame time", &frame_times[..]).build();
                });
        });

        context.window().gl_swap_window();

        let frame_time = std::time::Instant::now()
            .duration_since(start_time)
            .as_nanos() as u32;
        let time_left = target_frame_time_ns.checked_sub(frame_time).unwrap_or(0);
        std::thread::sleep(Duration::new(0, time_left));

        let fps_current = 1.
            / std::time::Instant::now()
            .duration_since(start_time)
            .as_secs_f32();
        if frame_times.len() >= 1024 {
            frame_times.remove(0);
        }
        frame_times.push(fps_current);
    }
}
