use renderer::time::{DurationSince, Now, Time};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MovementDirection {
    FORWARD,
    BACKWARD,
    LEFT,
    RIGHT,
}

#[derive(Debug)]
pub struct Camera {
    position: glm::Vec3,
    front: glm::Vec3,
    up: glm::Vec3,
    right: glm::Vec3,
    world_up: glm::Vec3,

    yaw: f32,
    pitch: f32,

    movement_speed: f32,
    mouse_sensitivity: f32,
    zoom: f32,
}

impl Camera {
    pub fn new(position: glm::Vec3, world_up: glm::Vec3, yaw: f32, pitch: f32) -> Self {
        let mut res = Self {
            position,
            front: glm::vec3(0., 0., -1.),
            up: glm::Vec3::default(),
            right: glm::Vec3::default(),
            world_up,

            yaw,
            pitch,

            movement_speed: 2.5,
            mouse_sensitivity: 0.1,
            zoom: 45.,
        };
        res.update_camera_vectors();
        res
    }

    #[must_use]
    pub fn position(&self) -> &glm::Vec3 { &self.position }

    pub fn view_matrix(&self) -> glm::Mat4 {
        glm::look_at(&self.position, &(self.position + self.front), &self.up)
    }

    pub fn process_keyboard<T: Now + DurationSince + Copy>(&mut self, movement_direction: MovementDirection, time: &Time<T>) {
        let velocity = time.duration().as_secs_f32() * self.movement_speed;
        match movement_direction {
            MovementDirection::FORWARD => self.position += self.front * velocity,
            MovementDirection::BACKWARD => self.position -= self.front * velocity,
            MovementDirection::LEFT => self.position -= self.right * velocity,
            MovementDirection::RIGHT => self.position += self.right * velocity,
        }
    }

    pub fn process_mouse_movement(&mut self, mut offset: (f32, f32), constraint_pitch: bool) {
        offset.0 *= self.mouse_sensitivity;
        offset.1 *= self.mouse_sensitivity;

        self.yaw += offset.0;
        self.pitch += offset.1;

        if constraint_pitch {
            self.pitch = self.pitch.clamp(-89., 89.);
        }

        self.update_camera_vectors();
    }

    pub fn process_mouse_scroll(&mut self, offset: f32) {
        self.zoom = (self.zoom - offset).clamp(1., 45.);
    }

    fn update_camera_vectors(&mut self) {
        let mut front = glm::Vec3::default();
        front.x = self.yaw.to_radians().cos() * self.pitch.to_radians().cos();
        front.y = self.pitch.to_radians().sin();
        front.z = self.yaw.to_radians().sin() * self.pitch.to_radians().cos();
        self.front = front.normalize();
        self.right = glm::cross(&self.front, &self.world_up).normalize();
        self.up = glm::cross(&self.right, &self.front).normalize();
    }
}
