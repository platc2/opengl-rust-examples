extern crate anyhow;
extern crate nalgebra_glm as glm;
extern crate renderer;
extern crate stb_image;

use std::time::Instant;

use anyhow::{anyhow, Result};
use stb_image::image::LoadResult;

use renderer::application;
use renderer::application::Application;
use renderer::input_manager::{InputManager, Key};
use renderer::renderer_context::{OpenGLVersion, RendererContext, WindowDimension};
use renderer::time::Time;

struct State {
    shader_program: gl::ProgramId,
    vertex_array_object: gl::VertexArrayId,
    container_texture: gl::TextureId,
    face_texture: gl::TextureId,

    texture_factor: f32,
    flip_face: bool,
    texture_scale: f32,

    quit: bool,
}

mod gl {
    pub use renderer::gl::buffer::*;
    pub use renderer::gl::framebuffer::*;
    pub use renderer::gl::image_format::*;
    pub use renderer::gl::pixel_format::*;
    pub use renderer::gl::pixel_type::*;
    pub use renderer::gl::program::*;
    pub use renderer::gl::rendering::*;
    pub use renderer::gl::shader::*;
    pub use renderer::gl::state::*;
    pub use renderer::gl::sys;
    pub use renderer::gl::texture::*;
    pub use renderer::gl::vertex_array::*;
    pub use renderer::gl::vertex_attrib::*;
}

impl Application for State {
    fn tick(&mut self, time: &Time<Instant>, input_manager: &dyn InputManager) {
        if input_manager.key_down(Key::ESCAPE) {
            self.quit = true;
        }

        gl::viewport((0, 0), (800, 600));

        gl::clear_color(0xFF334C4C);
        gl::clear(gl::ClearMask::COLOR_BUFFER_BIT);

        gl::active_texture(gl::TextureUnit::fixed(0));
        gl::bind_texture(gl::TextureTarget::TEXTURE_2D, self.container_texture);

        gl::active_texture(gl::TextureUnit::fixed(1));
        gl::bind_texture(gl::TextureTarget::TEXTURE_2D, self.face_texture);

        gl::use_program(self.shader_program);

        let t = time.duration_since_start().as_secs_f32();
        let transform = glm::Mat4::identity();
        let transform = glm::translate(&transform, &glm::vec3(0.5, -0.5, 0.));
        let transform = glm::rotate(&transform, t, &glm::vec3(0., 0., 1.));
        let transform = glm::scale(&transform, &glm::vec3(0.5, 0.5, 0.5));
        gl::uniform_matrix_4fv(gl::uniform_location(self.shader_program, "transform"), false, glm::value_ptr(&transform));
        gl::uniform_1i(gl::uniform_location(self.shader_program, "texture1"), 0);
        gl::uniform_1i(gl::uniform_location(self.shader_program, "texture2"), 1);
        gl::uniform_1f(gl::uniform_location(self.shader_program, "texture_factor"), self.texture_factor);
        gl::uniform_1i(gl::uniform_location(self.shader_program, "flip_face"), if self.flip_face { 1 } else { 0 });
        gl::uniform_1f(gl::uniform_location(self.shader_program, "texture_scale"), self.texture_scale);

        gl::bind_vertex_array(self.vertex_array_object);
        gl::draw_elements::<u8>(gl::DrawMode::TRIANGLES, 6, gl::IndexType::UNSIGNED_BYTE, None);

        let transform = glm::Mat4::identity();
        let transform = glm::translate(&transform, &glm::vec3(0.5, 0.5, 0.));
        let t = (t * 3.).sin();
        let transform = glm::scale(&transform, &glm::vec3(t, 1., 1.));
        let transform = glm::scale(&transform, &glm::vec3(0.5, 0.5, 0.5));
        gl::uniform_matrix_4fv(gl::uniform_location(self.shader_program, "transform"), false, glm::value_ptr(&transform));
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

    fn quit(&self) -> bool { self.quit }
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

            quit: false,
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
        core::mem::size_of::<f32>() * 8,
        0);
    gl::enable_vertex_attrib_array(0);
    gl::vertex_attrib_pointer(
        1,
        gl::ComponentSize::SIZE_3,
        gl::ComponentType::FLOAT,
        false,
        core::mem::size_of::<f32>() * 8,
        core::mem::size_of::<f32>() * 3,
    );
    gl::enable_vertex_attrib_array(1);
    gl::vertex_attrib_pointer(
        2,
        gl::ComponentSize::SIZE_2,
        gl::ComponentType::FLOAT,
        false,
        core::mem::size_of::<f32>() * 8,
        core::mem::size_of::<f32>() * 6,
    );
    gl::enable_vertex_attrib_array(2);
    gl::bind_vertex_array(gl::VertexArrayId::NO_VERTEX_ARRAY);

    let shader_program = program(
        include_str!("../assets/triangle.vert"),
        include_str!("../assets/triangle.frag"),
    )?;

    let container_texture = gl::create_texture(gl::TextureTarget::TEXTURE_2D);
    gl::bind_texture(gl::TextureTarget::TEXTURE_2D, container_texture);

    let texture_bytes = include_bytes!("../assets/container.jpg");
    unsafe {
        // Hack because the wrapper library does not support setting this parameter (yet)
        stb_image::stb_image::stbi_set_flip_vertically_on_load(1);
    }
    let image_data = stb_image::image::load_from_memory(texture_bytes);
    match image_data {
        LoadResult::Error(e) => panic!("Failed to load image: {}", e),
        LoadResult::ImageU8(image_data) => gl::tex_image_2d(
            gl::TextureTarget::TEXTURE_2D,
            0,
            gl::ImageFormat::RGB,
            (image_data.width, image_data.height),
            0,
            gl::PixelFormat::RGB,
            gl::PixelType::UNSIGNED_BYTE,
            &image_data.data[..],
        ),
        LoadResult::ImageF32(image_data) => gl::tex_image_2d(
            gl::TextureTarget::TEXTURE_2D,
            0,
            gl::ImageFormat::RGB,
            (image_data.width, image_data.height),
            0,
            gl::PixelFormat::RGB,
            gl::PixelType::FLOAT,
            image_data.data.as_slice(),
        )
    }
    gl::tex_parameter_iuiv(gl::TextureTarget::TEXTURE_2D, gl::TextureParameter::TEXTURE_MIN_FILTER, &[gl::sys::NEAREST]);
    gl::tex_parameter_iuiv(gl::TextureTarget::TEXTURE_2D, gl::TextureParameter::TEXTURE_MAG_FILTER, &[gl::sys::NEAREST]);
    gl::tex_parameter_iuiv(gl::TextureTarget::TEXTURE_2D, gl::TextureParameter::TEXTURE_WRAP_S, &[gl::sys::CLAMP_TO_EDGE]);
    gl::tex_parameter_iuiv(gl::TextureTarget::TEXTURE_2D, gl::TextureParameter::TEXTURE_WRAP_T, &[gl::sys::CLAMP_TO_EDGE]);
    gl::generate_mipmap(gl::TextureTarget::TEXTURE_2D);

    let face_texture = gl::create_texture(gl::TextureTarget::TEXTURE_2D);
    gl::bind_texture(gl::TextureTarget::TEXTURE_2D, face_texture);

    let texture_bytes = include_bytes!("../assets/awesomeface.png");
    let image_data = stb_image::image::load_from_memory(texture_bytes);
    match image_data {
        LoadResult::Error(e) => panic!("Failed to load image: {}", e),
        LoadResult::ImageU8(image_data) => gl::tex_image_2d(
            gl::TextureTarget::TEXTURE_2D,
            0,
            gl::ImageFormat::RGB,
            (image_data.width, image_data.height),
            0,
            gl::PixelFormat::RGBA,
            gl::PixelType::UNSIGNED_BYTE,
            &image_data.data[..],
        ),
        LoadResult::ImageF32(image_data) => gl::tex_image_2d(
            gl::TextureTarget::TEXTURE_2D,
            0,
            gl::ImageFormat::RGB,
            (image_data.width, image_data.height),
            0,
            gl::PixelFormat::RGBA,
            gl::PixelType::FLOAT,
            image_data.data.as_slice(),
        )
    }
    gl::tex_parameter_iuiv(gl::TextureTarget::TEXTURE_2D, gl::TextureParameter::TEXTURE_MIN_FILTER, &[gl::sys::NEAREST]);
    gl::tex_parameter_iuiv(gl::TextureTarget::TEXTURE_2D, gl::TextureParameter::TEXTURE_MAG_FILTER, &[gl::sys::NEAREST]);
    gl::tex_parameter_iuiv(gl::TextureTarget::TEXTURE_2D, gl::TextureParameter::TEXTURE_WRAP_S, &[gl::sys::MIRRORED_REPEAT]);
    gl::tex_parameter_iuiv(gl::TextureTarget::TEXTURE_2D, gl::TextureParameter::TEXTURE_WRAP_T, &[gl::sys::REPEAT]);
    gl::generate_mipmap(gl::TextureTarget::TEXTURE_2D);

    let state = State::new(shader_program, vertex_array_object, container_texture, face_texture);

    application::main_loop(context, state)
}

fn shader(shader_kind: gl::ShaderKind, shader_source: &str) -> Result<gl::ShaderId> {
    let shader = gl::create_shader(shader_kind);
    gl::shader_source(shader, shader_source);
    gl::compile_shader(shader);
    if gl::shader_compile_status(shader) {
        Ok(shader)
    } else {
        let info_log = gl::shader_info_log(shader);
        Err(anyhow!("Error compiling {shader_kind:?} shader: {}", info_log.unwrap_or("Unknown error".to_owned())))
    }
}

fn program(
    vertex_shader_source: &str,
    fragment_shader_source: &str,
) -> Result<gl::ProgramId> {
    let program = gl::create_program();
    let mut vertex_shader = shader(gl::ShaderKind::VERTEX_SHADER, vertex_shader_source)?;
    let mut fragment_shader = shader(gl::ShaderKind::FRAGMENT_SHADER, fragment_shader_source)?;
    gl::attach_shader(program, vertex_shader);
    gl::attach_shader(program, fragment_shader);
    gl::link_program(program);

    gl::delete_shader(&mut vertex_shader);
    gl::delete_shader(&mut fragment_shader);

    if gl::program_link_status(program) {
        Ok(program)
    } else {
        let info_log = gl::program_info_log(program);
        Err(anyhow!("Failed to link shader program: {}", info_log.unwrap_or("Unknown error".to_owned())))
    }
}
