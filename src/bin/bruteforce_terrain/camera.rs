use std::cmp::{max, min};

#[derive(Default)]
pub struct Camera {
    position: nalgebra_glm::Vec3,
    yaw: f32,
    pitch: f32,
}

impl Camera {
    pub const fn position(&self) -> nalgebra_glm::Vec3 {
        self.position
    }

    pub fn move_forward(&mut self, units: f32) {
        self.position += self.forward() * units;
    }

    pub fn move_right(&mut self, units: f32) {
        self.position += self.right() * units;
    }

    pub fn move_up(&mut self, units: f32) {
        self.position += self.up() * units;
    }

    pub fn look_up(&mut self, angle: f32) {
        self.pitch += angle;
        if self.pitch > 180f32 {
            self.pitch = 180f32;
        }
        if self.pitch < -180f32 {
            self.pitch = -180f32;
        }
    }

    pub fn look_right(&mut self, angle: f32) {
        self.yaw += angle;
    }

    pub fn view_matrix(&self) -> nalgebra_glm::Mat4 {
        nalgebra_glm::look_at(
            &self.position,
            &(self.position + self.forward()),
            &self.up(),
        )
    }

    fn forward(&self) -> nalgebra_glm::Vec3 {
        self.rotated(&nalgebra_glm::vec3(0f32, 0f32, -1f32))
    }

    fn backward(&self) -> nalgebra_glm::Vec3 {
        -self.forward()
    }

    fn right(&self) -> nalgebra_glm::Vec3 {
        self.rotated(&nalgebra_glm::vec3(1f32, 0f32, 0f32))
    }

    fn left(&self) -> nalgebra_glm::Vec3 {
        -self.right()
    }

    fn up(&self) -> nalgebra_glm::Vec3 {
        self.rotated(&nalgebra_glm::vec3(0f32, 1f32, 0f32))
    }

    fn down(&self) -> nalgebra_glm::Vec3 {
        -self.up()
    }

    fn rotated(&self, vector: &nalgebra_glm::Vec3) -> nalgebra_glm::Vec3 {
        nalgebra_glm::rotate_vec3(
            &nalgebra_glm::rotate_vec3(&vector, self.pitch, &nalgebra_glm::vec3(1f32, 0f32, 0f32)),
            -self.yaw,
            &nalgebra_glm::vec3(0f32, 1f32, 0f32),
        )
    }
}
