use std::time::Instant;

use gl::sys::types::{GLintptr, GLsizei};
use renderer::{Buffer, RenderPass};
use renderer::application::Application;
use renderer::input_manager::{InputManager, Key};
use renderer::time::Time;

use crate::TessellationParameters;

pub struct State {
    max_tessellation: gl::sys::types::GLenum,
    tessellation_parameters: TessellationParameters,
    main_render_pass: RenderPass,
    tessellation_parameters_buffer: Buffer,
    vertex_buffer: Buffer,
    quit: bool,
}

impl State {
    pub fn new(main_render_pass: RenderPass,
               tessellation_parameters_buffer: Buffer,
               vertex_buffer: Buffer) -> Self {
        Self {
            max_tessellation: std::cmp::min(gl::sys::MAX_TESS_GEN_LEVEL, 64),
            tessellation_parameters: TessellationParameters {
                outer: [1; 4 * 4],
                inner: [1; 2 * 4],
            },
            main_render_pass,
            tessellation_parameters_buffer,
            vertex_buffer,
            quit: false,
        }
    }
}

impl Application for State {
    fn tick(&mut self, time: &Time<Instant>, input_manager: &dyn InputManager) {
        if input_manager.key_down(Key::ESCAPE) {
            self.quit = true;
        }

        unsafe {
            gl::sys::PolygonMode(gl::sys::FRONT_AND_BACK, gl::sys::LINE);
        }

        self.main_render_pass.display();

        let tessellation_parameters_ptr =
            self.tessellation_parameters_buffer.map::<TessellationParameters>();
        tessellation_parameters_ptr.copy_from_slice(&[self.tessellation_parameters]);
        self.tessellation_parameters_buffer.unmap();

        unsafe {
            gl::sys::Clear(gl::sys::COLOR_BUFFER_BIT);
            gl::sys::Viewport(0, 0, 700, 700);
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
            gl::sys::PatchParameteri(gl::sys::PATCH_VERTICES, 3);
            gl::sys::DrawArrays(gl::sys::PATCHES, 0, 3);
        }
    }

    fn gui(&mut self, ui: &imgui::Ui) {
        ui.window("Settings")
            .save_settings(false)
            .always_auto_resize(true)
            .build(|| {
                ui.text("Tessellation parameters");
                ui.separator();
                ui.slider(
                    "Outer 0",
                    1,
                    self.max_tessellation,
                    &mut self.tessellation_parameters.outer[0 * 4],
                );
                ui.slider(
                    "Outer 1",
                    1,
                    self.max_tessellation,
                    &mut self.tessellation_parameters.outer[1 * 4],
                );
                ui.slider(
                    "Outer 2",
                    1,
                    self.max_tessellation,
                    &mut self.tessellation_parameters.outer[2 * 4],
                );
                ui.slider(
                    "Outer 3",
                    1,
                    self.max_tessellation,
                    &mut self.tessellation_parameters.outer[3 * 4],
                );
                ui.separator();
                ui.slider(
                    "Inner 0",
                    1,
                    self.max_tessellation,
                    &mut self.tessellation_parameters.inner[0 * 4],
                );
                ui.slider(
                    "Inner 1",
                    1,
                    self.max_tessellation,
                    &mut self.tessellation_parameters.inner[1 * 4],
                );
                ui.separator();
                ui.separator();
                let mut tessellation_value = 0;
                tessellation_value += self.tessellation_parameters.outer[0 * 4];
                tessellation_value += self.tessellation_parameters.outer[1 * 4];
                tessellation_value += self.tessellation_parameters.outer[2 * 4];
                tessellation_value += self.tessellation_parameters.outer[3 * 4];
                tessellation_value += self.tessellation_parameters.inner[0 * 4];
                tessellation_value += self.tessellation_parameters.inner[1 * 4];
                tessellation_value /= 6;
                if ui.slider("All", 1, self.max_tessellation, &mut tessellation_value) {
                    for i in 0..4 {
                        self.tessellation_parameters.outer[i * 4] = tessellation_value;
                    }
                    for i in 0..2 {
                        self.tessellation_parameters.inner[i * 4] = tessellation_value;
                    }
                }
            });
    }

    fn quit(&self) -> bool {
        self.quit
    }
}
