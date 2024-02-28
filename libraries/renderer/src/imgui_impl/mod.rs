extern crate imgui;

use std::any::Any;
use std::collections::HashMap;
use std::time::Duration;

use imgui::{BackendFlags, FontAtlas, TextureId};

use gl::sys::RawHandle;

use crate::input_manager::Key;
use crate::renderer::{Program, Shader, ShaderKind, Texture};


mod gl {
    pub use gl::buffer::*;
    pub use gl::capabilities::*;
    pub use gl::debug::*;
    pub use gl::program::*;
    pub use gl::rendering::*;
    pub use gl::shader::*;
    pub use gl::state::*;
    pub use gl::sys;
    pub use gl::texture::*;
    pub use gl::vertex_array::*;
    pub use gl::vertex_attrib::*;
}

const IMGUI_VERTEX_SHADER_SOURCE: &str = include_str!("shaders/imgui.vert");
const IMGUI_FRAGMENT_SHADER_SOURCE: &str = include_str!("shaders/imgui.frag");

const IMGUI_INDEX_TYPE: gl::IndexType = match std::mem::size_of::<imgui::DrawIdx>() {
    1 => gl::IndexType::UNSIGNED_BYTE,
    2 => gl::IndexType::UNSIGNED_SHORT,
    4 => gl::IndexType::UNSIGNED_INT,
    _ => panic!("Failed to evaluate index size for ImGui!"),
};

pub struct Imgui {
    context: imgui::Context,
    program: Program,
    vertex_array_object: gl::VertexArrayId,
    vertex_buffer_object: gl::BufferId,
    element_buffer_object: gl::BufferId,
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

        let vertex_buffer_object = gl::create_buffer();
        gl::bind_buffer(gl::BufferTarget::ARRAY_BUFFER, vertex_buffer_object);

        let element_buffer_object = gl::create_buffer();
        gl::bind_buffer(gl::BufferTarget::ELEMENT_ARRAY_BUFFER, element_buffer_object);

        let vertex_array_object = gl::create_vertex_array();
        gl::bind_vertex_array(vertex_array_object);
        gl::enable_vertex_attrib_array(0);
        gl::vertex_attrib_pointer(0, gl::ComponentSize::SIZE_2, gl::ComponentType::FLOAT, false, std::mem::size_of::<imgui::DrawVert>(), 0);
        gl::enable_vertex_attrib_array(1);
        gl::vertex_attrib_pointer(1, gl::ComponentSize::SIZE_2, gl::ComponentType::FLOAT, false, std::mem::size_of::<imgui::DrawVert>(), 2 * std::mem::size_of::<f32>());
        gl::enable_vertex_attrib_array(2);
        gl::vertex_attrib_pointer(2, gl::ComponentSize::SIZE_4, gl::ComponentType::UNSIGNED_BYTE, true, std::mem::size_of::<imgui::DrawVert>(), 4 * std::mem::size_of::<f32>());
        gl::bind_vertex_array(gl::VertexArrayId::NO_VERTEX_ARRAY);

        Self {
            context,
            program,
            vertex_array_object,
            vertex_buffer_object,
            element_buffer_object,
        }
    }

    pub fn want_capture_mouse(&self) -> bool {
        self.context.io().want_capture_mouse
    }

    pub fn prepare_unfocused(&mut self, window_dimension: WindowDimension, delta: Duration) {
        self.prepare(
            window_dimension,
            None,
            None,
            &HashMap::new(),
            &Vec::new(),
            delta
        );
    }

    pub fn prepare(
        &mut self,
        window_dimension: WindowDimension,
        mouse_pos: Option<MousePos>,
        mouse_button_state: Option<MouseButtonState>,
        key_changes: &HashMap<Key, bool>,
        text_input: &Vec<String>,
        delta: Duration,
    ) {
        let io = self.context.io_mut();
        io.display_size = window_dimension;
        io.delta_time = delta.as_secs_f32();

        if let Some(mouse_pos) = mouse_pos {
            io.mouse_pos = mouse_pos;
        }

        if let Some(mouse_button_state) = mouse_button_state {
            io.mouse_down[0] = mouse_button_state[0];
            io.mouse_down[1] = mouse_button_state[1];
        }

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

//        gl::push_debug_group(gl::DebugGroupSource::DEBUG_SOURCE_THIRD_PARTY, 2, "ImGui Rendering");

        let blend_enabled = gl::is_enabled(gl::Capability::BLEND);
        let cull_face_enabled = gl::is_enabled(gl::Capability::CULL_FACE);
        let depth_test_enabled = gl::is_enabled(gl::Capability::DEPTH_TEST);
        let scissor_test_enabled = gl::is_enabled(gl::Capability::SCISSOR_TEST);

        gl::enable(gl::Capability::BLEND);
        gl::blend_equation(gl::BlendEquation::FUNC_ADD);
        gl::blend_func(gl::BlendSourceFuncFactor::SRC_ALPHA, gl::BlendDestinationFuncFactor::ONE_MINUS_SRC_ALPHA);
        gl::disable(gl::Capability::CULL_FACE);
        gl::disable(gl::Capability::DEPTH_TEST);
        gl::disable(gl::Capability::SCISSOR_TEST);
        gl::polygon_mode(gl::PolygonFace::FRONT_AND_BACK, gl::PolygonMode::FILL);
        let [display_pos_x, display_pos_y] = draw_data.display_pos;
        let [display_size_w, display_size_h] = draw_data.display_size;
        let frame_buffer_width = display_pos_x + display_size_w;
        let frame_buffer_height = display_pos_y + display_size_h;
        gl::viewport(
            (display_pos_x as _, display_pos_y as _),
            (frame_buffer_width as _, frame_buffer_height as _),
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
        gl::uniform_1i(gl::UniformLocation::fixed(1), 0);
        gl::uniform_matrix_4fv(gl::UniformLocation::fixed(0), false, nalgebra_glm::value_ptr(&ortho));

        gl::bind_vertex_array(self.vertex_array_object);
        gl::bind_buffer(gl::BufferTarget::ELEMENT_ARRAY_BUFFER, self.element_buffer_object);

        gl::active_texture(gl::TextureUnit::fixed(0));
        for draw_list in draw_data.draw_lists() {
            let vtx_buffer = draw_list.vtx_buffer();
            let idx_buffer = draw_list.idx_buffer();
            gl::buffer_data(gl::BufferTarget::ARRAY_BUFFER, vtx_buffer, gl::BufferUsage::STREAM_DRAW);
            gl::buffer_data(gl::BufferTarget::ELEMENT_ARRAY_BUFFER, idx_buffer, gl::BufferUsage::STREAM_DRAW);

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
                            gl::scissor(
                                (clip_rect[0] as _, (frame_buffer_height - clip_rect[3]) as _),
                                ((clip_rect[2] - clip_rect[0]) as _, (clip_rect[3] - clip_rect[1]) as _));

                            let texture_id = unsafe { gl::TextureId::from_raw(cmd_params.texture_id.id() as _) };
                            gl::bind_texture(gl::TextureTarget::TEXTURE_2D, texture_id);
                            gl::draw_elements_base_vertex(
                                gl::DrawMode::TRIANGLES,
                                count,
                                IMGUI_INDEX_TYPE,
                                idx_offset,
                                vtx_offset,
                            );
                        }
                    }
                    x => {
                        panic!("Unimplemented! {:?}", x.type_id());
                    }
                }
            }
        }
        gl::bind_vertex_array(gl::VertexArrayId::NO_VERTEX_ARRAY);

        toggle_capability(blend_enabled)(gl::Capability::BLEND);
        toggle_capability(cull_face_enabled)(gl::Capability::CULL_FACE);
        toggle_capability(depth_test_enabled)(gl::Capability::DEPTH_TEST);
        toggle_capability(scissor_test_enabled)(gl::Capability::SCISSOR_TEST);

//        gl::pop_debug_group();
    }
}

fn toggle_capability(was_enabled: bool) -> fn(gl::Capability) {
    if was_enabled {
        gl::enable
    } else {
        gl::disable
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

impl Drop for Imgui {
    fn drop(&mut self) {
        gl::delete_vertex_array(&mut self.vertex_array_object);
        gl::delete_buffer(&mut self.vertex_buffer_object);
        gl::delete_buffer(&mut self.element_buffer_object);
    }
}
