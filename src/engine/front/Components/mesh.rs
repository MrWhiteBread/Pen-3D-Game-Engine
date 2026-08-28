use crate::engine::back::types::vertex::Vertex;

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,

    pub radius: f32,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>, radius: f32) -> Self {
        Self {
            vertices,
            indices,
            radius,
        }
    }

    pub fn unload(self) -> (Vec<Vertex>, Vec<u32>) {
        (self.vertices, self.indices)
    }
}