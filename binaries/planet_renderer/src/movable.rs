use nalgebra_glm as glm;

use crate::transform::Transform;

pub trait Movable {
    fn transform_mut(&mut self) -> &mut Transform;

    fn move_forward(&mut self, distance: f32) {
        let transform = self.transform_mut();
        transform.set_position(transform.position() + distance * transform.forward());
    }

    fn move_backward(&mut self, distance: f32) {
        let transform = self.transform_mut();
        transform.set_position(transform.position() - distance * transform.forward());
    }

    fn move_right(&mut self, distance: f32) {
        let transform = self.transform_mut();
        transform.set_position(transform.position() + distance * transform.right());
    }

    fn move_left(&mut self, distance: f32) {
        let transform = self.transform_mut();
        transform.set_position(transform.position() - distance * transform.right());
    }

    fn move_up(&mut self, distance: f32) {
        let transform = self.transform_mut();
        transform.set_position(transform.position() + distance * transform.up());
    }

    fn move_down(&mut self, distance: f32) {
        let transform = self.transform_mut();
        transform.set_position(transform.position() - distance * transform.up());
    }

    fn look_up(&mut self, angle: f32) {
        let transform = self.transform_mut();
        let new = transform.rotation() * Transform::euler_to_quaternion(&glm::vec3(-angle, 0., 0.));
        transform.set_rotation(new);
    }

    fn look_down(&mut self, angle: f32) {
        let transform = self.transform_mut();
        let new = transform.rotation() * Transform::euler_to_quaternion(&glm::vec3(angle, 0., 0.));
        transform.set_rotation(new);
    }

    fn look_right(&mut self, angle: f32) {
        let transform = self.transform_mut();
        let new = transform.rotation() * Transform::euler_to_quaternion(&glm::vec3(0., -angle, 0.));
        transform.set_rotation(new);
    }

    fn look_left(&mut self, angle: f32) {
        let transform = self.transform_mut();
        let new = transform.rotation() * Transform::euler_to_quaternion(&glm::vec3(0., angle, 0.));
        transform.set_rotation(new);
    }

    fn roll_ccw(&mut self, angle: f32) {
        let transform = self.transform_mut();
        let new = transform.rotation() * Transform::euler_to_quaternion(&glm::vec3(0., 0., -angle));
        transform.set_rotation(new);
    }

    fn roll_cw(&mut self, angle: f32) {
        let transform = self.transform_mut();
        let new = transform.rotation() * Transform::euler_to_quaternion(&glm::vec3(0., 0., angle));
        transform.set_rotation(new);
    }
}
