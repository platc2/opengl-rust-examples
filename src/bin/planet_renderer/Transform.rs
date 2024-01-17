use nalgebra_glm as glm;

pub struct Transform {
    position: glm::Vec3,
    euler_angles: glm::Vec3,
    scale: glm::Vec3,
}

impl Default for Transform {
    fn default() -> Self {
        Transform {
            position: glm::Vec3::default(),
            euler_angles: glm::Vec3::default(),
            scale: glm::vec3(1., 1., 1.),
        }
    }
}

impl Transform {
    pub fn new(position: glm::Vec3,
               euler_angles: glm::Vec3,
               scale: glm::Vec3) -> Self {
        Self { position, euler_angles, scale }
    }

    pub const fn position(&self) -> &glm::Vec3 {
        &self.position
    }

    pub const fn rotation(&self) -> &glm::Vec3 {
        &self.euler_angles
    }

    pub const fn scale(&self) -> &glm::Vec3 {
        &self.scale
    }
}
