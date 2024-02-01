use std::any::Any;
use std::collections::HashMap;
use std::ffi::c_void;
use std::time::Duration;

use imgui::{BackendFlags, FontAtlas, TextureId};

use gl::types::{GLint, GLsizei, GLuint};

use crate::input_manager::Key;
use crate::renderer::{Program, Shader, ShaderKind, Texture};

const IMGUI_VERTEX_SHADER_SOURCE: &str = include_str!("imgui.vert");
const IMGUI_FRAGMENT_SHADER_SOURCE: &str = include_str!("imgui.frag");

pub struct Imgui {
    context: imgui::Context,
    program: Program,
    vao: GLuint,
    vbo: GLuint,
    element_buffer_object: GLuint,
}

type WindowDimension = [f32; 2];
type MousePos = [f32; 2];
type MouseButtonState = [bool; 2];

impl Imgui {
    #[must_use]
    pub fn init() -> Self {
        let mut context = imgui::Context::create();
        context.io_mut().backend_flags = BackendFlags::RENDERER_HAS_VTX_OFFSET;
        let _font_texture = generate_font_texture_from_atlas(context.fonts());
        let program = create_program();
        let mut vertex_buffer_object: GLuint = 0;
        let mut element_buffer_object: GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut vertex_buffer_object);
            gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer_object);
            gl::GenBuffers(1, &mut element_buffer_object);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, element_buffer_object);
        };
        let mut vao: GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                2,
                gl::FLOAT,
                gl::FALSE,
                GLsizei::try_from(std::mem::size_of::<imgui::DrawVert>()).unwrap_unchecked(),
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                GLsizei::try_from(std::mem::size_of::<imgui::DrawVert>()).unwrap_unchecked(),
                (std::mem::size_of::<f32>() * 2) as *const gl::types::GLvoid,
            );
            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(
                2,
                4,
                gl::UNSIGNED_BYTE,
                gl::TRUE,
                GLsizei::try_from(std::mem::size_of::<imgui::DrawVert>()).unwrap_unchecked(),
                (std::mem::size_of::<f32>() * 4) as *const gl::types::GLvoid,
            );
            gl::BindVertexArray(0);
        }

        Self {
            context,
            program,
            vao,
            vbo: vertex_buffer_object,
            element_buffer_object,
        }
    }

    pub fn prepare(
        &mut self,
        window_dimension: WindowDimension,
        mouse_pos: MousePos,
        mouse_button_state: MouseButtonState,
        key_changes: &HashMap<Key, bool>,
        text_input: &Vec<String>,
        delta: Duration,
    ) {
        let io = self.context.io_mut();
        io.display_size = window_dimension;
        io.delta_time = delta.as_secs_f32();

        io.mouse_pos = mouse_pos;
        io.mouse_down[0] = mouse_button_state[0];
        io.mouse_down[1] = mouse_button_state[1];

        for (key, state) in key_changes {
            io.add_key_event(key.imgui, *state);
        }

        for text in text_input {
            for char in text.chars() {
                io.add_input_character(char);
            }
        }
    }

    /// # Panics
    /// - Unimplemented draw command
    #[allow(clippy::too_many_lines)]
    pub fn render<F>(&mut self, mut callback: F)
        where
            F: FnMut(&imgui::Ui),
    {
        let ui = self.context.frame();
        callback(ui);
        let draw_data = self.context.render();

        let message = "ImGui Rendering";
        // Message length is guaranteed to not exceed 31bits
        #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
        unsafe {
            gl::PushDebugGroup(
                gl::DEBUG_SOURCE_APPLICATION,
                2 as GLuint,
                message.len() as GLsizei,
                message.as_ptr().cast(),
            );
        }

        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendEquation(gl::FUNC_ADD);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Disable(gl::CULL_FACE);
            gl::Disable(gl::DEPTH_TEST);
            gl::Enable(gl::SCISSOR_TEST);
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);

            let [display_pos_x, display_pos_y] = draw_data.display_pos;
            let [display_size_w, display_size_h] = draw_data.display_size;
            let frame_buffer_width = display_pos_x + display_size_w;
            let frame_buffer_height = display_pos_y + display_size_h;
            gl::Viewport(
                display_pos_x as i32,
                display_pos_y as i32,
                frame_buffer_width as i32,
                frame_buffer_height as i32,
            );
            let ortho = nalgebra_glm::ortho(
                display_pos_x,
                display_pos_x + display_size_w,
                display_pos_y + display_size_h,
                display_pos_y,
                -1f32,
                1f32,
            );
            self.program.set_used();
            gl::Uniform1i(1, 0);
            gl::UniformMatrix4fv(0, 1, gl::FALSE, nalgebra_glm::value_ptr(&ortho).as_ptr());

            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.element_buffer_object);

            gl::ActiveTexture(gl::TEXTURE0);
            for draw_list in draw_data.draw_lists() {
                let vtx_buffer = draw_list.vtx_buffer();
                let idx_buffer = draw_list.idx_buffer();
                gl::BufferData(
                    gl::ARRAY_BUFFER,
                    gl::types::GLsizeiptr::try_from(std::mem::size_of_val(vtx_buffer))
                        .unwrap_unchecked(),
                    vtx_buffer.as_ptr().cast(),
                    gl::STREAM_DRAW,
                );
                gl::BufferData(
                    gl::ELEMENT_ARRAY_BUFFER,
                    gl::types::GLsizeiptr::try_from(std::mem::size_of_val(idx_buffer))
                        .unwrap_unchecked(),
                    idx_buffer.as_ptr().cast(),
                    gl::STREAM_DRAW,
                );

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

                            let vtx_offset = cmd_params.vtx_offset;
                            let idx_offset = cmd_params.idx_offset * std::mem::size_of::<imgui::DrawIdx>();
                            if clip_rect[0] < frame_buffer_width
                                && clip_rect[1] < frame_buffer_height
                                && clip_rect[2] >= 0f32
                                && clip_rect[3] >= 0f32
                            {
                                gl::Scissor(
                                    clip_rect[0] as GLint,
                                    (frame_buffer_height - clip_rect[3]) as GLint,
                                    (clip_rect[2] - clip_rect[0]) as GLint,
                                    (clip_rect[3] - clip_rect[1]) as GLint,
                                );

                                gl::BindTexture(
                                    gl::TEXTURE_2D,
                                    gl::types::GLuint::try_from(cmd_params.texture_id.id())
                                        .unwrap_unchecked(),
                                );
                                let gl_type = match std::mem::size_of::<imgui::DrawIdx>() {
                                    2 => gl::UNSIGNED_SHORT,
                                    _ => gl::UNSIGNED_INT,
                                };
                                gl::DrawElementsBaseVertex(
                                    gl::TRIANGLES,
                                    GLsizei::try_from(count).unwrap_unchecked(),
                                    gl_type,
                                    idx_offset as *const c_void,
                                    vtx_offset as GLint,
                                );
                            }
                        }
                        x => {
                            panic!("Unimplemented! {:?}", x.type_id());
                        }
                    }
                }
            }

            gl::BindVertexArray(0);

            gl::Disable(gl::BLEND);
            gl::Enable(gl::CULL_FACE);
            gl::Disable(gl::SCISSOR_TEST);
            gl::PopDebugGroup();
        }
    }
}

fn generate_font_texture_from_atlas(font_atlas: &mut FontAtlas) -> Texture {
    let font_atlas_texture = &mut font_atlas.build_rgba32_texture();
    let font_texture = Texture::from_raw(
        font_atlas_texture.data,
        font_atlas_texture.width as usize,
        font_atlas_texture.height as usize,
    )
        .expect("Failed to create font texture for Dear ImGui");
    font_atlas.tex_id = TextureId::new(font_texture.handle() as usize);
    font_texture
}

fn create_program() -> Program {
    let vertex_shader = Shader::from_source(IMGUI_VERTEX_SHADER_SOURCE, ShaderKind::Vertex)
        .expect("Failed to setup Dear ImGui vertex shader");
    let fragment_shader = Shader::from_source(IMGUI_FRAGMENT_SHADER_SOURCE, ShaderKind::Fragment)
        .expect("Failed to setup Dear ImGui fragment shader");
    Program::from_shaders(&[&vertex_shader, &fragment_shader])
        .expect("Failed to setup Dear ImGui program")
}
