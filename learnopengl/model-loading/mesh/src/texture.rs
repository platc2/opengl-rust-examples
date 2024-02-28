use gl;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TextureType {
    Diffuse,
    Specular,
    Unknown,
}

impl TextureType {
    pub fn name(self) -> String {
        match self {
            TextureType::Diffuse => "texture_diffuse".to_owned(),
            TextureType::Specular => "texture_specular".to_owned(),
            TextureType::Unknown => panic!("Unknown texture type"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Texture {
    pub id: gl::TextureId,
    pub texture_type: TextureType,
}
