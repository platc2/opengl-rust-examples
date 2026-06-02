use alloc::rc::Rc;
use imgui::Ui;
use renderer::application::View;
use std::cell::RefCell;

const GAMMA_MIN: f32 = 0.5_f32;
const GAMMA_MAX: f32 = 2.5_f32;

pub struct GammaWindow {
    gamma: Rc<RefCell<f32>>,
}

impl GammaWindow {
    pub const fn new(gamma: Rc<RefCell<f32>>) -> Self {
        Self { gamma }
    }
}

impl View for GammaWindow {
    fn name(&self) -> &'static str {
        "Gamma Settings"
    }

    fn show(&mut self, ui: &Ui) {
        ui.slider("Gamma", GAMMA_MIN, GAMMA_MAX, &mut *self.gamma.borrow_mut());
        if ui.button("Reset (1.0)") {
            *self.gamma.borrow_mut() = 1_f32;
        }
        ui.same_line();
        if ui.button("Reset (2.2)") {
            *self.gamma.borrow_mut() = 2.2_f32;
        }
    }
}
