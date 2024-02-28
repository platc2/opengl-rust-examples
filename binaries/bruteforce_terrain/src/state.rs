use std::collections::{HashMap, HashSet};
use std::time::Instant;

use imgui::Ui;
use noise::{Billow, Cache, MultiFractal, NoiseFn, OpenSimplex, Perlin, RidgedMulti, ScaleBias, Select};

use gl::sys::types::{GLintptr, GLsizei};
use renderer::{Buffer, RenderPass, Texture};
use renderer::application::Application;
use renderer::input_manager::{InputManager, Key};
use renderer::time::Time;

use crate::camera::Camera;
use crate::chunk::{Chunk, ChunkPosition};
use crate::matrix_uniform::MatrixUniform;
use crate::terrain_mesh::TerrainMesh;

const TERRAIN_SIZE: usize = 32;
const CHUNK_LOAD_AREA: (usize, usize) = (5, 5);

pub struct State {
    camera: Camera,
    chunks: HashMap<(i32, i32), Chunk>,
    matrix_uniforms: MatrixUniform,
    matrix_uniform_buffer: Buffer,
    main_render_pass: RenderPass,
    terrain_mesh: TerrainMesh,
    wireframe: bool,
    max_count: usize,
    count: usize,
    quit: bool,
}

impl State {
    pub fn new(matrix_uniforms: MatrixUniform, matrix_uniform_buffer: Buffer, main_render_pass: RenderPass) -> Self {
        let mut camera = Camera::default();
        camera.move_up(1.);
        let mut chunks = HashMap::new();
        for y in 0..CHUNK_LOAD_AREA.0 {
            for x in 0..CHUNK_LOAD_AREA.1 {
                let x = (x as i32) - ((CHUNK_LOAD_AREA.0 as i32) / 2);
                let y = (y as i32) - ((CHUNK_LOAD_AREA.1 as i32) / 2);
                chunks.insert((x, y), initialize_chunk((x, y)).unwrap());
            }
        }
        let terrain_mesh = TerrainMesh::of_size(TERRAIN_SIZE, TERRAIN_SIZE)
            .unwrap();
        let max_count = 2 * (terrain_mesh.width() + 2) * terrain_mesh.height();

        Self {
            camera,
            chunks,
            matrix_uniforms,
            matrix_uniform_buffer,
            main_render_pass,
            terrain_mesh,
            wireframe: false,
            max_count,
            count: max_count,
            quit: false,
        }
    }
}

impl Application for State {
    fn tick(&mut self, _: &Time<Instant>, input_manager: &dyn InputManager) {
        if input_manager.key_down(Key::ESCAPE) {
            self.quit = true;
        }

        let chunk_x = self.camera.position().x.round() as i32;
        let chunk_y = self.camera.position().z.round() as i32;
        let mut keys_to_remove = HashSet::<ChunkPosition>::new();
        let mut keys_to_add = HashSet::<ChunkPosition>::new();

        for y in 0..CHUNK_LOAD_AREA.0 {
            for x in 0..CHUNK_LOAD_AREA.1 {
                let x = (x as i32) - (CHUNK_LOAD_AREA.0 as i32 / 2) + chunk_x;
                let y = (y as i32) - (CHUNK_LOAD_AREA.1 as i32 / 2) + chunk_y;
                keys_to_add.insert((x, y));
            }
        }

        for key in self.chunks.keys() {
            if keys_to_add.contains(key) {
                keys_to_add.remove(key);
            } else {
                keys_to_remove.insert(*key);
            }
        }

        self.chunks.retain(|key, _| !keys_to_remove.contains(key));
        for key in keys_to_add {
            self.chunks.insert(key, initialize_chunk(key).unwrap());
        }

        let speed = 0.005f32;
        if input_manager.key_down(Key::W) {
            self.camera.move_forward(speed);
        }
        if input_manager.key_down(Key::S) {
            self.camera.move_forward(-speed);
        }
        if input_manager.key_down(Key::D) {
            self.camera.move_right(speed);
        }
        if input_manager.key_down(Key::A) {
            self.camera.move_right(-speed);
        }
        if input_manager.key_down(Key::SPACE) {
            self.camera.move_up(speed);
        }
        if input_manager.key_down(Key::LEFT_CONTROL) {
            self.camera.move_up(-speed);
        }
        if input_manager.key_down(Key::UP_ARROW) {
            self.camera.look_up(speed);
        }
        if input_manager.key_down(Key::DOWN_ARROW) {
            self.camera.look_up(-speed);
        }
        if input_manager.key_down(Key::RIGHT_ARROW) {
            self.camera.look_right(speed);
        }
        if input_manager.key_down(Key::LEFT_ARROW) {
            self.camera.look_right(-speed);
        }

        self.matrix_uniforms.view = self.camera.view_matrix();

        let matrix_uniforms_ptr = self.matrix_uniform_buffer.map::<MatrixUniform>();
        matrix_uniforms_ptr.copy_from_slice(&[self.matrix_uniforms]);
        self.matrix_uniform_buffer.unmap();

        unsafe {
            gl::sys::PolygonMode(
                gl::sys::FRONT_AND_BACK,
                if self.wireframe { gl::sys::LINE } else { gl::sys::FILL },
            );

            self.main_render_pass.display();

            gl::sys::Enable(gl::sys::CULL_FACE);
            gl::sys::CullFace(gl::sys::BACK);
            gl::sys::FrontFace(gl::sys::CW);
            gl::sys::Enable(gl::sys::DEPTH_TEST);
            gl::sys::Clear(gl::sys::COLOR_BUFFER_BIT | gl::sys::DEPTH_BUFFER_BIT);
            gl::sys::Viewport(0, 0, 900, 700);
            gl::sys::BindBuffer(gl::sys::ELEMENT_ARRAY_BUFFER, self.terrain_mesh.index_buffer_handle());
            gl::sys::BindVertexBuffer(
                0,
                self.terrain_mesh.vertex_buffer_handle(),
                0 as GLintptr,
                GLsizei::try_from(std::mem::size_of::<f32>() * 3).unwrap(),
            );

            for chunk in &self.chunks {
                let position = chunk.1.position();
                self.matrix_uniforms.model = nalgebra_glm::translation(&nalgebra_glm::vec3(
                    position.0 as f32,
                    0f32,
                    position.1 as f32));

                let matrix_uniforms_ptr = self.matrix_uniform_buffer.map::<MatrixUniform>();
                matrix_uniforms_ptr.copy_from_slice(&[self.matrix_uniforms]);
                self.matrix_uniform_buffer.unmap();

                gl::sys::ActiveTexture(gl::sys::TEXTURE0);
                gl::sys::BindTexture(gl::sys::TEXTURE_2D, chunk.1.texture_handle());

                gl::sys::DrawElements(
                    gl::sys::TRIANGLE_STRIP,
                    self.count as GLsizei,
                    gl::sys::UNSIGNED_SHORT,
                    std::ptr::null(),
                );
            }
        }
    }

    fn gui(&mut self, ui: &Ui) {
        ui.window("Settings")
            .save_settings(false)
            .always_auto_resize(true)
            .build(|| {
                ui.checkbox("Wireframe", &mut self.wireframe);
                ui.slider("Num vertices", 0, self.max_count, &mut self.count);
            });
    }

    fn quit(&self) -> bool {
        self.quit
    }
}

fn initialize_chunk(position: ChunkPosition) -> anyhow::Result<Chunk> {
    let texture = initialize_terrain(TERRAIN_SIZE, TERRAIN_SIZE, position.0, position.1)?;

    Ok(Chunk::init_chunk(position, texture))
}

fn initialize_terrain(width: usize, height: usize, x_offset: i32, y_offset: i32) -> anyhow::Result<Texture> {
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
