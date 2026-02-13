use alloc::rc::Rc;
use std::cell::RefCell;
use std::time::Instant;

use imgui::Ui;

use crate::gamma_window;
use gl::sys::types::{GLintptr, GLsizei};
use renderer::application::{Application, View};
use renderer::input_manager::{InputManager, Key};
use renderer::time::Time;
use renderer::{Buffer, RenderPass};

pub struct State {
    render_pass: RenderPass,
    gamma_buffer: Buffer,
    vertex_buffer: Buffer,
    gamma: Rc<RefCell<f32>>,
    views: Vec<Box<dyn View>>,

    quit: bool,
}

impl State {
    pub fn new(render_pass: RenderPass, gamma_buffer: Buffer, vertex_buffer: Buffer) -> Self {
        let gamma = Rc::new(RefCell::new(1.0f32));
        Self {
            render_pass,
            gamma_buffer,
            vertex_buffer,
            gamma: gamma.clone(),
            views: vec![Box::new(gamma_window::GammaWindow::new(gamma.clone()))],

            quit: false,
        }
    }
}

impl Application for State {
    fn tick(&mut self, _: &Time<Instant>, _: &dyn InputManager) {
        self.render_pass.display();

        unsafe {
            let gamma_ptr = self.gamma_buffer.map::<f32>();
            gamma_ptr.copy_from_slice(&[*self.gamma.borrow()]);
            self.gamma_buffer.unmap();

            gl::sys::Clear(gl::sys::COLOR_BUFFER_BIT);
            gl::sys::Viewport(0, 0, 900, 700);
            gl::sys::BindVertexBuffer(
                0,
                self.vertex_buffer.handle(),
                0 as GLintptr,
                GLsizei::try_from(size_of::<f32>() * 5).unwrap(),
            );
            gl::sys::BindVertexBuffer(
                1,
                self.vertex_buffer.handle(),
                GLintptr::try_from(size_of::<f32>() * 2).unwrap(),
                GLsizei::try_from(size_of::<f32>() * 5).unwrap(),
            );
            gl::sys::DrawArrays(gl::sys::TRIANGLES, 0, 3);
        }
    }

    fn views(&mut self) -> &mut [Box<dyn View>] {
        &mut self.views
    }

    fn quit(&self) -> bool {
        self.quit
    }
}
