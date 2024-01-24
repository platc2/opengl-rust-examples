use nalgebra_glm as glm;

#[derive(Debug, Copy, Clone)]
pub struct Transform {
    position: glm::Vec3,
    rotation: glm::Quat,
    scale: glm::Vec3,
    // Cache for regularly used values
    transform: glm::Mat4,
    forward: glm::Vec3,
    right: glm::Vec3,
    up: glm::Vec3,
    // Dirty flag to check if the transform has changed
    dirty: bool,
}

impl Default for Transform {
    fn default() -> Self {
        Self::new(
            glm::Vec3::default(),
            glm::Quat::identity(),
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

            transform: glm::Mat4::default(),
            forward: glm::Vec3::default(),
            right: glm::Vec3::default(),
            up: glm::Vec3::default(),

            dirty: false,
        }
    }

    #[must_use]
    pub fn new_euler(position: glm::Vec3,
                     euler_angles: glm::Vec3,
                     scale: glm::Vec3) -> Self {
        Self::new(position, Self::euler_to_quaternion(&euler_angles), scale)
    }

    #[must_use]
    pub const fn position(&self) -> &glm::Vec3 {
        &self.position
    }

    pub fn set_position(&mut self, position: glm::Vec3) {
        self.position = position;
        self.update();
        self.dirty = true;
    }

    #[must_use]
    pub const fn rotation(&self) -> &glm::Quat {
        &self.rotation
    }

    pub fn set_rotation(&mut self, rotation: glm::Quat) {
        self.rotation = rotation;
        self.update();
        self.dirty = true;
    }

    pub fn set_euler_angles(&mut self, rotation: glm::Vec3) {
        self.rotation = Self::euler_to_quaternion(&rotation);
        self.update();
        self.dirty = true;
    }

    #[must_use]
    pub const fn scale(&self) -> &glm::Vec3 {
        &self.scale
    }

    pub fn set_scale(&mut self, scale: glm::Vec3) {
        self.scale = scale;
        self.update();
        self.dirty = true;
    }

    #[must_use]
    pub fn transform(&self) -> &glm::Mat4 { &self.transform }

    #[must_use]
    pub fn forward(&self) -> &glm::Vec3 { &self.forward }

    #[must_use]
    pub fn right(&self) -> &glm::Vec3 { &self.right }

    #[must_use]
    pub fn up(&self) -> &glm::Vec3 { &self.up }

    #[must_use]
    pub fn changed(&mut self) -> bool {
        let dirty = self.dirty;
        self.dirty = false;
        dirty
    }

    fn update(&mut self) {
        self.transform = glm::translation(&self.position) *
            glm::quat_to_mat4(&self.rotation) *
            glm::scaling(&self.scale);
        self.forward = glm::quat_rotate_vec3(&self.rotation, &glm::vec3(0., 0., 1.));
        self.right = glm::quat_rotate_vec3(&self.rotation, &glm::vec3(1., 0., 0.));
        self.up = glm::cross(&self.forward, &self.right);
    }

    pub fn euler_to_quaternion(euler: &glm::Vec3) -> glm::Quat {
        let mut quat = glm::Quat::identity();

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
}
