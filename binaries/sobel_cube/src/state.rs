use alloc::borrow::Cow;
use std::f32::consts::PI;
use std::time::Instant;

use imgui::Ui;

use gl::sys::types::{GLfloat, GLintptr, GLsizei};
use renderer::{Buffer, RenderPass, Texture};
use renderer::application::Application;
use renderer::input_manager::{InputManager, Key};
use renderer::time::Time;

use crate::{KernelMatrix, Mat3, Vec3};

pub struct State {
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

impl State {
    pub fn new(matrices: Vec<KernelMatrix>,
               matrix_buffer: Buffer,
               kernel_buffer: Buffer,
               texture_switch_buffer: Buffer,
               light_buffer: Buffer,
               vertex_buffer: Buffer,
               index_buffer: Buffer,
               cube_vertices: Buffer,
               render_texture: Texture,
               main_render_pass: RenderPass,
               cube_render_pass: RenderPass) -> Self {
        Self {
            texture_fraction: 0.,
            rotate: false,
            matrix_index: 0,
            matrices,
            light_color: glm::vec3(1., 1., 0.),
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
        }
    }
}

impl Application for State {
    fn tick(&mut self, time: &Time<Instant>, input_manager: &dyn InputManager) {
        if input_manager.key_down(Key::ESCAPE) {
            self.quit = true;
        }

        self.angle += 0.005f32;
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
            self.texture_fraction = 0.0025f32.mul_add(self.delta * time.duration().as_secs_f32(), self.texture_fraction);
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
            gl::sys::DrawElements(gl::sys::TRIANGLES, count, gl::sys::UNSIGNED_SHORT, std::ptr::null());

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
                ui.combo("Kernel", &mut self.matrix_index, &self.matrices, |kernel_matrix| {
                    Cow::from(&kernel_matrix.label)
                });
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
