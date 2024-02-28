use std::time::Instant;

use imgui::Ui;

use gl::sys::types::{GLintptr, GLsizei};
use renderer::{Buffer, RenderPass};
use renderer::application::Application;
use renderer::input_manager::{InputManager, Key};
use renderer::time::Time;

pub struct State {
    render_pass: RenderPass,
    gamma_buffer: Buffer,
    vertex_buffer: Buffer,
    gamma: f32,

    quit: bool,
}

impl State {
    pub fn new(render_pass: RenderPass, gamma_buffer: Buffer, vertex_buffer: Buffer) -> Self {
        Self {
            render_pass,
            gamma_buffer,
            vertex_buffer,
            gamma: 1f32,
            quit: false,
        }
    }
}

impl Application for State {
    fn tick(&mut self, _: &Time<Instant>, input_manager: &dyn InputManager) {
        if input_manager.key_down(Key::ESCAPE) {
            self.quit = true;
        }

        unsafe {
            self.render_pass.display();

            let gamma_ptr = self.gamma_buffer.map::<f32>();
            gamma_ptr.copy_from_slice(&[self.gamma]);
            self.gamma_buffer.unmap();

            gl::sys::Clear(gl::sys::COLOR_BUFFER_BIT);
            gl::sys::Viewport(0, 0, 900, 700);
            gl::sys::BindVertexBuffer(
                0,
                self.vertex_buffer.handle(),
                0 as GLintptr,
                GLsizei::try_from(std::mem::size_of::<f32>() * 5).unwrap(),
            );
            gl::sys::BindVertexBuffer(
                1,
                self.vertex_buffer.handle(),
                GLintptr::try_from(std::mem::size_of::<f32>() * 2).unwrap(),
                GLsizei::try_from(std::mem::size_of::<f32>() * 5).unwrap(),
            );
            gl::sys::DrawArrays(gl::sys::TRIANGLES, 0, 3);
        }
    }

    fn gui(&mut self, ui: &Ui) {
        ui.window("Settings")
            .save_settings(false)
            .always_auto_resize(true)
            .build(|| {
                ui.slider("Gamma", 0.5f32, 2.5f32, &mut self.gamma);
                if ui.button("Reset (1.0)") {
                    self.gamma = 1f32;
                }
                ui.same_line();
                if ui.button("Reset (2.2)") {
                    self.gamma = 2.2f32;
                }
            });
    }

    fn quit(&self) -> bool {
        self.quit
    }
}
