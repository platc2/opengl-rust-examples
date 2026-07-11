use renderer::Labelled;
use alloc::borrow::Cow;
use anyhow::Context;
use imgui::Ui;
use std::f32::consts::PI;
use std::path::Path;
use std::time::Instant;

use gl::sys::types::{GLfloat, GLintptr, GLsizei};
use renderer::application::{App, Application};
use renderer::input::InputManager;
use renderer::resources::Resources;
use renderer::time::Time;
use renderer::{
    Buffer, BufferUsage, RenderPass, Shader, ShaderKind, Texture, VertexAttribute, VertexAttributeBinding,
};

use crate::{
    initialize_cube_vertices, initialize_indices, initialize_vertices, KernelMatrix, Mat3, Vec3,
};

pub struct SobelCube {
    texture_fraction: f32,
    rotate: bool,
    matrix_index: usize,
    matrices: Vec<KernelMatrix>,
    light_color: glm::Vec3,
    angle: f32,
    projection: glm::Mat4,
    view: glm::Mat4,
    matrix_buffer: Buffer,
    kernel_buffer: Buffer,
    texture_switch_buffer: Buffer,
    light_buffer: Buffer,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    cube_vertices: Buffer,
    render_texture: Texture,
    main_render_pass: RenderPass,
    cube_render_pass: RenderPass,
    delta: f32,
    quit: bool,
}

impl App for SobelCube {
    fn new() -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        type Mat4 = nalgebra_glm::TMat4<f32>;

        unsafe {
            gl::sys::Enable(gl::sys::DEBUG_OUTPUT);
            gl::sys::Enable(gl::sys::DEBUG_OUTPUT_SYNCHRONOUS);
        }

        let res = Resources::from_relative_exe_path(Path::new("assets"))?;

        let vertex_buffer = initialize_vertices()?;
        let index_buffer = initialize_indices()?;

        let vertex_shader =
            Shader::from_source(&res.load_string("/shaders/basic.vert")?, ShaderKind::Vertex)
                .context("Failed to initialize basic vertex shader")?;
        let fragment_shader = Shader::from_source(
            &res.load_string("/shaders/basic.frag")?,
            ShaderKind::Fragment,
        )
        .context("Failed to initialize basic fragment shader")?;

        let matrix_buffer =
            Buffer::allocate(BufferUsage::Uniform, std::mem::size_of::<Mat4>() * 2)?;
        let texture_switch_buffer =
            Buffer::allocate(BufferUsage::Uniform, std::mem::size_of::<f32>())?;
        let light_buffer = Buffer::allocate(BufferUsage::Uniform, std::mem::size_of::<Vec3>())?;
        let kernel_buffer = Buffer::allocate(BufferUsage::Uniform, std::mem::size_of::<Mat3>())?;

        let vertex_bindings = [
            VertexAttributeBinding::new(
                0,
                VertexAttribute::new(renderer::VertexAttributeFormat::RGB32F, 0),
            ),
            VertexAttributeBinding::new(
                1,
                VertexAttribute::new(renderer::VertexAttributeFormat::RG32F, 0),
            ),
            VertexAttributeBinding::new(
                2,
                VertexAttribute::new(renderer::VertexAttributeFormat::RGB32F, 0),
            ),
        ];

        let cube_texture = Texture::from(&mut res.load_image("/textures/cube.tga")?)?;
        let floor_texture = Texture::from(&mut res.load_image("/textures/floor.tga")?)?;

        let mut render_texture = Texture::blank(1024, 1024);
        render_texture.set_label("Render texture");

        let mut main_render_pass = RenderPass::new(
            &vertex_shader,
            &fragment_shader,
            &vertex_bindings,
            &[&matrix_buffer, &texture_switch_buffer, &light_buffer],
            &[&cube_texture, &floor_texture],
            &[&render_texture],
        )?;
        main_render_pass.set_label("main_render_pass");

        let cube_vertices = initialize_cube_vertices()?;
        let cube_vertex_bindings = [
            VertexAttributeBinding::new(
                0,
                VertexAttribute::new(renderer::VertexAttributeFormat::RG32F, 0),
            ),
            VertexAttributeBinding::new(
                1,
                VertexAttribute::new(renderer::VertexAttributeFormat::RG32F, 0),
            ),
        ];
        let cube_vertex_shader =
            Shader::from_source(&res.load_string("/shaders/cube.vert")?, ShaderKind::Vertex)
                .context("Failed to initialize cube vertex shader")?;
        let cube_fragment_shader = Shader::from_source(
            &res.load_string("/shaders/cube.frag")?,
            ShaderKind::Fragment,
        )
        .context("Failed to initialize cube fragment shader")?;
        let mut cube_render_pass = RenderPass::new(
            &cube_vertex_shader,
            &cube_fragment_shader,
            &cube_vertex_bindings,
            &[&kernel_buffer],
            &[&render_texture],
            &[],
        )?;
        cube_render_pass.set_label("cube_render_pass");

        unsafe {
            gl::sys::Enable(gl::sys::DEPTH_TEST);
            gl::sys::DepthFunc(gl::sys::LESS);
        }

        unsafe {
            gl::sys::Enable(gl::sys::CULL_FACE);
            gl::sys::FrontFace(gl::sys::CCW);
            gl::sys::CullFace(gl::sys::BACK);

            gl::sys::Enable(gl::sys::DEPTH_TEST);
            gl::sys::DepthFunc(gl::sys::LEQUAL);
        }

        let matrices = [
            KernelMatrix {
                label: String::from("Identity"),
                matrix: nalgebra_glm::mat3(0f32, 0f32, 0f32, 0f32, 1f32, 0f32, 0f32, 0f32, 0f32),
            },
            KernelMatrix {
                label: String::from("Sobel Filter"),
                matrix: nalgebra_glm::mat3(
                    -1f32, -1f32, -1f32, -1f32, 8f32, -1f32, -1f32, -1f32, -1f32,
                ),
            },
            KernelMatrix {
                label: String::from("Sharpen"),
                matrix: nalgebra_glm::mat3(
                    0f32, -1f32, 0f32, -1f32, 5f32, -1f32, 0f32, -1f32, 0f32,
                ),
            },
            KernelMatrix {
                label: String::from("Box Blur"),
                matrix: nalgebra_glm::mat3(
                    1f32 / 9f32,
                    1f32 / 9f32,
                    1f32 / 9f32,
                    1f32 / 9f32,
                    1f32 / 9f32,
                    1f32 / 9f32,
                    1f32 / 9f32,
                    1f32 / 9f32,
                    1f32 / 9f32,
                ),
            },
            KernelMatrix {
                label: String::from("Gaussian Blur"),
                matrix: nalgebra_glm::mat3(
                    1f32 / 16f32,
                    2f32 / 16f32,
                    1f32 / 16f32,
                    2f32 / 16f32,
                    4f32 / 16f32,
                    2f32 / 16f32,
                    1f32 / 16f32,
                    2f32 / 16f32,
                    1f32 / 16f32,
                ),
            },
        ];

        Ok(Self {
            texture_fraction: 0.,
            rotate: false,
            matrix_index: 0,
            matrices: Vec::from(matrices),
            light_color: glm::vec3(1., 1., 1.),
            angle: 0.,
            projection: glm::perspective(1., PI / 3., 0.001, 100.),
            view: glm::look_at(
                &glm::vec3(0., 0., 4.),
                &glm::vec3(0., 0., 0.),
                &glm::vec3(0., 1., 0.),
            ),
            matrix_buffer,
            kernel_buffer,
            texture_switch_buffer,
            light_buffer,
            vertex_buffer,
            index_buffer,
            cube_vertices,
            render_texture,
            main_render_pass,
            cube_render_pass,
            delta: 1.,
            quit: false,
        })
    }
}

impl Application for SobelCube {
    fn tick(&mut self, time: &Time<Instant>, _: &dyn InputManager) {
        self.angle += (0.0005 * time.duration().as_millis() as f32);
        let view_projection = self.projection * self.view;
        let model = nalgebra_glm::rotation(self.angle, &nalgebra_glm::vec3(1.5f32, 1f32, 0.5f32));
        let matrix_ptr = self.matrix_buffer.map::<glm::Mat4>();
        matrix_ptr.copy_from_slice(&[model, view_projection]);
        self.matrix_buffer.unmap();

        let texture_switch_ptr = self.texture_switch_buffer.map::<f32>();
        texture_switch_ptr.copy_from_slice(&[self.texture_fraction]);
        self.texture_switch_buffer.unmap();

        let kernel_ptr = self.kernel_buffer.map::<Mat3>();
        kernel_ptr.copy_from_slice(&[self.matrices[self.matrix_index].matrix]);
        self.kernel_buffer.unmap();

        let light_ptr = self.light_buffer.map::<Vec3>();
        light_ptr.copy_from_slice(&[self.light_color]);
        self.light_buffer.unmap();

        if self.rotate {
            self.texture_fraction = 0.0025f32.mul_add(
                self.delta * time.duration().as_secs_f32(),
                self.texture_fraction,
            );
            let new = self.texture_fraction.clamp(0f32, 1f32);
            if (self.texture_fraction - new).abs() > 1e-9 {
                self.delta = -self.delta;
            }
            self.texture_fraction = new;
        }

        unsafe {
            self.main_render_pass.display();
            clear_screen(0.3, 0.3, 0.5);
            clear_screen(0.0, 0.0, 0.0);
            gl::sys::Viewport(
                0,
                0,
                GLsizei::try_from(self.render_texture.width()).unwrap_unchecked(),
                GLsizei::try_from(self.render_texture.height()).unwrap_unchecked(),
            );
            gl::sys::Enable(gl::sys::DEPTH_TEST);
            gl::sys::DepthFunc(gl::sys::LEQUAL);

            gl::sys::BindVertexBuffer(
                0,
                self.vertex_buffer.handle(),
                0 as GLintptr,
                GLsizei::try_from(std::mem::size_of::<f32>() * 3).unwrap(),
            );
            gl::sys::BindVertexBuffer(
                1,
                self.vertex_buffer.handle(),
                GLintptr::try_from(std::mem::size_of::<f32>() * 72).unwrap(),
                GLsizei::try_from(std::mem::size_of::<f32>() * 2).unwrap(),
            );
            gl::sys::BindVertexBuffer(
                2,
                self.vertex_buffer.handle(),
                GLintptr::try_from(std::mem::size_of::<f32>() * 120).unwrap(),
                GLsizei::try_from(std::mem::size_of::<f32>() * 3).unwrap(),
            );
            gl::sys::BindBuffer(gl::sys::ELEMENT_ARRAY_BUFFER, self.index_buffer.handle());
            let count =
                GLsizei::try_from(self.index_buffer.size() / std::mem::size_of::<u16>()).unwrap();
            gl::sys::DrawElements(
                gl::sys::TRIANGLES,
                count,
                gl::sys::UNSIGNED_SHORT,
                std::ptr::null(),
            );

            self.cube_render_pass.display();
            gl::sys::Disable(gl::sys::DEPTH_TEST);
            gl::sys::Clear(gl::sys::COLOR_BUFFER_BIT);
            gl::sys::Viewport(0, 0, 900, 700);
            gl::sys::BindVertexBuffer(
                0,
                self.cube_vertices.handle(),
                0 as GLintptr,
                GLsizei::try_from(std::mem::size_of::<f32>() * 2).unwrap(),
            );
            gl::sys::BindVertexBuffer(
                1,
                self.cube_vertices.handle(),
                GLintptr::try_from(std::mem::size_of::<f32>() * 12).unwrap(),
                GLsizei::try_from(std::mem::size_of::<f32>() * 2).unwrap(),
            );
            gl::sys::DrawArrays(gl::sys::TRIANGLES, 0, 6);
        }
    }

    fn gui(&mut self, ui: &Ui) {
        ui.window("Settings")
            .save_settings(false)
            .always_auto_resize(true)
            .build(|| {
                ui.slider("Texture Switch", 0f32, 1f32, &mut self.texture_fraction);
                ui.same_line();
                ui.checkbox("Cycle", &mut self.rotate);
                ui.combo(
                    "Kernel",
                    &mut self.matrix_index,
                    &self.matrices,
                    |kernel_matrix| Cow::from(&kernel_matrix.label),
                );
                ui.input_float3("Light", self.light_color.as_mut()).build();
            });
    }

    fn quit(&self) -> bool {
        self.quit
    }
}

fn clear_screen(red: f32, green: f32, blue: f32) {
    unsafe {
        gl::sys::ClearColor(
            red as GLfloat,
            green as GLfloat,
            blue as GLfloat,
            1f32 as GLfloat,
        );
        gl::sys::Clear(gl::sys::COLOR_BUFFER_BIT | gl::sys::DEPTH_BUFFER_BIT);
    }
}
