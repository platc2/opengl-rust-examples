use std::fs::File;
use std::io::Read;
use std::iter::zip;
use std::path::Path;

use anyhow::Result;
use russimp::material::{Material, PropertyTypeInfo};
use russimp::node::Node;
use russimp::scene::{PostProcess, Scene};

use mesh::Mesh;
use texture::{Texture, TextureType};
use vertex::Vertex;

mod gl {
    pub use renderer::gl::program::*;
    pub use renderer::gl::texture::*;
}

pub struct Model {
    meshes: Vec<Mesh>,
    directory: String,
    textures_loaded: Vec<Texture>,
}

impl Model {
    pub fn new(path: &str) -> Self {
        let scene = Scene::from_file(path, vec![
            PostProcess::Triangulate,
            PostProcess::FlipUVs,
        ])
            .expect("Failed to load scene!");

        println!("{:?}", path);
        println!("{:?}", path.rfind("/"));
        let directory = if let Some(index) = path.rfind("/") {
            path[..index].to_owned()
        } else {
            path.to_owned()
        };

        let mut result = Self {
            directory,
            meshes: Vec::new(),
            textures_loaded: Vec::new(),
        };

        if let Some(root) = &scene.root {
            result.process_node(&scene, root);
        }

        result
    }

    fn process_node(&mut self, scene: &Scene, node: &Node) {
        for mesh in node.meshes.iter() {
            let mesh = &scene.meshes[*mesh as usize];
            self.process_mesh(scene, mesh);
        }

        for child in node.children.borrow().iter() {
            self.process_node(scene, child);
        }
    }

    fn process_mesh(&mut self, scene: &Scene, mesh: &russimp::mesh::Mesh) {
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();
        let mut textures: Vec<Texture> = Vec::new();

        for (idx, (vertex, normal)) in zip(&mesh.vertices, &mesh.normals).enumerate() {
            let position = glm::vec3(
                vertex.x, vertex.y, vertex.z);
            let normal = glm::vec3(
                normal.x, normal.y, normal.z);
            let tex_coord = mesh.texture_coords[0].clone()
                .map(|tex_coord| tex_coord[idx])
                .map(|tex_coord| glm::vec2(tex_coord.x, tex_coord.y));

            vertices.push(Vertex::new(
                position,
                normal,
                tex_coord,
            ));
        }

        for face in &mesh.faces {
            face.0.clone().into_iter().for_each(|idx| indices.push(idx));
        }

        let material = &scene.materials[mesh.material_index as usize];
        self.load_material_textures(material, &russimp::material::TextureType::Diffuse)
            .iter().for_each(|texture| textures.push(texture.clone()));
        self.load_material_textures(material, &russimp::material::TextureType::Specular)
            .iter().for_each(|texture| textures.push(texture.clone()));

        self.meshes.push(Mesh::new(vertices, &indices[..], &textures[..]));
    }

    pub fn draw(&self, shader_program: gl::ProgramId) {
        for mesh in &self.meshes {
            mesh.draw(shader_program);
        }
    }

    fn load_material_textures(&mut self, material: &Material, texture_type: &russimp::material::TextureType) -> Vec<Texture> {
        let mut result = Vec::new();

        for property in material.properties.iter()
            .filter(|prop| prop.semantic == *texture_type)
            .collect::<Vec<_>>() {
            let texture_type = match texture_type {
                russimp::material::TextureType::Diffuse => TextureType::Diffuse,
                russimp::material::TextureType::Specular => TextureType::Specular,
                _ => return Vec::new()
            };
            let file_name = match &property.data {
                PropertyTypeInfo::String(path) => path,
                _ => continue,
            };

            let file = format!("{}/{}", self.directory, file_name);

            let texture = if let Some(texture) = self.textures_loaded.iter().find(|t| t.path == file) {
                texture.clone()
            } else {
                let id = texture_from_file(file.as_str())
                    .unwrap();
                Texture { id, texture_type, path: file }
            };

            result.push(texture.clone());
            self.textures_loaded.push(texture.clone());
        }

        result
    }
}

fn texture_from_file(path: &str) -> Result<gl::TextureId> {
    let path = Path::new(path);
    let mut file = File::open(path)?;
    let mut texture_data = Vec::new();
    file.read_to_end(&mut texture_data)?;
    utils::load_texture_2d(texture_data.as_slice())
}
