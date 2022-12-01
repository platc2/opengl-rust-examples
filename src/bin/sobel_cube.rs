#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::must_use_candidate)]
extern crate alloc;
extern crate core;
extern crate gl;
extern crate sdl2;

use std::f32::consts::PI;
use std::path::Path;

use gl::types::{GLfloat, GLintptr, GLsizei};

use hello_triangle_rust::{imgui_wrapper, renderer};
use hello_triangle_rust::renderer::{Buffer, BufferUsage, RenderPass, Shader, ShaderKind, Texture, VertexAttribute, VertexBinding};
use hello_triangle_rust::renderer_context::{OpenGLVersion, RendererContext, WindowDimension};
use hello_triangle_rust::resources::Resources;

type Mat3 = nalgebra_glm::TMat3<f32>;

struct KernelMatrix {
    pub label: String,
    pub matrix: Mat3,
}

#[allow(clippy::too_many_lines)]
fn main() -> Result<(), String> {
    type Mat4 = nalgebra_glm::TMat4<f32>;

    // Initialize render-context
    let context = RendererContext::init("Sobel Cube", &WindowDimension::default(), OpenGLVersion::default())
        .map_err(|e| format!("{e}"))?;

    unsafe {
        gl::Enable(gl::DEBUG_OUTPUT);
        gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
    }

    let res = Resources::from_relative_exe_path(Path::new("../../assets/sobel_cube"))
        .map_err(|e| format!("{e}"))?;

    let vertex_buffer = initialize_vertices()?;
    let index_buffer = initialize_indices()?;

    let vertex_shader = Shader::from_source(
        &res.load_string("/shaders/basic.vert").map_err(|e| format!("{e}"))?,
        ShaderKind::Vertex)
        .map_err(|e| format!("{e}"))?;
    let fragment_shader = Shader::from_source(
        &res.load_string("/shaders/basic.frag").map_err(|e| format!("{e}"))?,
        ShaderKind::Fragment)
        .map_err(|e| format!("{e}"))?;

    let mut matrix_buffer = Buffer::allocate(BufferUsage::Uniform, std::mem::size_of::<Mat4>() * 2)
        .map_err(|e| format!("{:?}", e))?;
    let mut texture_switch_buffer = Buffer::allocate(BufferUsage::Uniform, std::mem::size_of::<f32>())
        .map_err(|e| format!("{:?}", e))?;
    let mut kernel_buffer = Buffer::allocate(BufferUsage::Uniform, std::mem::size_of::<Mat3>())
        .map_err(|e| format!("{:?}", e))?;

    let vertex_bindings = [
        VertexBinding::new(0, VertexAttribute::new(renderer::VertexAttributeFormat::RGB32F, 0)),
        VertexBinding::new(1, VertexAttribute::new(renderer::VertexAttributeFormat::RG32F, 0)),
        VertexBinding::new(2, VertexAttribute::new(renderer::VertexAttributeFormat::RGB32F, 0)),
    ];

    let cube_texture = Texture::from(&mut res.load_image("/textures/cube.tga")
        .map_err(|e| format!("{:?}", e))?)
        .map_err(|e| format!("{:?}", e))?;
    let floor_texture = Texture::from(&mut res.load_image("/textures/floor.tga")
        .map_err(|e| format!("{:?}", e))?)
        .map_err(|e| format!("{:?}", e))?;

    let render_texture = Texture::blank(1024, 1024);

    let main_render_pass = RenderPass::new(&vertex_shader, &fragment_shader, &vertex_bindings,
                                           &[&matrix_buffer, &texture_switch_buffer], &[&cube_texture, &floor_texture],
                                           &[&render_texture])
        .map_err(|e| format!("{e}"))?;

    let cube_vertices = initialize_cube_vertices()?;
    let cube_vertex_bindings = [
        VertexBinding::new(0, VertexAttribute::new(renderer::VertexAttributeFormat::RG32F, 0)),
        VertexBinding::new(1, VertexAttribute::new(renderer::VertexAttributeFormat::RG32F, 0)),
    ];
    let cube_vertex_shader = Shader::from_source(
        &res.load_string("/shaders/cube.vert").map_err(|e| format!("{e}"))?,
        ShaderKind::Vertex)
        .map_err(|e| format!("{e}"))?;
    let cube_fragment_shader = Shader::from_source(
        &res.load_string("/shaders/cube.frag").map_err(|e| format!("{e}"))?,
        ShaderKind::Fragment)
        .map_err(|e| format!("{e}"))?;
    let cube_render_pass = RenderPass::new(&cube_vertex_shader, &cube_fragment_shader, &cube_vertex_bindings,
                                           &[&kernel_buffer], &[&render_texture], &[])
        .map_err(|e| format!("{e}"))?;

    let projection = nalgebra_glm::perspective(1f32, PI / 3f32, 0.001f32, 100f32);
    let view = nalgebra_glm::look_at(
        &nalgebra_glm::vec3(0f32, 0f32, 4f32),
        &nalgebra_glm::vec3(0f32, 0f32, 0f32),
        &nalgebra_glm::vec3(0f32, 1f32, 0f32));

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LESS);
    }

    let mut mouse_pos = (0, 0);
    let mut mouse_left = false;
    let mut mouse_right = false;

    let mut angle = 0f32;
    let mut event_pump = context.sdl().event_pump().expect("Failed to get event pump");

    let mut imgui = imgui_wrapper::Imgui::init();

    unsafe {
        gl::Enable(gl::CULL_FACE);
        gl::FrontFace(gl::CCW);
        gl::CullFace(gl::BACK);

        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LEQUAL);
    }

    let mut delta = 1f32;
    let mut texture_fraction = 0f32;
    let mut rotate = false;
    let mut chars: Vec<char> = Vec::new();
    let matrices = [
        KernelMatrix {
            label: String::from("Identity"),
            matrix: nalgebra_glm::mat3(
                0f32, 0f32, 0f32,
                0f32, 1f32, 0f32,
                0f32, 0f32, 0f32,
            ),
        },
        KernelMatrix {
            label: String::from("Sobel Filter"),
            matrix: nalgebra_glm::mat3(
                -1f32, -1f32, -1f32,
                -1f32, 8f32, -1f32,
                -1f32, -1f32, -1f32,
            ),
        },
        KernelMatrix {
            label: String::from("Sharpen"),
            matrix: nalgebra_glm::mat3(
                0f32, -1f32, 0f32,
                -1f32, 5f32, -1f32,
                0f32, -1f32, 0f32,
            ),
        },
        KernelMatrix {
            label: String::from("Box Blur"),
            matrix: nalgebra_glm::mat3(
                1f32 / 9f32, 1f32 / 9f32, 1f32 / 9f32,
                1f32 / 9f32, 1f32 / 9f32, 1f32 / 9f32,
                1f32 / 9f32, 1f32 / 9f32, 1f32 / 9f32,
            ),
        },
        KernelMatrix {
            label: String::from("Gaussian Blur"),
            matrix: nalgebra_glm::mat3(
                1f32 / 16f32, 2f32 / 16f32, 1f32 / 16f32,
                2f32 / 16f32, 4f32 / 16f32, 2f32 / 16f32,
                1f32 / 16f32, 2f32 / 16f32, 1f32 / 16f32,
            ),
        }
    ];
    let mut matrix_index = 0;

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

        angle += 0.005f32;
        let view_projection = projection * view;
        let model = nalgebra_glm::rotation(angle, &nalgebra_glm::vec3(1.5f32, 1f32, 0.5f32));
        let matrix_ptr = matrix_buffer.map::<Mat4>();
        matrix_ptr.copy_from_slice(&[model, view_projection]);
        matrix_buffer.unmap();

        let texture_switch_ptr = texture_switch_buffer.map::<f32>();
        texture_switch_ptr.copy_from_slice(&[texture_fraction]);
        texture_switch_buffer.unmap();

        let kernel_ptr = kernel_buffer.map::<Mat3>();
        kernel_ptr.copy_from_slice(&[matrices[matrix_index].matrix]);
        kernel_buffer.unmap();

        if rotate {
            texture_fraction = 0.0025f32.mul_add(delta, texture_fraction);
            let new = texture_fraction.clamp(0f32, 1f32);
            if (texture_fraction - new).abs() > 1e-9 {
                delta = -delta;
            }
            texture_fraction = new;
        }

        unsafe {
            imgui.prepare(
                [900f32, 700f32],
                [mouse_pos.0.into(), mouse_pos.1.into()],
                [mouse_left, mouse_right],
                &mut chars);

            main_render_pass.display();
            clear_screen(0.3, 0.3, 0.5);
            clear_screen(0.0, 0.0, 0.0);
            gl::Viewport(0, 0,
                         GLsizei::try_from(render_texture.width()).unwrap_unchecked(),
                         GLsizei::try_from(render_texture.height()).unwrap_unchecked());
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LEQUAL);

            gl::BindVertexBuffer(0, vertex_buffer.handle(),
                                 0 as GLintptr,
                                 GLsizei::try_from(std::mem::size_of::<f32>() * 3).unwrap());
            gl::BindVertexBuffer(1, vertex_buffer.handle(),
                                 GLintptr::try_from(std::mem::size_of::<f32>() * 72).unwrap(),
                                 GLsizei::try_from(std::mem::size_of::<f32>() * 2).unwrap());
            gl::BindVertexBuffer(2, vertex_buffer.handle(),
                                 GLintptr::try_from(std::mem::size_of::<f32>() * 120).unwrap(),
                                 GLsizei::try_from(std::mem::size_of::<f32>() * 3).unwrap());
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, index_buffer.handle());
            let count = GLsizei::try_from(index_buffer.size() / std::mem::size_of::<u16>())
                .unwrap();
            gl::DrawElements(gl::TRIANGLES, count, gl::UNSIGNED_SHORT, std::ptr::null());

            cube_render_pass.display();
            gl::Disable(gl::DEPTH_TEST);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::Viewport(0, 0, 900, 700);
            gl::BindVertexBuffer(0, cube_vertices.handle(), 0 as GLintptr,
                                 GLsizei::try_from(std::mem::size_of::<f32>() * 2).unwrap());
            gl::BindVertexBuffer(1, cube_vertices.handle(),
                                 GLintptr::try_from(std::mem::size_of::<f32>() * 12).unwrap(),
                                 GLsizei::try_from(std::mem::size_of::<f32>() * 2).unwrap());
            gl::DrawArrays(gl::TRIANGLES, 0, 6);

            imgui.render(|ui| {
                imgui::Window::new("Settings")
                    .save_settings(false)
                    .always_auto_resize(true)
                    .build(ui, || {
                        imgui::Slider::new("Texture Switch", 0f32, 1f32)
                            .build(ui, &mut texture_fraction);
                        ui.same_line();
                        ui.checkbox("Cycle", &mut rotate);
                        imgui::ComboBox::new("Kernel")
                            .preview_value(&matrices[matrix_index].label)
                            .build(ui, || {
                                for (index, kernel_matrix) in matrices.iter().enumerate() {
                                    if imgui::Selectable::new(&kernel_matrix.label)
                                        .selected(index == matrix_index)
                                        .build(ui) {
                                        matrix_index = index;
                                    }
                                }
                            });
                    });
            });
        }

        context.window().gl_swap_window();
    }
}

/// # Errors
/// - Fail to initialize vertex buffer
fn initialize_cube_vertices() -> Result<Buffer, String> {
    let vertices = vec![
        -1f32, 1f32, -1f32, -1f32, 1f32, -1f32,
        1f32, -1f32, 1f32, 1f32, -1f32, 1f32,
        0f32, 1f32, 0f32, 0f32, 1f32, 0f32,
        1f32, 0f32, 1f32, 1f32, 0f32, 1f32,
    ];

    let mut vertex_buffer = Buffer::allocate(BufferUsage::Vertex, std::mem::size_of::<f32>() * vertices.len())
        .map_err(|e| format!("{:?}", e))?;
    let ptr = vertex_buffer.map::<f32>();
    ptr.copy_from_slice(&vertices);
    vertex_buffer.unmap();
    Ok(vertex_buffer)
}

/// # Errors
/// - Fail to initialize vertex buffer
fn initialize_vertices() -> Result<Buffer, String> {
    let vertices = vec![
        // Vertices
        // Front face
        -0.5f32, 0.5f32, 0.5f32, -0.5f32, -0.5f32, 0.5f32, 0.5f32, -0.5f32, 0.5f32, 0.5f32, 0.5f32, 0.5f32,
        // Right face
        0.5f32, 0.5f32, 0.5f32, 0.5f32, -0.5f32, 0.5f32, 0.5f32, -0.5f32, -0.5f32, 0.5f32, 0.5f32, -0.5f32,
        // Back face
        0.5f32, 0.5f32, -0.5f32, 0.5f32, -0.5f32, -0.5f32, -0.5f32, -0.5f32, -0.5f32, -0.5f32, 0.5f32, -0.5f32,
        // Left face
        -0.5f32, 0.5f32, -0.5f32, -0.5f32, -0.5f32, -0.5f32, -0.5f32, -0.5f32, 0.5f32, -0.5f32, 0.5f32, 0.5f32,
        // Top face
        -0.5f32, 0.5f32, -0.5f32, -0.5f32, 0.5f32, 0.5f32, 0.5f32, 0.5f32, 0.5f32, 0.5f32, 0.5f32, -0.5f32,
        // Bottom face
        -0.5f32, -0.5f32, 0.5f32, -0.5f32, -0.5f32, -0.5f32, 0.5f32, -0.5f32, -0.5f32, 0.5f32, -0.5f32, 0.5f32,

        // Texture coordinates
        // Front face
        0.0f32, 1.0f32, 0.0f32, 0.0f32, 1.0f32, 0.0f32, 1.0f32, 1.0f32,
        // Right face
        0.0f32, 1.0f32, 0.0f32, 0.0f32, 1.0f32, 0.0f32, 1.0f32, 1.0f32,
        // Back face
        0.0f32, 1.0f32, 0.0f32, 0.0f32, 1.0f32, 0.0f32, 1.0f32, 1.0f32,
        // Left face
        0.0f32, 1.0f32, 0.0f32, 0.0f32, 1.0f32, 0.0f32, 1.0f32, 1.0f32,
        // Top face
        0.0f32, 1.0f32, 0.0f32, 0.0f32, 1.0f32, 0.0f32, 1.0f32, 1.0f32,
        // Bottom face
        0.0f32, 1.0f32, 0.0f32, 0.0f32, 1.0f32, 0.0f32, 1.0f32, 1.0f32,

        // Normals
        // Front face
        0f32, 0f32, 1f32, 0f32, 0f32, 1f32, 0f32, 0f32, 1f32, 0f32, 0f32, 1f32,
        // Right face
        1f32, 0f32, 0f32, 1f32, 0f32, 0f32, 1f32, 0f32, 0f32, 1f32, 0f32, 0f32,
        // Back face
        0f32, 0f32, -1f32, 0f32, 0f32, -1f32, 0f32, 0f32, -1f32, 0f32, 0f32, -1f32,
        // Left face
        -1f32, 0f32, 0f32, -1f32, 0f32, 0f32, -1f32, 0f32, 0f32, -1f32, 0f32, 0f32,
        // Top face
        0f32, 1f32, 0f32, 0f32, 1f32, 0f32, 0f32, 1f32, 0f32, 0f32, 1f32, 0f32,
        // Bottom face
        0f32, -1f32, 0f32, 0f32, -1f32, 0f32, 0f32, -1f32, 0f32, 0f32, -1f32, 0f32,
    ];
    let mut vertex_buffer = Buffer::allocate(BufferUsage::Vertex, std::mem::size_of::<f32>() * vertices.len())
        .map_err(|e| format!("{:?}", e))?;
    let ptr = vertex_buffer.map::<f32>();
    ptr.copy_from_slice(&vertices);
    vertex_buffer.unmap();
    Ok(vertex_buffer)
}

/// # Errors
/// - Fail to initialize index buffer
pub fn initialize_indices() -> Result<Buffer, String> {
    let indices = vec![
        // Front face
        0u16, 1u16, 2u16, 0u16, 2u16, 3u16,
        // Right face
        4u16, 5u16, 6u16, 4u16, 6u16, 7u16,
        // Back face
        8u16, 9u16, 10u16, 8u16, 10u16, 11u16,
        // Left face
        12u16, 13u16, 14u16, 12u16, 14u16, 15u16,
        // Top face
        16u16, 17u16, 18u16, 16u16, 18u16, 19u16,
        // Bottom face
        20u16, 21u16, 22u16, 20u16, 22u16, 23u16,
    ];
    let mut index_buffer = Buffer::allocate(BufferUsage::Index, std::mem::size_of::<u16>() * indices.len())
        .map_err(|e| format!("{:?}", e))?;
    let ptr = index_buffer.map::<u16>();
    ptr.copy_from_slice(&indices);
    index_buffer.unmap();
    Ok(index_buffer)
}

fn clear_screen(red: f32, green: f32, blue: f32) {
    unsafe {
        gl::ClearColor(
            red as GLfloat,
            green as GLfloat,
            blue as GLfloat,
            1f32 as GLfloat);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }
}
