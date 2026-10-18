use half::f16;
use crate::engine::back::types::vertex::Vertex;
use crate::engine::front::components::mesh::Mesh;
use crate::engine::front::components::shapes::import_mesh::normals_to_u32;

pub fn cuboid(size: &[f32; 3]) -> Mesh {
    let mut vertices = Vec::with_capacity(24);
    let mut indices = Vec::with_capacity(36);

    let w = size[0];
    let h = size[1];
    let d = size[2];
    let radius = (w.powi(2) + h.powi(2) + d.powi(2)).sqrt();

    let uvs = [
        // Front (+Z)
        [0.0, 0.0],
        [1.0, 0.0],
        [1.0, 1.0],
        [0.0, 1.0],

        // Back (-Z)
        [1.0, 0.0],
        [0.0, 0.0],
        [0.0, 1.0],
        [1.0, 1.0],

        // Left (-X)
        [0.0, 0.0],
        [1.0, 0.0],
        [1.0, 1.0],
        [0.0, 1.0],

        // Right (+X)
        [0.0, 0.0],
        [1.0, 0.0],
        [1.0, 1.0],
        [0.0, 1.0],

        // Top (+Y)
        [0.0, 0.0],
        [1.0, 0.0],
        [1.0, 1.0],
        [0.0, 1.0],

        // Bottom (-Y)
        [0.0, 0.0],
        [1.0, 0.0],
        [1.0, 1.0],
        [0.0, 1.0],
    ];

    let normals: [[f32; 3]; 6] = [
        [0.0, 0.0, 1.0],
        [0.0, 0.0, -1.0],
        [-1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, -1.0, 0.0],
    ];

    let vertices_list = [
        // Front (+Z)
        [-w / 2.0, -h / 2.0, d / 2.0],
        [w / 2.0, -h / 2.0, d / 2.0],
        [w / 2.0, h / 2.0, d / 2.0],
        [-w / 2.0, h / 2.0, d / 2.0],

        // Back (-Z)
        [w / 2.0, -h / 2.0, -d / 2.0],
        [-w / 2.0, -h / 2.0, -d / 2.0],
        [-w / 2.0, h / 2.0, -d / 2.0],
        [w / 2.0, h / 2.0, -d / 2.0],

        // Left (-X)
        [-w / 2.0, -h / 2.0, -d / 2.0],
        [-w / 2.0, -h / 2.0, d / 2.0],
        [-w / 2.0, h / 2.0, d / 2.0],
        [-w / 2.0, h / 2.0, -d / 2.0],

        // Right (+X)
        [w / 2.0, -h / 2.0, d / 2.0],
        [w / 2.0, -h / 2.0, -d / 2.0],
        [w / 2.0, h / 2.0, -d / 2.0],
        [w / 2.0, h / 2.0, d / 2.0],

        // Top (+Y)
        [-w / 2.0, h / 2.0, d / 2.0],
        [w / 2.0, h / 2.0, d / 2.0],
        [w / 2.0, h / 2.0, -d / 2.0],
        [-w / 2.0, h / 2.0, -d / 2.0],

        // Bottom (-Y)
        [-w / 2.0, -h / 2.0, -d / 2.0],
        [w / 2.0, -h / 2.0, -d / 2.0],
        [w / 2.0, -h / 2.0, d / 2.0],
        [-w / 2.0, -h / 2.0, d / 2.0],
    ];

    for i in 0..vertices_list.len() {
        vertices.push(Vertex {
            position: vertices_list[i],
            normal: normals_to_u32(normals[i / 4]),
            uv: [f16::from_f32(uvs[i][0]), f16::from_f32(uvs[i][1])],
        });
    }

    for i in 0..6 {
        let base = (i * 4) as u32;
        indices.extend([base, base + 1, base + 2,
            base, base + 2, base + 3,]);
    }

    Mesh::new(vertices, indices, radius)
}