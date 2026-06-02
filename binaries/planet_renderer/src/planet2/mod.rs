use crate::planet2::node::{Node, NodeType};
use crate::polyhedron::Polyhedron;
use crate::transform::Transform;
use nalgebra_glm as glm;

mod node;

#[derive(Debug)]
pub struct Planet {
    pub transform: Transform,
    pub root_nodes: Vec<Node>,
}

impl Planet {
    pub fn new(transform: Transform) -> Self {
        let mut root_nodes = Vec::new();

        let original_triangle = [
            glm::vec3(0., 3f32.sqrt() / 3., 0.0),
            glm::vec3(-0.5, -3f32.sqrt() / 6., 0.),
            glm::vec3(0.5, -3f32.sqrt() / 6., 0.),
        ];

        let icosahedron = Polyhedron::regular_icosahedron();
        for (a, b, c) in icosahedron.triangles.iter() {
            let centroid = calculate_centroid(a, b, c);
            let translation = centroid;
            let rotation = yet_another(&original_triangle, &[**a, **b, **c]);
            let scaling = glm::distance(a, b) / glm::distance(&original_triangle[0], &original_triangle[1]);
            let transform = Transform::new(translation, rotation, glm::vec3(scaling, scaling, scaling));
            root_nodes.push(Node::new(transform, 0));
        }

        Self {
            transform,
            root_nodes,
        }
    }

    pub fn update(&mut self, camera_position: &glm::Vec3, lod_ranges: &Vec<f32>) {
        for node in self.root_nodes.iter_mut() {
            node.update(camera_position, lod_ranges);
        }
    }

    pub fn all_nodes(&self) -> Vec<&Node> {
        collect_nodes(&self.root_nodes)
    }
}

fn collect_nodes(nodes: &Vec<Node>) -> Vec<&Node> {
    let (mut leaves, mut branches): (Vec<_>, Vec<_>) = nodes.into_iter()
        .partition(|node| node.node_type == NodeType::Leaf);

    let mut others = branches.into_iter()
        .flat_map(|node| collect_nodes(&node.children))
        .collect::<Vec<_>>();

    leaves.append(&mut others);
    leaves
}

fn calculate_centroid(a: &glm::Vec3, b: &glm::Vec3, c: &glm::Vec3) -> glm::Vec3 {
    (a + b + c) / 3.
}

fn calculate_triangle_normal(triangle: &[glm::Vec3; 3]) -> glm::Vec3 {
    glm::cross(
        &(triangle[1] - triangle[0]),
        &(triangle[2] - triangle[0]))
        .normalize()
}

fn quat_between_vectors(a: &glm::Vec3, b: &glm::Vec3) -> glm::Quat {
    let axis = glm::cross(a, b);
    let angle = glm::dot(a, b).acos();

    glm::quat_angle_axis(angle, &axis)
}

fn yet_another(original: &[glm::Vec3; 3], new: &[glm::Vec3; 3]) -> glm::Quat {
    let original_normal = calculate_triangle_normal(original);
    let new_normal = calculate_triangle_normal(new);
    let original_edge = original[2] - original[0];
    let new_edge = new[2] - new[0];

    let normal_quat = quat_rotation_between(&original_normal, &new_normal);

    let original_edge = glm::quat_rotate_vec3(&normal_quat, &original_edge);
    let quat = quat_rotation_between(&original_edge, &new_edge);

    quat * normal_quat
}

fn quat_rotation_between(v1: &glm::Vec3, v2: &glm::Vec3) -> glm::Quat {
    // Normalize both vectors
    let v1 = v1.normalize();
    let v2 = v2.normalize();

    // Calculate the dot product
    let dot = glm::dot(&v1, &v2);

    // If the vectors are almost identical, return the identity quaternion
    if dot > 0.9999 {
        return glm::Quat::identity();
    }

    // If the vectors are nearly opposite, we need a special case to avoid degenerate axis
    if dot < -0.9999 {
        // Find an arbitrary perpendicular axis (not aligned with v1)
        let perp_axis = if v1.x.abs() < 0.1 {
            glm::vec3(1.0, 0.0, 0.0)
        } else {
            glm::vec3(0.0, 1.0, 0.0)
        };
        let axis = glm::cross(&v1, &perp_axis).normalize();
        // Return a quaternion representing a 180-degree rotation around the perpendicular axis
        return glm::quat_angle_axis(std::f32::consts::PI, &axis);
    }

    // Compute the rotation axis (cross product of the two vectors)
    let axis = glm::cross(&v1, &v2).normalize();

    // Compute the rotation angle (from dot product)
    let angle = dot.acos();

    // Return the quaternion representing the rotation
    glm::quat_angle_axis(angle, &axis)
}
