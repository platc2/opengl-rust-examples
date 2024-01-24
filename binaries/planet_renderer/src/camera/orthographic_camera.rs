use nalgebra_glm as glm;

use crate::camera::Camera;
use crate::movable::Movable;
use crate::transform::Transform;

pub struct OrthographicCamera {
    transform: Transform,
    projection: glm::Mat4,
    view: glm::Mat4,
}

impl Camera for OrthographicCamera {
    #[must_use]
    fn projection(&self) -> &glm::Mat4 { &self.projection }

    #[must_use]
    fn view(&self) -> &glm::Mat4 { &self.view }
}

impl Movable for OrthographicCamera {
    fn transform_mut(&mut self) -> &mut Transform { &mut self.transform }
}

impl OrthographicCamera {
    #[must_use]
    pub fn new(near: f32, far: f32) -> Self {
        assert!(near < far, "near ({}) must be smaller than far ({}) ", near, far);

        let mut transform = Transform::default();
        transform.set_position(glm::vec3(0., 0., -5.));

        let world_pos = transform.position();
        let look_at = world_pos + transform.forward();
        let up = transform.up();
        let view = glm::look_at(world_pos, &look_at, up);

        let projection = glm::ortho(-1., 1., -1., 1., 0.1, 100.);

        Self {
            transform,
            view,
            projection,
        }
    }

    pub fn update(&mut self) {
        if self.transform.changed() {
            let world_pos = self.transform.position();
            let look_at = world_pos + self.transform.forward();
            let up = self.transform.up();
            self.view = glm::look_at(world_pos, &look_at, up);
        }
    }
}
