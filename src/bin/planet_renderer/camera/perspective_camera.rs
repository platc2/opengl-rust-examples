use nalgebra_glm as glm;

use crate::camera::Camera;
use crate::movable::Movable;
use crate::transform::Transform;

pub struct PerspectiveCamera {
    transform: Transform,
    projection: glm::Mat4,
    view: glm::Mat4,

    aspect_ratio: f32,
    fov_y: f32,
    near: f32,
    far: f32,

    dirty: bool,
}

impl Camera for PerspectiveCamera {
    #[must_use]
    fn projection(&self) -> &glm::Mat4 { &self.projection }

    #[must_use]
    fn view(&self) -> &glm::Mat4 { &self.view }
}

impl Movable for PerspectiveCamera {
    fn transform_mut(&mut self) -> &mut Transform { &mut self.transform }
}

impl PerspectiveCamera {
    #[must_use]
    pub fn new(aspect_ratio: f32, fov_y: f32, near: f32, far: f32) -> Self {
        assert!(near < far, "near ({}) must be smaller than far ({}) ", near, far);

        let mut transform = Transform::default();
        transform.set_position(glm::vec3(0., 0., -5.));

        let world_pos = transform.position();
        let look_at = world_pos + transform.forward();
        let up = transform.up();
        let view = glm::look_at(world_pos, &look_at, up);

        let projection = glm::perspective(aspect_ratio, fov_y, near, far);

        Self {
            transform,
            view,
            projection,

            aspect_ratio,
            fov_y,
            near,
            far,

            dirty: false,
        }
    }

    #[must_use]
    pub fn transform(&self) -> &Transform { &self.transform }

    #[must_use]
    pub fn aspect_ratio(&self) -> f32 { self.aspect_ratio }

    pub fn set_aspect_ratio(&mut self, aspect_ratio: f32) {
        self.aspect_ratio = aspect_ratio;
        self.dirty = true;
    }

    #[must_use]
    pub fn fov(&self) -> f32 { self.fov_y }

    pub fn set_fov(&mut self, fov: f32) {
        self.fov_y = fov;
        self.dirty = true;
    }

    #[must_use]
    pub fn near(&self) -> f32 { self.near }

    pub fn set_near(&mut self, near: f32) {
        self.near = near;
        self.dirty = true;
    }

    #[must_use]
    pub fn far(&self) -> f32 { self.far }

    pub fn set_far(&mut self, far: f32) {
        self.far = far;
        self.dirty = true;
    }

    pub fn update(&mut self) {
        if self.transform.changed() {
            let world_pos = self.transform.position();
            let look_at = world_pos + self.transform.forward();
            let up = self.transform.up();
            self.view = glm::look_at(world_pos, &look_at, up);
        }

        if self.dirty {
            self.dirty = false;

            self.projection = glm::perspective(self.aspect_ratio, self.fov_y, self.near, self.far);
        }
    }
}
