use std::time::Instant;

use imgui::{Condition, TextureId, Ui};

use renderer::{Program, Texture};
use renderer::application::Application;
use renderer::input_manager::{InputManager, Key};
use renderer::time::Time;

use crate::camera::Camera;

pub struct State {
    camera: Camera,
    compute_program: Program,
    ssbo: gl::sys::types::GLuint,
    texture: Texture,
    t: i32,
    quit: bool,
}

impl State {
    pub fn new(compute_program: Program, ssbo: gl::sys::types::GLuint, texture: Texture) -> Self {
        Self {
            camera: Camera::default(),
            compute_program,
            ssbo,
            texture,
            t: 0,
            quit: false,
        }
    }
}

impl Application for State {
    fn tick(&mut self, time: &Time<Instant>, input_manager: &dyn InputManager) {
        if input_manager.key_down(Key::ESCAPE) {
            self.quit = true;
        }

        let speed = 5. * time.duration().as_secs_f32();
        if input_manager.key_down(Key::W) {
            self.camera.move_forward(speed);
        }
        if input_manager.key_down(Key::S) {
            self.camera.move_forward(-speed);
        }
        if input_manager.key_down(Key::D) {
            self.camera.move_right(speed);
        }
        if input_manager.key_down(Key::A) {
            self.camera.move_right(-speed);
        }
        if input_manager.key_down(Key::SPACE) {
            self.camera.move_up(speed);
        }
        if input_manager.key_down(Key::LEFT_CONTROL) {
            self.camera.move_up(-speed);
        }
        if input_manager.key_down(Key::UP_ARROW) {
            self.camera.look_up(speed);
        }
        if input_manager.key_down(Key::DOWN_ARROW) {
            self.camera.look_up(-speed);
        }
        if input_manager.key_down(Key::RIGHT_ARROW) {
            self.camera.look_right(speed);
        }
        if input_manager.key_down(Key::LEFT_ARROW) {
            self.camera.look_right(-speed);
        }

        unsafe {
            gl::sys::Clear(gl::sys::COLOR_BUFFER_BIT);
            gl::sys::Viewport(0, 0, 900, 700);

            self.compute_program.set_used();

            gl::sys::BindBufferBase(gl::sys::SHADER_STORAGE_BUFFER, 5, self.ssbo);

            self.t += 1;
            unsafe {
                gl::sys::Uniform3fv(0, 1, self.camera.position().as_ptr());
                gl::sys::Uniform3fv(1, 1, self.camera.forward().as_ptr());
                gl::sys::Uniform3fv(2, 1, self.camera.up().as_ptr());
            }

            unsafe {
                gl::sys::DispatchCompute(512, 512, 1);
                gl::sys::MemoryBarrier(gl::sys::SHADER_IMAGE_ACCESS_BARRIER_BIT);
            }
        }
    }

    fn gui(&mut self, ui: &Ui) {
        ui.window("Result")
            .save_settings(false)
            .resizable(false)
            .position([(900. - 512.) / 2., (700. - 512.) / 2.], Condition::Once)
            .no_decoration()
            .movable(false)
            .bring_to_front_on_focus(false)
            .always_use_window_padding(false)
            .focused(false)
            .build(|| {
                imgui::Image::new(TextureId::from(self.texture.handle() as usize), [512f32, 512f32])
                    .build(ui);
            });
        ui.window("Main")
            .save_settings(false)
            .always_auto_resize(true)
            .build(|| {
//                ui.text(format!("FPS: {}", fps));
//                ui.plot_lines("Frame time", &frame_times[..]).build();
            });
    }

    fn quit(&self) -> bool {
        self.quit
    }
}
