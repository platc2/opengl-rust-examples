use nalgebra_glm as glm;

use crate::transform::Transform;
pub use perspective_camera::PerspectiveCamera;
pub use camera_controller::CameraController;

mod orthographic_camera;
mod perspective_camera;
mod camera_controller;

pub trait Camera {
    #[must_use]
    fn projection(&self) -> &glm::Mat4;

    #[must_use]
    fn view(&self) -> &glm::Mat4;
}

fn update_camera_view_matrix(view: &mut glm::Mat4, transform: &Transform) {
    let world_pos = transform.position();
    let look_at = world_pos + transform.forward();
    let up = transform.up();
    *view = glm::look_at(world_pos, &look_at, up);
}
