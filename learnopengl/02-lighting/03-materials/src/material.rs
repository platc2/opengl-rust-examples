use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
pub struct JsonMaterial {
    name: String,

    ambient: [f32; 3],
    diffuse: [f32; 3],
    specular: [f32; 3],
    shininess: f32,
}

#[derive(Debug)]
pub struct Material {
    name: String,

    ambient: glm::Vec3,
    diffuse: glm::Vec3,
    specular: glm::Vec3,
    shininess: f32,
}

impl From<JsonMaterial> for Material {
    fn from(value: JsonMaterial) -> Self {
        Self {
            name: value.name,
            ambient: glm::Vec3::from(value.ambient),
            diffuse: glm::Vec3::from(value.ambient),
            specular: value.specular.into(),
            shininess: value.shininess,
        }
    }
}

impl Material {
    #[must_use]
    pub fn name(&self) -> &String { &self.name }

    #[must_use]
    pub fn ambient(&self) -> &glm::Vec3 { &self.ambient }

    #[must_use]
    pub fn diffuse(&self) -> &glm::Vec3 { &self.diffuse }

    #[must_use]
    pub fn specular(&self) -> &glm::Vec3 { &self.specular }

    #[must_use]
    pub fn shininess(&self) -> f32 { self.shininess }
}
