extern crate anyhow;
extern crate nalgebra_glm as glm;
extern crate renderer;
extern crate stb_image;

use std::fmt::{Display, Formatter};
use std::time::Instant;

use anyhow::{anyhow, Result};
use stb_image::image::LoadResult;

use camera::{Camera, MovementDirection};
use renderer::application;
use renderer::application::Application;
use renderer::input_manager::{InputManager, Key};
use renderer::renderer_context::{OpenGLVersion, RendererContext, WindowDimension};
use renderer::time::Time;

mod camera;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum LightType {
    Directional,
    Point,
    Spot,
}

impl Display for LightType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LightType::Directional => write!(f, "Directional"),
            LightType::Point => write!(f, "Point"),
            LightType::Spot => write!(f, "Spot"),
        }
    }
}

struct State {
    vertex_array_object: gl::VertexArrayId,
    cube_program: gl::ProgramId,
    light_program: gl::ProgramId,

    diffuse_texture: gl::TextureId,
    specular_texture: gl::TextureId,

    cubes: Vec<glm::Vec3>,

    shininess: f32,
    light_types: Vec<LightType>,
    light_type_selected: usize,

    camera: Camera,
}

mod gl {
    pub use renderer::gl::buffer::*;
    pub use renderer::gl::capabilities::*;
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
        if input_manager.key_down(Key::W) {
            self.camera.process_keyboard(MovementDirection::FORWARD, time);
        }
        if input_manager.key_down(Key::S) {
            self.camera.process_keyboard(MovementDirection::BACKWARD, time);
        }
        if input_manager.key_down(Key::A) {
            self.camera.process_keyboard(MovementDirection::LEFT, time);
        }
        if input_manager.key_down(Key::D) {
            self.camera.process_keyboard(MovementDirection::RIGHT, time);
        }

        let mouse_movement = input_manager.mouse_movement();
        self.camera.process_mouse_movement((mouse_movement.0 as _, -mouse_movement.1 as _), true);

        let (_, scroll_y) = input_manager.scroll();
        self.camera.process_mouse_scroll(scroll_y as _);

        gl::viewport((0, 0), (800, 600));
        gl::enable(gl::Capability::DEPTH_TEST);

        gl::clear_color(0xFF020202);
        gl::clear(gl::ClearMask::COLOR_BUFFER_BIT | gl::ClearMask::DEPTH_BUFFER_BIT);

        let projection = glm::perspective(800. / 600., self.camera.zoom().to_radians(), 0.1, 100.);
        let view = self.camera.view_matrix();

        gl::bind_vertex_array(self.vertex_array_object);

        gl::use_program(self.cube_program);
        gl::uniform_matrix_4fv(gl::uniform_location(self.cube_program, "projection"), false, glm::value_ptr(&projection));
        gl::uniform_matrix_4fv(gl::uniform_location(self.cube_program, "view"), false, glm::value_ptr(&view));
        gl::uniform_3fv(gl::uniform_location(self.cube_program, "viewPos"), glm::value_ptr(self.camera.position()));

        gl::active_texture(gl::TextureUnit::fixed(0));
        gl::bind_texture(gl::TextureTarget::TEXTURE_2D, self.diffuse_texture);
        gl::uniform_1i(gl::uniform_location(self.cube_program, "material.diffuse"), 0);

        gl::active_texture(gl::TextureUnit::fixed(1));
        gl::bind_texture(gl::TextureTarget::TEXTURE_2D, self.specular_texture);
        gl::uniform_1i(gl::uniform_location(self.cube_program, "material.specular"), 1);

        gl::uniform_1f(gl::uniform_location(self.cube_program, "material.shininess"), self.shininess);

        match self.light_types[self.light_type_selected] {
            LightType::Directional =>
                gl::uniform_3fv(gl::uniform_location(self.cube_program, "light.direction"), glm::value_ptr(&glm::vec3(-0.2, -1., -0.3))),
            LightType::Point => {
                gl::uniform_3fv(gl::uniform_location(self.cube_program, "light.position"), glm::value_ptr(&glm::vec3(1.2, 1., 2.)));
                gl::uniform_1f(gl::uniform_location(self.cube_program, "light.constant"), 1.);
                gl::uniform_1f(gl::uniform_location(self.cube_program, "light.linear"), 0.09);
                gl::uniform_1f(gl::uniform_location(self.cube_program, "light.quadratic"), 0.032);
            }
            LightType::Spot => {
                gl::uniform_3fv(gl::uniform_location(self.cube_program, "light.position"), glm::value_ptr(self.camera.position()));
                gl::uniform_3fv(gl::uniform_location(self.cube_program, "light.direction"), glm::value_ptr(self.camera.front()));
                gl::uniform_1f(gl::uniform_location(self.cube_program, "light.cutoff"), 12.5f32.to_radians().cos());
                gl::uniform_1f(gl::uniform_location(self.cube_program, "light.outerCutoff"), 17.5f32.to_radians().cos());
            }
        }

        gl::uniform_3fv(gl::uniform_location(self.cube_program, "light.ambient"), glm::value_ptr(&glm::vec3(0.2, 0.2, 0.2)));
        gl::uniform_3fv(gl::uniform_location(self.cube_program, "light.diffuse"), glm::value_ptr(&glm::vec3(0.5, 0.5, 0.5)));
        gl::uniform_3fv(gl::uniform_location(self.cube_program, "light.specular"), glm::value_ptr(&glm::vec3(1., 1., 1.)));
        gl::uniform_1ui(gl::uniform_location(self.cube_program, "light.type"), self.light_type_selected as _);

        for (index, position) in self.cubes.iter().enumerate() {
            let model = glm::translation(position);
            let angle = 20. * (index as f32);
            let model = glm::rotate(&model, angle.to_radians(), &glm::vec3(1., 0.3, 0.5));
            gl::uniform_matrix_4fv(gl::uniform_location(self.cube_program, "model"), false, glm::value_ptr(&model));
            gl::draw_arrays(gl::DrawMode::TRIANGLES, 0, 36);
        }

        if self.light_types[self.light_type_selected] == LightType::Point {
            gl::use_program(self.light_program);
            let model = glm::Mat4::identity();
            let model = glm::translate(&model, &glm::vec3(1.2, 1., 2.));
            let model = glm::scale(&model, &glm::vec3(0.2, 0.2, 0.2));
            gl::uniform_matrix_4fv(gl::uniform_location(self.light_program, "projection"), false, glm::value_ptr(&projection));
            gl::uniform_matrix_4fv(gl::uniform_location(self.light_program, "view"), false, glm::value_ptr(&view));
            gl::uniform_matrix_4fv(gl::uniform_location(self.light_program, "model"), false, glm::value_ptr(&model));
            gl::draw_arrays(gl::DrawMode::TRIANGLES, 0, 36);
        }
    }

    fn gui(&mut self, ui: &imgui::Ui) {
        ui.window("Settings")
            .save_settings(false)
            .always_auto_resize(true)
            .build(|| {
                ui.slider_config("Shininess", 1., 256.)
                    .flags(imgui::SliderFlags::LOGARITHMIC)
                    .build(&mut self.shininess);

                ui.combo("Light type", &mut self.light_type_selected, self.light_types.as_slice(), |lt| std::borrow::Cow::from(format!("{}", lt)));
            });
    }
}

impl State {
    pub fn new(vertex_array_object: gl::VertexArrayId,
               cube_program: gl::ProgramId,
               light_program: gl::ProgramId,
               diffuse_texture: gl::TextureId,
               specular_texture: gl::TextureId) -> Self {
        Self {
            vertex_array_object,
            cube_program,
            light_program,

            diffuse_texture,
            specular_texture,

            cubes: vec![
                glm::vec3(0., 0., 0.),
                glm::vec3(2., 5., -15.),
                glm::vec3(-1.5, -2.2, -2.5),
                glm::vec3(-3.8, -2., -12.3),
                glm::vec3(2.4, -0.4, -3.5),
                glm::vec3(-1.7, 3., -7.5),
                glm::vec3(1.3, -2., -2.5),
                glm::vec3(1.5, 2., -2.5),
                glm::vec3(1.5, 0.2, -1.5),
                glm::vec3(-1.3, 1., -1.5),
            ],

            shininess: 32.,
            light_types: vec![
                LightType::Directional,
                LightType::Point,
                LightType::Spot,
            ],
            light_type_selected: 0,

            camera: Camera::new(glm::vec3(0., 0., 3.), glm::vec3(0., 1., 0.), -90., 0.),
        }
    }
}

pub fn main() -> Result<()> {
    let context = RendererContext::init(
        "LearnOpenGL",
        &WindowDimension::of(800, 600),
        &OpenGLVersion::of(3, 3),
    )?;

    let vertex_data: [f32; 288] = [
        -0.5, -0.5, -0.5, 0., 0., -1., 0., 0.,
        0.5, -0.5, -0.5, 0., 0., -1., 1., 0.,
        0.5, 0.5, -0.5, 0., 0., -1., 1., 1.,
        0.5, 0.5, -0.5, 0., 0., -1., 1., 1.,
        -0.5, 0.5, -0.5, 0., 0., -1., 0., 1.,
        -0.5, -0.5, -0.5, 0., 0., -1., 0., 0.,
        -0.5, -0.5, 0.5, 0., 0., 1., 0., 0.,
        0.5, -0.5, 0.5, 0., 0., 1., 1., 0.,
        0.5, 0.5, 0.5, 0., 0., 1., 1., 1.,
        0.5, 0.5, 0.5, 0., 0., 1., 1., 1.,
        -0.5, 0.5, 0.5, 0., 0., 1., 0., 1.,
        -0.5, -0.5, 0.5, 0., 0., 1., 0., 0.,
        -0.5, 0.5, 0.5, -1., 0., 0., 1., 0.,
        -0.5, 0.5, -0.5, -1., 0., 0., 1., 1.,
        -0.5, -0.5, -0.5, -1., 0., 0., 0., 1.,
        -0.5, -0.5, -0.5, -1., 0., 0., 0., 1.,
        -0.5, -0.5, 0.5, -1., 0., 0., 0., 0.,
        -0.5, 0.5, 0.5, -1., 0., 0., 1., 0.,
        0.5, 0.5, 0.5, 1., 0., 0., 1., 0.,
        0.5, 0.5, -0.5, 1., 0., 0., 1., 1.,
        0.5, -0.5, -0.5, 1., 0., 0., 0., 1.,
        0.5, -0.5, -0.5, 1., 0., 0., 0., 1.,
        0.5, -0.5, 0.5, 1., 0., 0., 0., 0.,
        0.5, 0.5, 0.5, 1., 0., 0., 1., 0.,
        -0.5, -0.5, -0.5, 0., -1., 0., 0., 1.,
        0.5, -0.5, -0.5, 0., -1., 0., 1., 1.,
        0.5, -0.5, 0.5, 0., -1., 0., 1., 0.,
        0.5, -0.5, 0.5, 0., -1., 0., 1., 0.,
        -0.5, -0.5, 0.5, 0., -1., 0., 0., 0.,
        -0.5, -0.5, -0.5, 0., -1., 0., 0., 1.,
        -0.5, 0.5, -0.5, 0., 1., 0., 0., 1.,
        0.5, 0.5, -0.5, 0., 1., 0., 1., 1.,
        0.5, 0.5, 0.5, 0., 1., 0., 1., 0.,
        0.5, 0.5, 0.5, 0., 1., 0., 1., 0.,
        -0.5, 0.5, 0.5, 0., 1., 0., 0., 0.,
        -0.5, 0.5, -0.5, 0., 1., 0., 0., 1.
    ];

    let vertex_array_object = gl::create_vertex_array();
    gl::bind_vertex_array(vertex_array_object);

    let triangle_vbo = gl::create_buffer();
    gl::bind_buffer(gl::BufferTarget::ARRAY_BUFFER, triangle_vbo);
    gl::buffer_data(gl::BufferTarget::ARRAY_BUFFER, &vertex_data, gl::BufferUsage::STATIC_DRAW);

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

    let cube_program = program(
        include_str!("../assets/cube.vert"),
        include_str!("../assets/cube.frag"),
    )?;

    let light_program = program(
        include_str!("../assets/light.vert"),
        include_str!("../assets/light.frag"),
    )?;

    let diffuse_texture = gl::create_texture(gl::TextureTarget::TEXTURE_2D);
    gl::bind_texture(gl::TextureTarget::TEXTURE_2D, diffuse_texture);

    let texture_bytes = include_bytes!("../assets/container2.png");
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
    gl::tex_parameter_iuiv(gl::TextureTarget::TEXTURE_2D, gl::TextureParameter::TEXTURE_WRAP_S, &[gl::sys::CLAMP_TO_EDGE]);
    gl::tex_parameter_iuiv(gl::TextureTarget::TEXTURE_2D, gl::TextureParameter::TEXTURE_WRAP_T, &[gl::sys::CLAMP_TO_EDGE]);
    gl::generate_mipmap(gl::TextureTarget::TEXTURE_2D);

    let specular_texture = gl::create_texture(gl::TextureTarget::TEXTURE_2D);
    gl::bind_texture(gl::TextureTarget::TEXTURE_2D, specular_texture);

    let texture_bytes = include_bytes!("../assets/container2_specular.png");
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
    gl::tex_parameter_iuiv(gl::TextureTarget::TEXTURE_2D, gl::TextureParameter::TEXTURE_WRAP_S, &[gl::sys::CLAMP_TO_EDGE]);
    gl::tex_parameter_iuiv(gl::TextureTarget::TEXTURE_2D, gl::TextureParameter::TEXTURE_WRAP_T, &[gl::sys::CLAMP_TO_EDGE]);
    gl::generate_mipmap(gl::TextureTarget::TEXTURE_2D);

    let state = State::new(vertex_array_object, cube_program, light_program, diffuse_texture, specular_texture);

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
