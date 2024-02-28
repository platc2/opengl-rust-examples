#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::must_use_candidate)]
extern crate alloc;
extern crate core;
extern crate gl_bindings as gl;
extern crate sdl2;

use std::path::Path;

use anyhow::{Context, Result};

use renderer::{application, Program, Shader, ShaderKind, Texture};
use renderer::renderer_context::{OpenGLVersion, RendererContext, WindowDimension};
use renderer::resources::Resources;

use crate::state::State;

mod camera;
mod state;

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

    let mut ssbo: gl::sys::types::GLuint = 0;
    unsafe {
        gl::sys::CreateBuffers(1, &mut ssbo);
        gl::sys::BindBuffer(gl::sys::SHADER_STORAGE_BUFFER, ssbo);
        gl::sys::BufferData(
            gl::sys::SHADER_STORAGE_BUFFER,
            (std::mem::size_of::<Sphere>() * spheres.len() * std::mem::size_of::<f32>())
                as gl::sys::types::GLsizeiptr,
            spheres.as_ptr() as *const gl::sys::types::GLvoid,
            gl::sys::STATIC_DRAW,
        );
        gl::sys::BindBufferBase(gl::sys::SHADER_STORAGE_BUFFER, 5, ssbo);
    }

    let mut texture = Texture::blank(512, 512);
    unsafe {
        gl::sys::BindImageTexture(
            0,
            texture.handle(),
            0,
            gl::sys::FALSE,
            0,
            gl::sys::READ_ONLY,
            gl::sys::RGBA32F,
        );
    }

    /*
    let mut t = 0;
    let mut frame_times = vec![60.];
    let target_fps = 60;
    let target_frame_time_ns = 1_000_000_000u32 / target_fps;

        let fps = frame_times.iter().sum::<f32>() / (frame_times.len() as f32);

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
     */
    let state = State::new(
        compute_program,
        ssbo,
        texture,
    );

    application::main_loop(context, state)
}
