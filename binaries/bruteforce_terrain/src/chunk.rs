use renderer::Texture;

pub type ChunkPosition = (i32, i32);

pub struct Chunk {
    position: ChunkPosition,
    texture: Texture
}

impl Chunk {

    pub fn init_chunk(position: ChunkPosition, texture: Texture) -> Self {
        Self { position, texture }
    }

    pub const fn position(&self) -> ChunkPosition { self.position }

    pub const fn texture_handle(&self) -> gl::types::GLuint { self.texture.handle() }
}
