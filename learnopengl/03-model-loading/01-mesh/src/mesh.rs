use gl;
use texture::{Texture, TextureType};
use vertex::Vertex;

pub struct Mesh {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    textures: Vec<Texture>,

    vertex_array_object: gl::VertexArrayId,
    vertex_buffer_object: gl::BufferId,
    element_buffer_object: gl::BufferId,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: &[u32], textures: &[Texture]) -> Self {
        let mut result = Self {
            vertices: Vec::from(vertices),
            indices: Vec::from(indices),
            textures: Vec::from(textures),

            vertex_array_object: gl::VertexArrayId::NO_VERTEX_ARRAY,
            vertex_buffer_object: gl::BufferId::NO_BUFFER,
            element_buffer_object: gl::BufferId::NO_BUFFER,
        };

        result.setupMesh();
        result
    }

    fn setupMesh(&mut self) {
        self.vertex_array_object = gl::create_vertex_array();
        self.vertex_buffer_object = gl::create_buffer();
        self.element_buffer_object = gl::create_buffer();

        gl::bind_vertex_array(self.vertex_array_object);

        gl::bind_buffer(gl::BufferTarget::ARRAY_BUFFER, self.vertex_buffer_object);
        gl::buffer_data(gl::BufferTarget::ARRAY_BUFFER, &self.vertices, gl::BufferUsage::STATIC_DRAW);

        gl::bind_buffer(gl::BufferTarget::ELEMENT_ARRAY_BUFFER, self.element_buffer_object);
        gl::buffer_data(gl::BufferTarget::ELEMENT_ARRAY_BUFFER, &self.indices, gl::BufferUsage::STATIC_DRAW);

        // Vertex positions
        gl::enable_vertex_attrib_array(0);
        gl::vertex_attrib_pointer(0, gl::ComponentSize::SIZE_3, gl::ComponentType::FLOAT, false, core::mem::size_of::<Vertex>(), 0);
        // Vertex normals
        gl::enable_vertex_attrib_array(1);
        gl::vertex_attrib_pointer(1, gl::ComponentSize::SIZE_3, gl::ComponentType::FLOAT, false, core::mem::size_of::<Vertex>(), core::mem::size_of::<glm::Vec3>());
        // Vertex texture coordinates
        gl::enable_vertex_attrib_array(2);
        gl::vertex_attrib_pointer(2, gl::ComponentSize::SIZE_2, gl::ComponentType::FLOAT, false, core::mem::size_of::<Vertex>(), core::mem::size_of::<glm::Vec3>() * 2);

        gl::bind_vertex_array(gl::VertexArrayId::NO_VERTEX_ARRAY);
    }

    pub fn draw(&self, shader_program: gl::ProgramId) {
        gl::use_program(shader_program);

        let mut diffuse_nr = 0;
        let mut specular_nr = 0;

        for (texture_id, texture) in self.textures.iter().enumerate() {
            gl::active_texture(gl::TextureUnit::fixed(texture_id));

            let index = match texture.texture_type {
                TextureType::Diffuse => {
                    diffuse_nr += 1;
                    diffuse_nr
                }
                TextureType::Specular => {
                    specular_nr += 1;
                    specular_nr
                }
                t => panic!("Unknown texture type {t:?}"),
            };

            let uniform_name = format!("material.{}{}", texture.texture_type.name(), index);
            gl::uniform_1i(gl::uniform_location(shader_program, uniform_name), texture_id as _);
            gl::bind_texture(gl::TextureTarget::TEXTURE_2D, texture.id);
        }
        gl::active_texture(gl::TextureUnit::fixed(0));

        // Draw mesh
        gl::bind_vertex_array(self.vertex_array_object);
        gl::draw_elements::<u32>(gl::DrawMode::TRIANGLES, self.indices.len(), gl::IndexType::UNSIGNED_INT, None);
        gl::bind_vertex_array(gl::VertexArrayId::NO_VERTEX_ARRAY);
    }
}
