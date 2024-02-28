use std::iter::zip;
use std::ops::Index;
use std::path::Path;
use assimp::{Node, Scene};
use gl;
use mesh::Mesh;
use texture::Texture;
use vertex::Vertex;

pub struct Model {
    meshes: Vec<Mesh>,
    directory: String,
}

impl Model {
    pub fn new(path: &str) -> Self {
        let importer = {
            let mut res = assimp::Importer::new();

            res.triangulate(true);
            res.flip_uvs(true);

            res
        };

        let scene = importer.read_file(path)
            .expect("Failed to load scene!");
        if scene.is_incomplete() {
            panic!("Scene is incomplete!");
        }

        let directory = if let Some(index) = path.rfind("/") {
            path[index..].to_owned()
        } else {
            path.to_owned()
        };

        let mut result = Self {
            directory,
            meshes: Vec::new()
        };

        result.process_node(&scene, scene.root_node());

        result
    }

    fn process_node(&mut self, scene: &assimp::Scene, node: assimp::Node) {
        for mesh in node.meshes().iter()
            .flat_map(|mesh_id| scene.mesh(*mesh_id as usize)) {
            self.process_mesh(scene, mesh);
        }

        for child in node.child_iter() {
            self.process_node(scene, child);
        }
    }

    fn process_mesh(&mut self, scene: &Scene, mesh: assimp::Mesh) {
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();
        let mut textures: Vec<Texture> = Vec::new();

        for (idx, (vertex, normal)) in zip(mesh.vertex_iter(), mesh.normal_iter()).enumerate() {
            let position = glm::vec3(
                vertex.x, vertex.y, vertex.z);
            let normal = glm::vec3(
                normal.x, normal.y, normal.z);
            let tex_coord = mesh.get_texture_coord(0, idx as _)
                .map(|tex_coord| glm::vec2(tex_coord.x, tex_coord.y));

            vertices.push(Vertex::new(
                position,
                normal,
                tex_coord
            ));
        }

        for face in mesh.face_iter() {
            let a = (0..face.num_indices)
                .map(|idx| *face.index(idx as _) as u32)
                .for_each(|idx| indices.push(idx));
        }

        for vertex in mesh.vertex_iter() {
        }
    }

    pub fn draw(&self, shader_program: gl::ProgramId) {
        for mesh in &self.meshes {
            mesh.draw(shader_program);
        }
    }
}
