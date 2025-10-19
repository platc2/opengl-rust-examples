use crate::camera::Camera;
use crate::movable::Movable;
use renderer::input_manager::{InputManager, Key};
use std::cell::RefCell;
use std::rc::Rc;

pub struct CameraController<C: Camera + Movable>
{
    camera: Rc<RefCell<C>>,
}

impl<C: Camera + Movable> CameraController<C> {
    pub fn new(camera: &Rc<RefCell<C>>) -> Self {
        Self { camera: camera.clone() }
    }

    pub fn handle_input<I: InputManager + ?Sized>(&self, input_manager: &I, delta: f32) {
        let speed = delta;

        let mut camera = self.camera.borrow_mut();
        if input_manager.key_down(Key::W) { camera.move_forward(speed); }
        if input_manager.key_down(Key::S) { camera.move_backward(speed); }
        if input_manager.key_down(Key::A) { camera.move_right(speed); }
        if input_manager.key_down(Key::D) { camera.move_left(speed); }
        if input_manager.key_down(Key::SPACE) { camera.move_up(speed); }
        if input_manager.key_down(Key::LEFT_CONTROL) { camera.move_down(speed); }
        if input_manager.key_down(Key::UP_ARROW) { camera.look_up(speed); }
        if input_manager.key_down(Key::DOWN_ARROW) { camera.look_down(speed); }
        if input_manager.key_down(Key::RIGHT_ARROW) { camera.look_right(speed); }
        if input_manager.key_down(Key::LEFT_ARROW) { camera.look_left(speed); }
        if input_manager.key_down(Key::Q) { camera.roll_ccw(speed); }
        if input_manager.key_down(Key::E) { camera.roll_cw(speed); }
    }
}
