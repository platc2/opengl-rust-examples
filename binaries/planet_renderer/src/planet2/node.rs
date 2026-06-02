use crate::transform::Transform;
use nalgebra_glm as glm;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum NodeType {
    Leaf,
    Branch,
}

#[derive(Debug)]
pub struct Node {
    pub transform: Transform,
    pub node_type: NodeType,
    lod: u32,
    pub children: Vec<Node>,
}

impl Node {
    pub fn new(transform: Transform, lod: u32) -> Self {
        Self {
            transform,
            node_type: NodeType::Leaf,
            lod,
            children: Vec::new(),
        }
    }

    pub fn update(&mut self, camera_position: &glm::Vec3, lod_ranges: &Vec<f32>) {
        if lod_ranges.len() <= self.lod as usize {
            return;
        }

        for child in self.children.iter_mut() {
            child.update(camera_position, lod_ranges);
        }

        // Update child nodes
        let distance = glm::length2(&(camera_position - self.transform.position()));
        let in_range = distance < lod_ranges[self.lod as usize];
        if in_range && self.node_type == NodeType::Leaf {
            self.node_type = NodeType::Branch;

            let up = self.transform.up();
            let right = self.transform.right();
            let scale: f32 = self.transform.scale().x;
            let scale = 1. / 2_i32.pow(self.lod) as f32;
            let offsets = [
                up * ((1. / 6.) * scale),
                -up * ((1. / 6.) * scale) + right * ((1. / 6.) * scale),
                -up * ((1. / 6.) * scale) - right * ((1. / 6.) * scale)
            ];

            offsets.into_iter()
                .for_each(|offset| {
                    self.children.push(Node::new(
                        Transform::new(
                            self.transform.position() + offset,
                            *self.transform.rotation(),
                            self.transform.scale() * 0.5
                        ), self.lod + 1
                    ));
                });
        } else if !in_range && self.node_type == NodeType::Branch {
            self.node_type = NodeType::Leaf;

            self.children.clear();
        }
    }
}
