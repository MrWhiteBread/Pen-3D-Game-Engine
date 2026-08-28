use glam::Vec3;
use crate::engine::front::components::mesh::Mesh;

pub fn set_normals(mesh: &mut Mesh) {
    for indices in mesh.indices.chunks_exact_mut(3) {
        let v0 = mesh.vertices[indices[0] as usize];
        let v1 = mesh.vertices[indices[1] as usize];
        let v2 = mesh.vertices[indices[2] as usize];

        let f0 = Vec3::from(v0.position);
        let f1 = Vec3::from(v1.position);
        let f2 = Vec3::from(v2.position);

        let edge1 = f1 - f0;
        let edge2 = f2 - f1;
        let face_normal = edge1.cross(edge2);

        mesh.vertices[indices[0] as usize].normal[0] = face_normal.x;
        mesh.vertices[indices[0] as usize].normal[1] = face_normal.y;
        mesh.vertices[indices[0] as usize].normal[2] = face_normal.z;

        mesh.vertices[indices[1] as usize].normal[0] = face_normal.x;
        mesh.vertices[indices[1] as usize].normal[1] = face_normal.y;
        mesh.vertices[indices[1] as usize].normal[2] = face_normal.z;

        mesh.vertices[indices[2] as usize].normal[0] = face_normal.x;
        mesh.vertices[indices[2] as usize].normal[1] = face_normal.y;
        mesh.vertices[indices[2] as usize].normal[2] = face_normal.z;
    }

    for vertex in mesh.vertices.iter_mut() {
        let normal = Vec3::from_array(vertex.normal).normalize_or_zero();
        vertex.normal[0] = normal.x;
        vertex.old_normal[0] = normal.x;
        vertex.normal[1] = normal.y;
        vertex.old_normal[1] = normal.y;
        vertex.normal[2] = normal.z;
        vertex.old_normal[2] = normal.z;
    }
}