#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::must_use_candidate)]
extern crate alloc;
extern crate core;
extern crate gl;
extern crate sdl2;

use std::f32::consts::PI;
use std::path::Path;

use gl::types::{GLfloat, GLintptr, GLsizei};
use imgui::TextureId;

use crate::renderer::{Buffer, BufferUsage, Program, RenderPass, Shader, ShaderKind, Texture, VertexAttribute, VertexBinding};
use crate::renderer_context::RendererContext;
use crate::resources::Resources;

pub mod renderer;
mod resources;
mod renderer_context;

type Mat3 = nalgebra_glm::TMat3<f32>;

struct KernelMatrix {
    pub label: String,
    pub matrix: Mat3,
}

fn main() -> Result<(), String> {
    // Initialize render-context
    let context = RendererContext::init()
        .map_err(|e| format!("{e}"))?;

    unsafe {
        gl::Enable(gl::DEBUG_OUTPUT);
        gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
    }

    let res = Resources::from_relative_exe_path(Path::new("assets"))
        .map_err(|e| format!("{e}"))?;

    type Mat4 = nalgebra_glm::TMat4<f32>;
    let vertex_buffer = initialize_vertices()
        .map_err(|e| format!("{e}"))?;
    let index_buffer = initialize_indices()
        .map_err(|e| format!("{e}"))?;

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
                                           &[&render_texture])?;

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
                                           &[&kernel_buffer], &[&render_texture], &[])?;

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

    let imgui_context = &mut imgui::Context::create();

    let _font_texture = {
        let imgui_font = &mut imgui_context.fonts();
        let imgui_texture = &mut imgui_font.build_rgba32_texture();
        let font_texture = Texture::from_raw(imgui_texture.data,
                                             imgui_texture.width as usize,
                                             imgui_texture.height as usize)
            .map_err(|e| format!("{:?}", e))?;
        imgui_font.tex_id = TextureId::new(font_texture.handle() as usize);
        font_texture
    };

    let imgui_vertex = Shader::from_source(
        &res.load_string("/shaders/imgui.vert").map_err(|e| format!("{e}"))?,
        ShaderKind::Vertex)
        .map_err(|e| format!("{e}"))?;
    let imgui_fragment = Shader::from_source(
        &res.load_string("/shaders/imgui.frag").map_err(|e| format!("{e}"))?,
        ShaderKind::Fragment)
        .map_err(|e| format!("{e}"))?;
    let imgui_program = Program::from_shaders(&[&imgui_vertex, &imgui_fragment])?;
    let mut imgui_vbo: gl::types::GLuint = 0;
    let mut imgui_ebo: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut imgui_vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, imgui_vbo);
        gl::GenBuffers(1, &mut imgui_ebo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, imgui_ebo);
    };
    let mut imgui_vao: gl::types::GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut imgui_vao);
        gl::BindVertexArray(imgui_vao);
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE,
                                GLsizei::try_from(std::mem::size_of::<imgui::DrawVert>()).unwrap_unchecked(),
                                (std::mem::size_of::<f32>() * 0) as *const gl::types::GLvoid);
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE,
                                GLsizei::try_from(std::mem::size_of::<imgui::DrawVert>()).unwrap_unchecked(),
                                (std::mem::size_of::<f32>() * 2) as *const gl::types::GLvoid);
        gl::EnableVertexAttribArray(2);
        gl::VertexAttribPointer(2, 4, gl::UNSIGNED_BYTE, gl::TRUE,
                                GLsizei::try_from(std::mem::size_of::<imgui::DrawVert>()).unwrap_unchecked(),
                                (std::mem::size_of::<f32>() * 4) as *const gl::types::GLvoid);
        gl::BindVertexArray(0);
    }

    unsafe {
        gl::Enable(gl::CULL_FACE);
        gl::FrontFace(gl::CCW);
        gl::CullFace(gl::BACK);

        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LEQUAL);
    }

    let mut texture_switch = 0f32;
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

        angle += 0.005f32;
        let view_projection = projection * view;
        let model = nalgebra_glm::rotation(angle, &nalgebra_glm::vec3(1.5f32, 1f32, 0.5f32));
        let matrix_ptr = matrix_buffer.map::<Mat4>();
        matrix_ptr.copy_from_slice(&[model, view_projection]);
        matrix_buffer.unmap();

        let texture_switch_ptr = texture_switch_buffer.map::<f32>();
        texture_switch_ptr.copy_from_slice(&[texture_switch]);
        texture_switch_buffer.unmap();

        let kernel_ptr = kernel_buffer.map::<Mat3>();
        kernel_ptr.copy_from_slice(&[matrices[matrix_index].matrix]);
        kernel_buffer.unmap();

        unsafe {
            // Prepare GUI
            {
                let imgui_io = imgui_context.io_mut();
                imgui_io.display_size = [900f32, 700f32];
                imgui_io.delta_time = 1f32 / 60f32;
                imgui_io.mouse_pos = [mouse_pos.0 as f32, mouse_pos.1 as f32];
                imgui_io.mouse_down[0] = mouse_left;
                imgui_io.mouse_down[1] = mouse_right;

                for &char in &chars {
                    imgui_io.add_input_character(char);
                }
                chars.truncate(0);
            }

            main_render_pass.display();
            clear_screen(0.3, 0.3, 0.5);
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

            // Render GUI
            let display_framebuffer_scale = imgui_context.io().display_framebuffer_scale;
            let ui = imgui_context.frame();
            {
                let _ = imgui::Slider::new("Texture Switch", 0f32, 1f32)
                    .build(&ui, &mut texture_switch);
                let _ = imgui::ComboBox::new("Kernel")
                    .preview_value(&matrices[matrix_index].label)
                    .build(&ui, || {
                        for (index, kernel_matrix) in matrices.iter().enumerate() {
                            if imgui::Selectable::new(&kernel_matrix.label)
                                .selected(index == matrix_index)
                                .build(&ui) {
                                matrix_index = index;
                            }
                        }
                    });
            }

            let draw_data = ui.render();
            // TODO
//            draw_data.scale_clip_rects(imgui_io)

            gl::Enable(gl::BLEND);
            gl::BlendEquation(gl::FUNC_ADD);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Disable(gl::CULL_FACE);
            gl::Disable(gl::DEPTH_TEST);
            gl::Enable(gl::SCISSOR_TEST);
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);

            let display_pos_x = draw_data.display_pos[0];
            let display_pos_y = draw_data.display_pos[1];
            let frame_buffer_width = draw_data.display_size[0] * display_framebuffer_scale[0];
            let frame_buffer_height = draw_data.display_size[1] * display_framebuffer_scale[1];
            gl::Viewport(display_pos_x as i32, display_pos_y as i32, frame_buffer_width as i32, frame_buffer_height as i32);
            let ortho = nalgebra_glm::ortho(0f32, 900f32, 700f32, 0f32, -1f32, 1f32);
            imgui_program.set_used();
            gl::Uniform1i(1, 0);
            gl::UniformMatrix4fv(0, 1, gl::FALSE, nalgebra_glm::value_ptr(&ortho).as_ptr());

            gl::BindVertexArray(imgui_vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, imgui_vbo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, imgui_ebo);

            for draw_list in draw_data.draw_lists() {
                let vtx_buffer = draw_list.vtx_buffer();
                let idx_buffer = draw_list.idx_buffer();
                let mut draw_offset = 0;
                gl::BufferData(gl::ARRAY_BUFFER,
                               gl::types::GLsizeiptr::try_from(std::mem::size_of::<imgui::DrawVert>() * vtx_buffer.len()).unwrap_unchecked(),
                               vtx_buffer.as_ptr().cast(), gl::STREAM_DRAW);
                gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                               gl::types::GLsizeiptr::try_from(std::mem::size_of::<imgui::DrawIdx>() * idx_buffer.len()).unwrap_unchecked(),
                               idx_buffer.as_ptr().cast(), gl::STREAM_DRAW);

                for command in draw_list.commands() {
                    match command {
                        imgui::DrawCmd::Elements { count, cmd_params } => {
                            let clip_rect = cmd_params.clip_rect;
                            let clip_rect = [
                                clip_rect[0] - display_pos_x,
                                clip_rect[1] - display_pos_y,
                                clip_rect[2] - display_pos_x,
                                clip_rect[3] - display_pos_y,
                            ];

                            if clip_rect[0] < frame_buffer_width && clip_rect[1] < frame_buffer_height && clip_rect[2] >= 0f32 && clip_rect[3] >= 0f32 {
                                gl::Scissor(clip_rect[0] as gl::types::GLint,
                                            (frame_buffer_height - clip_rect[3]) as gl::types::GLint,
                                            (clip_rect[2] - clip_rect[0]) as gl::types::GLint,
                                            (clip_rect[3] - clip_rect[1]) as gl::types::GLint);
                                gl::BindTexture(gl::TEXTURE_2D, gl::types::GLuint::try_from(cmd_params.texture_id.id()).unwrap_unchecked());
                                let gl_type = match std::mem::size_of::<imgui::DrawIdx>() {
                                    2 => gl::UNSIGNED_SHORT,
                                    _ => gl::UNSIGNED_INT
                                };
                                gl::DrawElements(gl::TRIANGLES, gl::types::GLsizei::try_from(count).unwrap_unchecked(),
                                                 gl_type, draw_offset as *const std::ffi::c_void);
                            }
                            draw_offset += count;

                            loop {
                                match gl::GetError() {
                                    gl::NO_ERROR => break,
                                    _ => panic!("ERROR"),
                                }
                            }
                        }
                        _ => {
                            panic!("Unimplemented!");
                        }
                    }
                    //                    gl::DrawElements(gl::TRIANGLES, command.ElemCount, gl::UNSIGNED_SHORT, std::ptr::null());
                }
            }

            gl::Disable(gl::BLEND);
            gl::Enable(gl::CULL_FACE);
            gl::Enable(gl::DEPTH_TEST);
            gl::Disable(gl::SCISSOR_TEST);
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
