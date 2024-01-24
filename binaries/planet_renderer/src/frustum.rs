use nalgebra_glm as glm;

use crate::camera::PerspectiveCamera;

pub struct Plane {
    pub position: glm::Vec3,
    pub normal: glm::Vec3,
}

pub struct Frustum {
    pub top_face: Plane,
    pub bottom_face: Plane,
    pub right_face: Plane,
    pub left_face: Plane,
    pub far_face: Plane,
    pub near_face: Plane,
}

impl Frustum {
    pub fn from_perspective_camera(camera: &PerspectiveCamera) -> Self {
        let camera_forward = glm::vec3(0., 0., -1.);
        let camera_up = glm::vec3(0., 1., 0.);
        let camera_right = glm::vec3(1., 0., 0.);

        let z_far: f32 = camera.far();
        let z_near: f32 = camera.near();
        let fov_y: f32 = camera.fov();
        let aspect_ratio: f32 = camera.aspect_ratio();
        let half_vertical_side: f32 = z_far * (fov_y * 0.5).tan();
        let half_horizontal_side: f32 = half_vertical_side * aspect_ratio;
        let front_far = z_far * camera_forward;

        let near_face = Plane {
            position: camera_forward * z_near,
            normal: camera_forward,
        };
        let far_face = Plane {
            position: camera_forward * z_far,
            normal: -camera_forward,
        };
        let right_face = Plane {
            position: glm::Vec3::zeros(),
            normal: glm::cross(&(front_far - camera_right * half_horizontal_side), &camera_up),
        };
        let left_face = Plane {
            position: glm::Vec3::zeros(),
            normal: glm::cross(&camera_up, &(front_far + camera_right * half_horizontal_side)),
        };
        let top_face = Plane {
            position: glm::Vec3::zeros(),
            normal: glm::cross(&camera_right, &(front_far - camera_up * half_vertical_side)),
        };
        let bottom_face = Plane {
            position: glm::Vec3::zeros(),
            normal: glm::cross(&(front_far + camera_up * half_vertical_side), &camera_right),
        };

        Self {
            top_face,
            bottom_face,
            right_face,
            left_face,
            far_face,
            near_face,
        }
    }
}
