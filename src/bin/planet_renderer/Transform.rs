use nalgebra_glm as glm;

pub struct Transform {
    position: glm::Vec3,
    rotation: glm::Quat,
    scale: glm::Vec3,

    transform: glm::Mat4,
    forward: glm::Vec3,
    right: glm::Vec3,
    up: glm::Vec3,
}

impl Default for Transform {
    fn default() -> Self {
        Self::new(
            glm::Vec3::default(),
            glm::Quat::default(),
            glm::vec3(1., 1., 1.))
    }
}

impl Transform {
    #[must_use]
    pub fn new(position: glm::Vec3,
               rotation: glm::Quat,
               scale: glm::Vec3) -> Self {
        let transform = glm::translation(&position) *
            glm::quat_to_mat4(&rotation) *
            glm::scaling(&scale);
        let forward = glm::quat_rotate_vec3(&rotation, &glm::vec3(0., 0., 1.));
        let right = glm::quat_rotate_vec3(&rotation, &glm::vec3(1., 0., 0.));
        let up = glm::cross(&forward, &right);

        Self {
            position,
            rotation,
            scale,

            transform,
            forward,
            right,
            up,
        }
    }

    #[must_use]
    pub fn new_euler(position: glm::Vec3,
                     euler_angles: glm::Vec3,
                     scale: glm::Vec3) -> Self {
        Self::new(position, euler_to_quaternion(&euler_angles), scale)
    }

    #[must_use]
    pub const fn position(&self) -> &glm::Vec3 {
        &self.position
    }

    pub fn set_position(&mut self, position: glm::Vec3) { self.position = position; }

    #[must_use]
    pub const fn rotation(&self) -> &glm::Quat {
        &self.rotation
    }

    pub fn set_rotation(&mut self, rotation: glm::Quat) { self.rotation = rotation; }

    pub fn set_euler_angles(&mut self, rotation: glm::Vec3) { self.rotation = euler_to_quaternion(&rotation); }

    #[must_use]
    pub const fn scale(&self) -> &glm::Vec3 {
        &self.scale
    }

    pub fn set_scale(&mut self, scale: glm::Vec3) { self.scale = scale; }
}

fn euler_to_quaternion(euler: &glm::Vec3) -> glm::Quat {
    let mut quat = glm::Quat::default();

    let euler = euler * 0.5;
    let cr = euler.x.cos();
    let sr = euler.x.sin();
    let cp = euler.y.cos();
    let sp = euler.y.sin();
    let cy = euler.z.cos();
    let sy = euler.z.sin();

    quat.w = cr * cp * cy + sr * sp * sy;
    quat.i = sr * cp * cy - cr * sp * sy;
    quat.j = cr * sp * cy + sr * cp * sy;
    quat.k = cr * cp * sy - sr * sp * cy;

    quat
}
