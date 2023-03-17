#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::must_use_candidate)]
extern crate alloc;
extern crate core;
extern crate gl;
extern crate sdl2;

use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result};
use gl::types::{GLintptr, GLsizei};
use noise::{Billow, MultiFractal, NoiseFn, Perlin, RidgedMulti, ScaleBias, Select};
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

use hello_triangle_rust::key_codes::KeyCodes;
use hello_triangle_rust::mouse_buttons::MouseButtons;
use hello_triangle_rust::renderer::{
    Buffer, BufferUsage, RenderPass, Shader, ShaderKind, Texture, VertexAttribute, VertexBinding,
};
use hello_triangle_rust::renderer_context::{OpenGLVersion, RendererContext, WindowDimension};
use hello_triangle_rust::resources::Resources;
use hello_triangle_rust::{imgui_wrapper, renderer};

use crate::camera::Camera;

type Mat4 = nalgebra_glm::TMat4<f32>;

#[derive(Default, Copy, Clone)]
struct MatrixUniform {
    model: Mat4,
    view: Mat4,
    projection: Mat4,
}

const TERRAIN_TEXTURE_SIZE: usize = 256;
const TERRAIN_MESH_SIZE: usize = 128;

fn main() -> Result<()> {
    let window_dimension = WindowDimension::default();
    // Initialize render-context
    let context = RendererContext::init(
        "Bruteforce Terrain",
        &window_dimension,
        &OpenGLVersion::default(),
    )?;

    let res = Resources::from_relative_exe_path(Path::new("../../assets/bruteforce_terrain"))?;

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

    let terrain_texture = initialize_terrain(TERRAIN_TEXTURE_SIZE, TERRAIN_TEXTURE_SIZE)?;
    let vertex_buffer = initialize_terrain_vertices(TERRAIN_MESH_SIZE, TERRAIN_MESH_SIZE)?;
    let index_buffer = initialize_terrain_indices(TERRAIN_MESH_SIZE, TERRAIN_MESH_SIZE)?;

    let terrain_vertex_shader = res
        .load_string("/shaders/terrain.vert")
        .map_err(Into::into)
        .and_then(|source| Shader::from_source(&source, ShaderKind::Vertex))
        .context("Failed to initialize terrain vertex shader")?;
    let terrain_fragment_shader = res
        .load_string("/shaders/terrain.frag")
        .map_err(Into::into)
        .and_then(|source| Shader::from_source(&source, ShaderKind::Fragment))
        .context("Failed to initialize terrain fragment shader")?;

    let vertex_bindings = [VertexBinding::new(
        0,
        VertexAttribute::new(renderer::VertexAttributeFormat::RGB32F, 0),
    )];

    let main_render_pass = RenderPass::new(
        &terrain_vertex_shader,
        &terrain_fragment_shader,
        &vertex_bindings,
        &[&matrix_uniform_buffer],
        &[&terrain_texture],
        &[],
    )?;

    let mut camera = Camera::default();

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

    'main: loop {
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

        let speed = 0.005f32;
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

        let time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();
        let angle = ((time % (90 * 360)) as f32).to_radians() / 90f32;
        let position = nalgebra_glm::vec3(angle.cos() * 2f32, 1f32, angle.sin() * 2f32);
        matrix_uniforms.view = nalgebra_glm::look_at(
            &position,
            &nalgebra_glm::vec3(0f32, 0f32, 0f32),
            &nalgebra_glm::vec3(0f32, 1f32, 0f32),
        );
        matrix_uniforms.view = camera.view_matrix();

        let matrix_uniforms_ptr = matrix_uniform_buffer.map::<MatrixUniform>();
        matrix_uniforms_ptr.copy_from_slice(&[matrix_uniforms]);
        matrix_uniform_buffer.unmap();

        unsafe {
            gl::PolygonMode(
                gl::FRONT_AND_BACK,
                if wireframe { gl::LINE } else { gl::FILL },
            );
            main_render_pass.display();

            gl::Disable(gl::CULL_FACE);
            gl::Enable(gl::DEPTH_TEST);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::Viewport(0, 0, 900, 700);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, index_buffer.handle());
            gl::BindVertexBuffer(
                0,
                vertex_buffer.handle(),
                0 as GLintptr,
                GLsizei::try_from(std::mem::size_of::<f32>() * 3).unwrap(),
            );
            let count = index_buffer.size() / std::mem::size_of::<u16>();
            gl::DrawElements(
                gl::TRIANGLE_STRIP,
                count as GLsizei,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
        }

        imgui_context.render(|ui| {
            ui.window("Settings")
                .save_settings(false)
                .always_auto_resize(true)
                .build(|| {
                    ui.checkbox("Wireframe", &mut wireframe);
                });
        });

        context.window().gl_swap_window();
    }
}

fn initialize_terrain(width: usize, height: usize) -> Result<Texture> {
    let mut image_data = Vec::<u8>::with_capacity(width * height * 4);

    let base_mountain_terrain = RidgedMulti::<Perlin>::default()
        .set_frequency(2.5)
        .set_attenuation(1.2)
        .set_persistence(0.5)
        .set_lacunarity(1.8);
    let mountain_terrain = ScaleBias::new(base_mountain_terrain)
        .set_scale(1.0)
        .set_bias(0.5);
    let base_flat_terrain = Billow::<Perlin>::default().set_frequency(2.0);
    let flat_terrain = ScaleBias::new(base_flat_terrain)
        .set_scale(0.125)
        .set_bias(0.25);
    let noise = Select::new(mountain_terrain, flat_terrain, Perlin::default()).set_falloff(0.8);

    for y in 0..height {
        for x in 0..width {
            let x = x as f64 / width as f64;
            let y = y as f64 / height as f64;
            let height = (1.0 + noise.get([x, y])) / 2.0;
            image_data.push((height * 256.0) as u8);
            image_data.push(0);
            image_data.push(0);
            image_data.push(0);
        }
    }

    Ok(Texture::from_raw(image_data.as_slice(), width, height)?)
}

fn initialize_terrain_vertices(width: usize, height: usize) -> Result<Buffer> {
    let num_vertices = width * height * 3;
    let mut vertices = vec![0f32; num_vertices];

    for y in 0..height {
        for x in 0..width {
            let index = ((y * width) + x) * 3;
            #[allow(clippy::cast_precision_loss)]
            let pos_x = (x as f32 * 2f32) / (width as f32) - 1f32;
            #[allow(clippy::cast_precision_loss)]
            let pos_z = (y as f32 * 2f32) / (height as f32) - 1f32;
            vertices[index] = pos_x;
            vertices[index + 1] = 0f32;
            vertices[index + 2] = pos_z;
        }
    }

    let mut vertex_buffer = Buffer::allocate(
        BufferUsage::Vertex,
        std::mem::size_of::<f32>() * vertices.len(),
    )?;
    let ptr = vertex_buffer.map::<f32>();
    ptr.copy_from_slice(&vertices);
    vertex_buffer.unmap();
    Ok(vertex_buffer)
}

fn initialize_terrain_indices(width: usize, height: usize) -> Result<Buffer> {
    let width = u32::try_from(width)?;
    let height = u32::try_from(height)?;
    let mut indices = vec![0u32; 0];

    for y in 0..(height - 1) {
        for x in 0..width {
            let row_index = (y * width) + x;
            let next_row_index = ((y + 1) * width) + x;
            if x == 0 {
                indices.push(row_index);
            };
            indices.push(row_index);
            indices.push(next_row_index);
            if x == (width - 1) {
                indices.push(next_row_index);
            };
        }
    }

    let mut index_buffer = Buffer::allocate(
        BufferUsage::Index,
        std::mem::size_of::<u32>() * indices.len(),
    )?;
    let ptr = index_buffer.map();
    ptr.copy_from_slice(&indices);
    index_buffer.unmap();
    Ok(index_buffer)
}

mod camera {
    use std::cmp::{max, min};

    #[derive(Default)]
    pub struct Camera {
        position: nalgebra_glm::Vec3,
        yaw: f32,
        pitch: f32,
    }

    impl Camera {
        pub fn move_forward(&mut self, units: f32) {
            self.position += self.forward() * units;
        }

        pub fn move_right(&mut self, units: f32) {
            self.position += self.right() * units;
        }

        pub fn move_up(&mut self, units: f32) {
            self.position += self.up() * units;
        }

        pub fn look_up(&mut self, angle: f32) {
            self.pitch += angle;
            if self.pitch > 180f32 {
                self.pitch = 180f32;
            }
            if self.pitch < -180f32 {
                self.pitch = -180f32;
            }
        }

        pub fn look_right(&mut self, angle: f32) {
            self.yaw += angle;
        }

        pub fn view_matrix(&self) -> nalgebra_glm::Mat4 {
            nalgebra_glm::look_at(
                &self.position,
                &(self.position + self.forward()),
                &self.up(),
            )
        }

        fn forward(&self) -> nalgebra_glm::Vec3 {
            self.rotated(&nalgebra_glm::vec3(0f32, 0f32, -1f32))
        }

        fn backward(&self) -> nalgebra_glm::Vec3 {
            -self.forward()
        }

        fn right(&self) -> nalgebra_glm::Vec3 {
            self.rotated(&nalgebra_glm::vec3(1f32, 0f32, 0f32))
        }

        fn left(&self) -> nalgebra_glm::Vec3 {
            -self.right()
        }

        fn up(&self) -> nalgebra_glm::Vec3 {
            self.rotated(&nalgebra_glm::vec3(0f32, 1f32, 0f32))
        }

        fn down(&self) -> nalgebra_glm::Vec3 {
            -self.up()
        }

        fn rotated(&self, vector: &nalgebra_glm::Vec3) -> nalgebra_glm::Vec3 {
            nalgebra_glm::rotate_vec3(
                &nalgebra_glm::rotate_vec3(
                    &vector,
                    self.pitch,
                    &nalgebra_glm::vec3(1f32, 0f32, 0f32),
                ),
                -self.yaw,
                &nalgebra_glm::vec3(0f32, 1f32, 0f32),
            )
        }
    }
}
