use crate::engine::back::types::vertex::Vertex;
use crate::engine::front::components::mesh::Mesh;

pub fn cuboid(size: &[f32; 3], color: &[f32; 4], material: &[f32; 2]) -> Mesh {
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

    let normals = [
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
            old_position: vertices_list[i],
            color: *color,
            old_color: *color,
            normal: normals[i / 4],
            old_normal: normals[i / 4],
            material: *material,
            old_material: *material,
            uv: uvs[i],
            old_uv: uvs[i],
            skeleton: 0,
        });
    }

    for i in 0..6 {
        let base = (i * 4) as u32;
        indices.extend([base, base + 1, base + 2,
            base, base + 2, base + 3,]);
    }

    Mesh::new(vertices, indices, radius)
}

pub fn lag_cuboid(size: &[f32; 3], color: &[f32; 4], cubes: &u32, material: &[f32; 2]) -> Mesh {
    let mut vertices = Vec::with_capacity(24 * *cubes as usize);
    let mut indices = Vec::with_capacity(36 * *cubes as usize);

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

    let normals = [
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

    for _ in 0..*cubes {
        for i in 0..vertices_list.len() {
            vertices.push(Vertex {
                position: vertices_list[i],
                old_position: vertices_list[i],
                color: *color,
                old_color: *color,
                normal: normals[i / 4],
                old_normal: normals[i / 4],
                material: *material,
                old_material: *material,
                uv: uvs[i],
                old_uv: uvs[i],
                skeleton: 0,
            });
        }

        for i in 0..6 {
            let base = (i * 4) as u32;
            indices.extend([base, base + 1, base + 2,
                base, base + 2, base + 3,]);
        }
    }

    Mesh::new(vertices, indices, radius)
}