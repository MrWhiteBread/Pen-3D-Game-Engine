use std::collections::HashMap;
use std::ops::Range;
use rand::{rng, RngExt};
use crate::engine::front::components::mesh::Mesh;
use crate::engine::front::components::shapes::normals::set_normals;

pub fn randomise(mesh: &mut Mesh, range: Range<f32>) {
    let mut r: [f32; 3] = [0.0; 3];
    let mut rand_v: HashMap<(i32, i32, i32), [f32; 3]> = HashMap::new();

    for vertex in mesh.vertices.iter_mut() {
        let a = (
            (vertex.position[0] * 1000.0) as i32,
            (vertex.position[1] * 1000.0) as i32,
            (vertex.position[2] * 1000.0) as i32,
        );

        if let Some(rand) = rand_v.get(&a) {
            vertex.position[0] += rand[0];
            vertex.position[1] += rand[1];
            vertex.position[2] += rand[2];

            vertex.old_position[0] += rand[0];
            vertex.old_position[1] += rand[1];
            vertex.old_position[2] += rand[2];
        } else {
            r[0] = rng().random_range(range.clone());
            r[1] = rng().random_range(range.clone());
            r[2] = rng().random_range(range.clone());

            vertex.position[0] += r[0];
            vertex.position[1] += r[1];
            vertex.position[2] += r[2];

            vertex.old_position[0] += r[0];
            vertex.old_position[1] += r[1];
            vertex.old_position[2] += r[2];
            rand_v.insert(a, r);
        }

        let radius = (vertex.position[0].powi(2) + vertex.position[1].powi(2) + vertex.position[2].powi(2)).sqrt();
        if radius > mesh.radius {
            mesh.radius = radius;
        }
    }

    set_normals(mesh);
}