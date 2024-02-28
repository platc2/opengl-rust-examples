#[repr(packed)]
#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    position: glm::Vec3,
    normal: glm::Vec3,
    tex_coord: glm::Vec2,
}

impl Vertex {
    pub fn new(position: glm::Vec3, normal: glm::Vec3, tex_coord: Option<glm::Vec2>) -> Self {
        Self {
            position,
            normal,
            tex_coord: tex_coord.unwrap_or(glm::Vec2::default()),
        }
    }
}
