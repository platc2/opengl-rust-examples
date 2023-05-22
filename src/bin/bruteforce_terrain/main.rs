#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::must_use_candidate)]
extern crate core;
extern crate gl;
extern crate sdl2;

use std::collections::HashSet;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result};
use gl::types::{GLintptr, GLsizei};
use imgui::TextureId;
use noise::{
    Billow, Cache, MultiFractal, NoiseFn, OpenSimplex, Perlin, RidgedMulti, ScaleBias, Select,
    Simplex,
};
use sdl2::keyboard::Keycode;
use sdl2::libc::exit;
use sdl2::mouse::MouseButton;

use hello_triangle_rust::{imgui_wrapper, renderer};
use hello_triangle_rust::key_codes::KeyCodes;
use hello_triangle_rust::mouse_buttons::MouseButtons;
use hello_triangle_rust::renderer::{
    Buffer, BufferUsage, RenderPass, Shader, ShaderKind, Texture, VertexAttribute, VertexBinding,
};
use hello_triangle_rust::renderer_context::{OpenGLVersion, RendererContext, WindowDimension};
use hello_triangle_rust::resources::Resources;

use crate::camera::Camera;
use crate::chunk::{Chunk, ChunkPosition};
use crate::matrix_uniform::MatrixUniform;
use crate::terrain_mesh::TerrainMesh;

mod chunk;
mod terrain_mesh;
mod matrix_uniform;
mod camera;

const TERRAIN_SIZE: usize = 32;
const CHUNK_LOAD_AREA: (usize, usize) = (5, 5);

fn main() -> Result<()> {
    let window_dimension = WindowDimension::default();
    // Initialize render-context
    let context = RendererContext::init(
        "Bruteforce Terrain",
        &window_dimension,
        &OpenGLVersion::default(),
    )?;

    let res = Resources::from_relative_exe_path(Path::new("assets/bruteforce_terrain"))?;

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

    let terrain_texture = Texture::blank(0, 0);
    let terrain_mesh = TerrainMesh::of_size(TERRAIN_SIZE, TERRAIN_SIZE)?;

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
    let terrain_geometry_shader = res
        .load_string("/shaders/terrain.geom")
        .map_err(Into::into)
        .and_then(|source| Shader::from_source(&source, ShaderKind::Geometry))
        .context("Failed to initialize terrain geometry shader")?;

    let vertex_bindings = [VertexBinding::new(
        0,
        VertexAttribute::new(renderer::VertexAttributeFormat::RGB32F, 0),
    )];

    let grass_texture = res
        .load_image("/textures/grass.jpg")
        .map_err(Into::into)
        .and_then(|mut image_data| Texture::from(image_data.as_mut_slice()))
        .context("Failed to load grass texture")?;
    let sand_texture = res
        .load_image("/textures/sand.jpg")
        .map_err(Into::into)
        .and_then(|mut image_data| Texture::from(image_data.as_mut_slice()))
        .context("Failed to load sand texture")?;
    let stone_texture = res
        .load_image("/textures/stone.jpg")
        .map_err(Into::into)
        .and_then(|mut image_data| Texture::from(image_data.as_mut_slice()))
        .context("Failed to load stone texture")?;
    let snow_texture = res
        .load_image("/textures/snow.jpg")
        .map_err(Into::into)
        .and_then(|mut image_data| Texture::from(image_data.as_mut_slice()))
        .context("Failed to load snow texture")?;

    let main_render_pass = RenderPass::new_geom(
        &terrain_vertex_shader,
        &terrain_fragment_shader,
        &terrain_geometry_shader,
        &vertex_bindings,
        &[&matrix_uniform_buffer],
        &[
            &terrain_texture,
            &grass_texture,
            &sand_texture,
            &stone_texture,
            &snow_texture,
        ],
        &[],
    )?;

    let mut camera = Camera::default();
    camera.move_up(1f32);
    let mut chunks = std::collections::HashMap::new();
    for y in 0..CHUNK_LOAD_AREA.0 {
        for x in 0..CHUNK_LOAD_AREA.1 {
            let x = (x as i32) - ((CHUNK_LOAD_AREA.0 as i32) / 2);
            let y = (y as i32) - ((CHUNK_LOAD_AREA.1 as i32) / 2);
            chunks.insert((x, y), initialize_chunk((x, y))?);
        }
    }

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

    let max_count = 2 * (terrain_mesh.width() + 2) * terrain_mesh.height();
    let mut count = max_count;

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

        let chunk_x = camera.position().x.round() as i32;
        let chunk_y = camera.position().z.round() as i32;
        let mut keys_to_remove = HashSet::<ChunkPosition>::new();
        let mut keys_to_add = HashSet::<ChunkPosition>::new();

        for y in 0..CHUNK_LOAD_AREA.0 {
            for x in 0..CHUNK_LOAD_AREA.1 {
                let x = (x as i32) - (CHUNK_LOAD_AREA.0 as i32 / 2) + chunk_x;
                let y = (y as i32) - (CHUNK_LOAD_AREA.1 as i32 / 2) + chunk_y;
                keys_to_add.insert((x, y));
            }
        }

        for key in chunks.keys() {
            if keys_to_add.contains(key) {
                keys_to_add.remove(key);
            } else {
                keys_to_remove.insert(*key);
            }
        }

        chunks.retain(|key, _| !keys_to_remove.contains(key));
        for key in keys_to_add {
            chunks.insert(key, initialize_chunk(key)?);
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

            gl::Enable(gl::CULL_FACE);
            gl::CullFace(gl::BACK);
            gl::FrontFace(gl::CW);
            gl::Enable(gl::DEPTH_TEST);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::Viewport(0, 0, 900, 700);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, terrain_mesh.index_buffer_handle());
            gl::BindVertexBuffer(
                0,
                terrain_mesh.vertex_buffer_handle(),
                0 as GLintptr,
                GLsizei::try_from(std::mem::size_of::<f32>() * 3).unwrap(),
            );

            for chunk in &chunks {
                let position = chunk.1.position();
                matrix_uniforms.model = nalgebra_glm::translation(&nalgebra_glm::vec3(
                    position.0 as f32,
                    0f32,
                    position.1 as f32));

                let matrix_uniforms_ptr = matrix_uniform_buffer.map::<MatrixUniform>();
                matrix_uniforms_ptr.copy_from_slice(&[matrix_uniforms]);
                matrix_uniform_buffer.unmap();

                gl::ActiveTexture(gl::TEXTURE0);
                gl::BindTexture(gl::TEXTURE_2D, chunk.1.texture_handle());

                gl::DrawElements(
                    gl::TRIANGLE_STRIP,
                    count as GLsizei,
                    gl::UNSIGNED_SHORT,
                    std::ptr::null(),
                );
            }
        }

        imgui_context.render(|ui| {
            ui.window("Settings")
                .save_settings(false)
                .always_auto_resize(true)
                .build(|| {
                    ui.checkbox("Wireframe", &mut wireframe);
                    ui.slider("Num vertices", 0, max_count, &mut count);
                });
        });

        context.window().gl_swap_window();
    }
}

fn initialize_chunk(position: ChunkPosition) -> Result<Chunk> {
    let texture = initialize_terrain(TERRAIN_SIZE, TERRAIN_SIZE, position.0, position.1)?;

    Ok(Chunk::init_chunk(position, texture))
}

fn initialize_terrain(width: usize, height: usize, x_offset: i32, y_offset: i32) -> Result<Texture> {
    let mut image_data = vec![0u8; width * height * 4];

    let base_mountain_terrain = RidgedMulti::<OpenSimplex>::default()
        .set_frequency(3.0)
        .set_attenuation(1.2)
        .set_persistence(0.7)
        .set_lacunarity(1.6);
    let mountain_terrain = ScaleBias::new(base_mountain_terrain)
        .set_scale(0.5)
        .set_bias(0.5);
    let base_flat_terrain = Billow::<OpenSimplex>::default().set_frequency(2.0);
    let flat_terrain = ScaleBias::new(base_flat_terrain)
        .set_scale(0.125)
        .set_bias(0.25);
    let noise = Select::new(mountain_terrain, flat_terrain, Perlin::default()).set_falloff(0.8);
    let noise = Cache::new(noise);

    for y in 0..height {
        let base_index = y * width;
        for x in 0..width {
            let index = base_index + x;
            let x = x as f64 / width as f64;
            let y = y as f64 / height as f64;
            let x = x + (x_offset as f64) * 0.5;
            let y = y + (y_offset as f64) * 0.5;
            let noise_value = noise.get([x, y]);
            let height = (1.0 + noise_value) / 2.0;
            image_data[index] = (height * 256.0) as u8;
        }
    }

    Ok(Texture::from_raw_1(image_data.as_slice(), width, height)?)
}
