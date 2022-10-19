use std::any::Any;
use std::ffi::c_void;
use gl::types::{GLint, GLsizei, GLuint};
use imgui::{FontAtlasRefMut, TextureId};

use crate::renderer::{Program, Shader, ShaderKind, Texture};

const IMGUI_VERTEX_SHADER_SOURCE: &'static str = include_str!("imgui.vert");
const IMGUI_FRAGMENT_SHADER_SOURCE: &'static str = include_str!("imgui.frag");

pub struct Imgui {
    context: imgui::Context,
    program: Program,
    vao: GLuint,
    _vbo: GLuint,
    ebo: GLuint,
}


impl Imgui {
    pub fn init() -> Self {
        let mut context = imgui::Context::create();
        let _font_texture = generate_font_texture_from_atlas(&mut context.fonts());
        let program = create_program();
        let mut vbo: GLuint = 0;
        let mut ebo: GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::GenBuffers(1, &mut ebo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        };
        let mut vao: GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE,
                                    GLsizei::try_from(std::mem::size_of::<imgui::DrawVert>()).unwrap_unchecked(),
                                    std::ptr::null());
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

        Self { context, program, vao, _vbo: vbo, ebo }
    }

    pub fn prepare(&mut self, window_width: f32, window_height: f32, mouse_x: f32, mouse_y: f32,
                   mouse_left: bool, mouse_right: bool, chars: &mut Vec<char>) {
        let io = self.context.io_mut();
        io.display_size = [window_width, window_height];
        io.delta_time = 1f32 / 60f32;

        io.mouse_pos = [mouse_x, mouse_y];
        io.mouse_down[0] = mouse_left;
        io.mouse_down[1] = mouse_right;

        for char in chars.iter() {
            io.add_input_character(*char);
        }
        chars.truncate(0);
    }

    pub fn render<F>(&mut self, mut callback: F) where F: FnMut(&imgui::Ui) -> ()
    {
        let ui = self.context.frame();
        callback(&ui);
        let draw_data = ui.render();

        let message = "ImGui Rendering";
        unsafe { gl::PushDebugGroup(gl::DEBUG_SOURCE_APPLICATION, 2 as GLuint, message.len() as GLsizei, message.as_ptr().cast()); }

        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendEquation(gl::FUNC_ADD);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Disable(gl::CULL_FACE);
            gl::Disable(gl::DEPTH_TEST);
            gl::Enable(gl::SCISSOR_TEST);

            let [display_pos_x, display_pos_y] = draw_data.display_pos;
            let [display_size_w, display_size_h] = draw_data.display_size;
            let frame_buffer_width = display_pos_x + display_size_w;
            let frame_buffer_height = display_pos_y + display_size_h;
            gl::Viewport(display_pos_x as i32, display_pos_y as i32, frame_buffer_width as i32, frame_buffer_height as i32);
            let ortho = nalgebra_glm::ortho(
                display_pos_x, display_pos_x + display_size_w,
                display_pos_y + display_size_h, display_pos_y, -1f32, 1f32);
            self.program.set_used();
            gl::Uniform1i(1, 0);
            gl::UniformMatrix4fv(0, 1, gl::FALSE, nalgebra_glm::value_ptr(&ortho).as_ptr());

            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);

            for draw_list in draw_data.draw_lists() {
                let vtx_buffer = draw_list.vtx_buffer();
                let idx_buffer = draw_list.idx_buffer();
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

                            let vtx_offset = cmd_params.vtx_offset;
                            let idx_offset = cmd_params.idx_offset;
                            if clip_rect[0] < frame_buffer_width && clip_rect[1] < frame_buffer_height && clip_rect[2] >= 0f32 && clip_rect[3] >= 0f32 {
                                gl::Scissor(clip_rect[0] as GLint,
                                            (frame_buffer_height - clip_rect[3]) as GLint,
                                            (clip_rect[2] - clip_rect[0]) as GLint,
                                            (clip_rect[3] - clip_rect[1]) as GLint);

                                gl::BindTexture(gl::TEXTURE_2D, gl::types::GLuint::try_from(cmd_params.texture_id.id()).unwrap_unchecked());
                                let gl_type = match std::mem::size_of::<imgui::DrawIdx>() {
                                    2 => gl::UNSIGNED_SHORT,
                                    _ => gl::UNSIGNED_INT
                                };
                                gl::DrawElementsBaseVertex(gl::TRIANGLES, GLsizei::try_from(count * 3).unwrap_unchecked(),
                                                 gl_type, idx_offset as *const c_void, vtx_offset as GLint);
                            }
                        }
                        x => {
                            panic!("Unimplemented! {:?}", x.type_id());
                        }
                    }
                    //                    gl::DrawElements(gl::TRIANGLES, command.ElemCount, gl::UNSIGNED_SHORT, std::ptr::null());
                }
            }

            gl::Disable(gl::BLEND);
            gl::Enable(gl::CULL_FACE);
            gl::Disable(gl::SCISSOR_TEST);
            gl::PopDebugGroup();
        }
    }
}

fn generate_font_texture_from_atlas(font_atlas: &mut FontAtlasRefMut) -> Texture {
    let font_atlas_texture = &mut font_atlas.build_rgba32_texture();
    let font_texture = Texture::from_raw(font_atlas_texture.data,
                                         font_atlas_texture.width as usize,
                                         font_atlas_texture.height as usize)
        .expect("Failed to create font texture for Dear ImGui");
    font_atlas.tex_id = TextureId::new(font_texture.handle() as usize);
    font_texture
}

fn create_program() -> Program {
    let _vertex_shader = Shader::from_source(IMGUI_VERTEX_SHADER_SOURCE, ShaderKind::Vertex)
        .expect("Failed to setup Dear ImGui vertex shader");
    let _fragment_shader = Shader::from_source(IMGUI_FRAGMENT_SHADER_SOURCE, ShaderKind::Fragment)
        .expect("Failed to setup Dear ImGui fragment shader");
    Program::from_shaders(&[&_vertex_shader, &_fragment_shader])
        .expect("Failed to setup Dear ImGui program")
}
