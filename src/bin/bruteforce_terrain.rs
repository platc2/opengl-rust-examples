#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::must_use_candidate)]
extern crate alloc;
extern crate core;
extern crate gl;
extern crate sdl2;

use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use gl::types::{GLintptr, GLsizei};
use noise::{NoiseFn, Perlin};

use hello_triangle_rust::{imgui_wrapper, renderer};
use hello_triangle_rust::renderer::{Buffer, BufferUsage, RenderPass, Shader, ShaderKind, Texture, VertexAttribute, VertexBinding};
use hello_triangle_rust::renderer_context::{OpenGLVersion, RendererContext, WindowDimension};
use hello_triangle_rust::resources::Resources;

type Mat4 = nalgebra_glm::TMat4<f32>;

#[derive(Default, Copy, Clone)]
struct MatrixUniform {
    model: Mat4,
    view: Mat4,
    projection: Mat4,
}

fn main() -> Result<(), String> {
    let window_dimension = WindowDimension::default();
    // Initialize render-context
    let context = RendererContext::init("Bruteforce Terrain", &window_dimension, OpenGLVersion::default())
        .map_err(|e| format!("{e}"))?;

    let res = Resources::from_relative_exe_path(Path::new("../../assets/bruteforce_terrain"))
        .map_err(|e| format!("{e}"))?;

    let mut matrix_uniform_buffer = Buffer::allocate(BufferUsage::Uniform, std::mem::size_of::<MatrixUniform>())
        .map_err(|e| format!("{e}"))?;
    let mut matrix_uniforms = MatrixUniform::default();

    matrix_uniforms.model = nalgebra_glm::TMat4::identity();
    matrix_uniforms.projection = nalgebra_glm::perspective(
        window_dimension.width as f32 / window_dimension.height as f32,
        60f32.to_radians(),
        0.01f32,
        100f32);

    const TERRAIN_TEXTURE_SIZE: usize = 256;
    const TERRAIN_MESH_SIZE: usize = 128;
    let terrain_texture = initialize_terrain(TERRAIN_TEXTURE_SIZE, TERRAIN_TEXTURE_SIZE)?;
    let mut vertex_buffer = initialize_terrain_vertices(TERRAIN_MESH_SIZE, TERRAIN_MESH_SIZE)?;
    let index_buffer = initialize_terrain_indices(TERRAIN_MESH_SIZE, TERRAIN_MESH_SIZE)?;

    let terrain_vertex_shader = Shader::from_source(
        &res.load_string("/shaders/terrain.vert").map_err(|e| format!("{e}"))?,
        ShaderKind::Vertex)
        .map_err(|e| format!("{e}"))?;
    let terrain_fragment_shader = Shader::from_source(
        &res.load_string("/shaders/terrain.frag").map_err(|e| format!("{e}"))?,
        ShaderKind::Fragment)
        .map_err(|e| format!("{e}"))?;

    let vertex_bindings = [
        VertexBinding::new(0, VertexAttribute::new(renderer::VertexAttributeFormat::RGB32F, 0)),
    ];

    let main_render_pass = RenderPass::new(&terrain_vertex_shader, &terrain_fragment_shader, &vertex_bindings,
                                           &[&matrix_uniform_buffer], &[&terrain_texture], &[])
        .map_err(|e| format!("{e}"))?;

    let mut mouse_pos = (0, 0);
    let mut mouse_left = false;
    let mut mouse_right = false;

    let mut event_pump = context.sdl().event_pump().expect("Failed to get event pump");

    let mut chars: Vec<char> = Vec::new();

    let mut imgui_context = imgui_wrapper::Imgui::init();
    let mut wireframe = false;

    'main: loop {
        for event in event_pump.poll_iter() {
            use sdl2::event::Event;
            use sdl2::mouse::MouseButton;
            use sdl2::keyboard::Keycode;
            match event {
                Event::MouseMotion { x, y, .. } => mouse_pos = (
                    // This is ok - Mouse coordinates shouldn't reach numbers which overflow 16bit
                    i16::try_from(x).unwrap_or(0),
                    i16::try_from(y).unwrap_or(0)),
                Event::MouseButtonDown { mouse_btn: MouseButton::Left, .. } => mouse_left = true,
                Event::MouseButtonUp { mouse_btn: MouseButton::Left, .. } => mouse_left = false,
                Event::MouseButtonDown { mouse_btn: MouseButton::Right, .. } => mouse_right = true,
                Event::MouseButtonUp { mouse_btn: MouseButton::Right, .. } => mouse_right = false,
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } =>
                    break 'main Ok(()),
                Event::KeyDown { keycode: Some(key_code), .. } => {
                    let key_code = key_code as u32;
                    if (32..512).contains(&key_code) { chars.push(char::from_u32(key_code).unwrap()); }
                }
                _ => {}
            }
        }

        imgui_context.prepare(
            [window_dimension.width as f32, window_dimension.height as f32],
            [mouse_pos.0.into(), mouse_pos.1.into()],
            [mouse_left, mouse_right],
            &mut chars);

        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| format!("{e}"))?
            .as_millis();
        let angle = ((time % (90 * 360)) as f32).to_radians() / 90f32;
        let position = nalgebra_glm::vec3(
            angle.cos() * 2f32,
            1f32,
            angle.sin() * 2f32,
        );
        matrix_uniforms.view = nalgebra_glm::look_at(
            &position,
            &nalgebra_glm::vec3(0f32, 0f32, 0f32),
            &nalgebra_glm::vec3(0f32, 1f32, 0f32));

        let matrix_uniforms_ptr = matrix_uniform_buffer.map::<MatrixUniform>();
        matrix_uniforms_ptr.copy_from_slice(&[matrix_uniforms]);
        matrix_uniform_buffer.unmap();

        unsafe {
            gl::PolygonMode(gl::FRONT_AND_BACK, if wireframe { gl::LINE } else { gl::FILL });
            main_render_pass.display();

            gl::Disable(gl::CULL_FACE);
            gl::Enable(gl::DEPTH_TEST);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::Viewport(0, 0, 900, 700);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, index_buffer.handle());
            gl::BindVertexBuffer(0, vertex_buffer.handle(), 0 as GLintptr,
                                 GLsizei::try_from(std::mem::size_of::<f32>() * 3).unwrap());
            let count = index_buffer.size() / std::mem::size_of::<u16>();
            gl::DrawElements(gl::TRIANGLE_STRIP, count as GLsizei, gl::UNSIGNED_INT, std::ptr::null());
        }

        imgui_context.render(|ui| {
            imgui::Window::new("Settings")
                .save_settings(false)
                .always_auto_resize(true)
                .build(ui, || {
                    ui.checkbox("Wireframe", &mut wireframe);
                });
        });

        context.window().gl_swap_window();
    }
}

const NOISE_SCALE: f64 = 200.0;

fn fbm(perlin: &Perlin, point: [f64; 2], lacunarity: f32, H: f32, octaves: u8) -> f32 {
    let mut value = 0.0;
    let mut point = point;
    for i in 0..octaves {
        value += lacunarity.powf(-(i as f32) * H) * perlin.get(point) as f32;
        point = point.map(|v| v * lacunarity as f64);
    }

    value
}

fn layered_noise(x: f64, y: f64, perlin: &Perlin) -> f64 {
    let mut value = 0.0;

    let frequency: f64 = 2.0;
    let amplitude: f64 = 2.0;
    for i in 0..8 {
        let i = i as f64;
        let frequency = frequency.powf(i);
        let amplitude = amplitude.powf(-(i + 2.0));
        value += perlin.get([frequency * x, frequency * y]) * amplitude;
    }
    value * 2.0
}

fn initialize_terrain(width: usize, height: usize) -> Result<Texture, String> {
    let mut image_data = Vec::<u8>::with_capacity(width * height * 4);

    let perlin = Perlin::new(1);
    for y in 0..height {
        for x in 0..width {
            let x = (x as f64 / width as f64);
            let y = (y as f64 / height as f64);
            let height = layered_noise(x, y, &perlin);
            image_data.push((height * 512.0) as u8);
            image_data.push(0);
            image_data.push(0);
            image_data.push(0);
        }
    }

    Texture::from_raw(image_data.as_slice(), width, height)
        .map_err(|e| format!("{e}"))
}

fn initialize_terrain_vertices(width: usize, height: usize) -> Result<Buffer, String> {
    let num_vertices = width * height * 3;
    let mut vertices = vec![0f32; num_vertices];

    for y in 0..height {
        for x in 0..width {
            let index = ((y * width) + x) * 3;
            let x = x as f32;
            let y = y as f32;
            let width = width as f32;
            let height = height as f32;
            let x = (x * 2f32) / width - 1f32;
            let y = (y * 2f32) / height - 1f32;
            vertices[index] = x;
            vertices[index + 1] = 0f32;
            vertices[index + 2] = y;
        }
    };

    let mut vertex_buffer = Buffer::allocate(BufferUsage::Vertex, std::mem::size_of::<f32>() * vertices.len())
        .map_err(|e| format!("{e}"))?;
    let ptr = vertex_buffer.map::<f32>();
    ptr.copy_from_slice(&vertices);
    vertex_buffer.unmap();
    Ok(vertex_buffer)
}

fn initialize_terrain_indices(width: usize, height: usize) -> Result<Buffer, String> {
    let mut indices = vec![0u32; 0];

    for y in 0..(height - 1) {
        for x in 0..width {
            let row_index = ((y * width) + x) as u32;
            let next_row_index = (((y + 1) * width) + x) as u32;
            if x == 0 { indices.push(row_index) };
            indices.push(row_index);
            indices.push(next_row_index);
            if x == (width - 1) { indices.push(next_row_index) };
        }
    }

    let mut index_buffer = Buffer::allocate(BufferUsage::Index, std::mem::size_of::<u32>() * indices.len())
        .map_err(|e| format!("{e}"))?;
    let ptr = index_buffer.map();
    ptr.copy_from_slice(&indices);
    index_buffer.unmap();
    Ok(index_buffer)
}
