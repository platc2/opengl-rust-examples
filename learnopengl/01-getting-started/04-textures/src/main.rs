extern crate anyhow;
extern crate learnopengl_utils as utils;
extern crate nalgebra_glm as glm;
extern crate renderer;
extern crate stb_image;

use std::time::Instant;

use anyhow::Result;

use renderer::application;
use renderer::application::Application;
use renderer::input_manager::InputManager;
use renderer::renderer_context::{OpenGLVersion, RendererContext, WindowDimension};
use renderer::time::Time;
use utils::gl;

struct State {
    shader_program: gl::ProgramId,
    vertex_array_object: gl::VertexArrayId,
    container_texture: gl::TextureId,
    face_texture: gl::TextureId,

    texture_factor: f32,
    flip_face: bool,
    texture_scale: f32,
}

impl Application for State {
    fn tick(&mut self, _: &Time<Instant>, _input_manager: &dyn InputManager) {
        gl::viewport((0, 0), (800, 600));

        gl::clear_color(0xFF334C4C);
        gl::clear(gl::ClearMask::COLOR_BUFFER_BIT);

        gl::active_texture(gl::TextureUnit::fixed(0));
        gl::bind_texture(gl::TextureTarget::TEXTURE_2D, self.container_texture);

        gl::active_texture(gl::TextureUnit::fixed(1));
        gl::bind_texture(gl::TextureTarget::TEXTURE_2D, self.face_texture);

        gl::use_program(self.shader_program);

        gl::uniform_1i(gl::uniform_location(self.shader_program, "texture1"), 0);
        gl::uniform_1i(gl::uniform_location(self.shader_program, "texture2"), 1);
        gl::uniform_1f(gl::uniform_location(self.shader_program, "texture_factor"), self.texture_factor);
        gl::uniform_1i(gl::uniform_location(self.shader_program, "flip_face"), if self.flip_face { 1 } else { 0 });
        gl::uniform_1f(gl::uniform_location(self.shader_program, "texture_scale"), self.texture_scale);

        gl::bind_vertex_array(self.vertex_array_object);
        gl::draw_elements::<u8>(gl::DrawMode::TRIANGLES, 6, gl::IndexType::UNSIGNED_BYTE, None);
    }

    fn gui(&mut self, ui: &imgui::Ui) {
        ui.window("Settings")
            .save_settings(false)
            .always_auto_resize(true)
            .build(|| {
                ui.slider("Texture mix factor", 0., 1., &mut self.texture_factor);
                ui.checkbox("Flip face horizontally", &mut self.flip_face);
                ui.slider("Texture scale", 0.25, 4., &mut self.texture_scale);
            });
    }
}

impl State {
    pub fn new(shader_program: gl::ProgramId, vertex_array_object: gl::VertexArrayId, container_texture: gl::TextureId, face_texture: gl::TextureId) -> Self {
        Self {
            shader_program,
            vertex_array_object,
            container_texture,
            face_texture,

            texture_factor: 0.2,
            flip_face: false,
            texture_scale: 1.,
        }
    }
}

pub fn main() -> Result<()> {
    let context = RendererContext::init(
        "LearnOpenGL",
        &WindowDimension::of(800, 600),
        &OpenGLVersion::of(3, 3),
    )?;

    let vertex_data: [f32; 32] = [
        0.5, 0.5, 0., 1., 0., 0., 1., 1.,
        0.5, -0.5, 0., 0., 1., 0., 1., 0.,
        -0.5, -0.5, 0., 0., 0., 1., 0., 0.,
        -0.5, 0.5, 0., 1., 1., 0., 0., 1.
    ];
    let index_data: Vec<u8> = vec![
        0, 1, 3,  // first triangle
        1, 2, 3,  // second triangle
    ];

    let vertex_array_object = gl::create_vertex_array();
    gl::bind_vertex_array(vertex_array_object);

    let triangle_vbo = gl::create_buffer();
    gl::bind_buffer(gl::BufferTarget::ARRAY_BUFFER, triangle_vbo);
    gl::buffer_data(gl::BufferTarget::ARRAY_BUFFER, &vertex_data, gl::BufferUsage::STATIC_DRAW);

    let triangle_ebo = gl::create_buffer();
    gl::bind_buffer(gl::BufferTarget::ELEMENT_ARRAY_BUFFER, triangle_ebo);
    gl::buffer_data(gl::BufferTarget::ELEMENT_ARRAY_BUFFER, &index_data[..], gl::BufferUsage::STATIC_DRAW);

    gl::vertex_attrib_pointer(
        0,
        gl::ComponentSize::SIZE_3,
        gl::ComponentType::FLOAT,
        false,
        size_of::<f32>() * 8,
        0);
    gl::enable_vertex_attrib_array(0);
    gl::vertex_attrib_pointer(
        1,
        gl::ComponentSize::SIZE_3,
        gl::ComponentType::FLOAT,
        false,
        size_of::<f32>() * 8,
        size_of::<f32>() * 3,
    );
    gl::enable_vertex_attrib_array(1);
    gl::vertex_attrib_pointer(
        2,
        gl::ComponentSize::SIZE_2,
        gl::ComponentType::FLOAT,
        false,
        size_of::<f32>() * 8,
        size_of::<f32>() * 6,
    );
    gl::enable_vertex_attrib_array(2);
    gl::bind_vertex_array(gl::VertexArrayId::NO_VERTEX_ARRAY);

    let shader_program = utils::program(
        include_str!("../assets/triangle.vert"),
        include_str!("../assets/triangle.frag"),
    )?;

    let container_texture = utils::load_texture_2d(include_bytes!("../assets/container.jpg"))?;
    let face_texture = utils::load_texture_2d(include_bytes!("../assets/awesomeface.png"))?;
    gl::tex_parameter_iuiv(gl::TextureTarget::TEXTURE_2D, gl::TextureParameter::TEXTURE_WRAP_S, &[gl::sys::MIRRORED_REPEAT]);
    gl::tex_parameter_iuiv(gl::TextureTarget::TEXTURE_2D, gl::TextureParameter::TEXTURE_WRAP_T, &[gl::sys::REPEAT]);

    let state = State::new(shader_program, vertex_array_object, container_texture, face_texture);

    application::main_loop(context, state)
}
