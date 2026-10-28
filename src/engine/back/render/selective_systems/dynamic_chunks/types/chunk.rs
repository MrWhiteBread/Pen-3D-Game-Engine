use std::collections::{HashSet};
use glam::Vec3;
use crate::engine::back::render::selective_systems::dynamic_chunks::data::{Chunk, ChunkData, ChunkRequirements, Plane};
use crate::engine::back::render::selective_systems::dynamic_chunks::DynamicChunks;

impl DynamicChunks {
    pub fn set_manual_chunk_loading_state(&mut self, id: u32, state: bool) {
        let chunk = self.chunks.get_mut(&id).expect("no chunk found");

        if state && !chunk.loaded {
            self.lid.chunks.insert(id);
            Self::add_chunk_data(&mut self.lid, &mut self.data, chunk);
            chunk.loaded = true;

            return
        }

        if chunk.loaded {
            self.lid.chunks.swap_remove(&id);
            Self::remove_chunk_data(&mut self.lid, &mut self.data, chunk);

            chunk.loaded = false;
        }
    }

    pub fn new_manual_chunk(&mut self, points: Vec<[f32; 3]>, vertices: &Vec<[f32; 3]>, indices: &Vec<u32>, protected: bool, private: bool) -> u32 { // protected: the system can t insert data, only the user can, private: can t be loaded and unloaded by the system
        //WARNING: manual chunks won t work properly if you have other manual chunk`s points that are closer to a data_center inside your manual chunks than your manual chunks points
        let id = self.chunks_ids.get_id();
        let mut planes = Vec::with_capacity(vertices.len() / 3);
        let mut position = Vec3::from([0.0; 3]);
        let mut radius = 0.0;

        for indices in indices.chunks(3) {
            let v1 = Vec3::from(vertices[indices[0] as usize]);
            let v2 = Vec3::from(vertices[indices[1] as usize]);
            let v3 = Vec3::from(vertices[indices[2] as usize]);

            position += v1 + v2 + v3;

            let temp1 = v2 - v1;
            let temp2 = v3 - v1;
            let normal = temp1.cross(temp2).normalize();


            planes.push(Plane {
                normal,
                d: -normal.dot(v1),
            });
        }

        position[0] /= vertices.len() as f32;
        position[1] /= vertices.len() as f32;
        position[2] /= vertices.len() as f32;

        for vertex in vertices {
            let length = position.distance(Vec3::from(*vertex));

            if radius < length {
                radius = length;
            }
        }

        let chunk = Chunk {
            manual: true,
            protected,
            private,

            loaded: false,

            position: position.to_array(),
            chunk_req: Some(ChunkRequirements {
                points: points.clone(),

                radius,
                planes,
            }),

            chunk_data: ChunkData {
                lights: HashSet::new(),
                objects: HashSet::new(),
            }
        };

        self.chunks.insert(id, chunk);
        self.new_chunks.insert(id);

        for mut position in points {
            Self::apply_jitter3(&mut position, id);

            if !protected {
                self.non_protected_tree.add(&position, id);
            }

            if !private {
                self.non_private_tree.add(&position, id);
            }
        }

        id
    }
    
    pub(super) fn new_chunk(&mut self, mut position: [f32; 3]) -> u32 {
        let id = self.chunks_ids.get_id();
        let chunk = Chunk {
            manual: false,
            protected: false,
            private: false,

            loaded: false,

            position,
            chunk_req: None,

            chunk_data: ChunkData {
                lights: HashSet::new(),
                objects: HashSet::new(),
            }
        };

        self.chunks.insert(id, chunk);
        self.new_chunks.insert(id);

        Self::apply_jitter3(&mut position, id);
        self.auto_tree.add(&position, id);
        
        id
    }

    pub(super) fn remove_empty_chunk(&mut self, id: u32) {
        let chunk = self.chunks.get(&id).expect("chunk doesnt exist");

        if chunk.chunk_data.objects.is_empty() && chunk.chunk_data.lights.is_empty() {
            if chunk.manual {
                self.remove_manual_chunk(id);
                return
            }

            self.remove_chunk(id);
        }
    }

    fn remove_manual_chunk(&mut self, id: u32) {
        self.chunks_ids.remove(id);
        self.lid.chunks.swap_remove(&id);
        self.new_chunks.swap_remove(&id);

        let chunk = self.chunks.remove(&id).expect("chunk doesn't exist");

        for mut position in chunk.chunk_req.expect("idk").points {
            Self::apply_jitter3(&mut position, id);

            if !chunk.protected {
                self.non_protected_tree.remove(&position, id);
            }

            if !chunk.private {
                self.non_private_tree.remove(&position, id);
            }
        }
    }
    
    fn remove_chunk(&mut self, id: u32) {
        self.chunks_ids.remove(id);
        self.lid.chunks.swap_remove(&id);
        self.new_chunks.swap_remove(&id);

        let mut chunk = self.chunks.remove(&id).expect("chunk doesn't exist");
        
        Self::apply_jitter3(&mut chunk.position, id);
        self.auto_tree.remove(&chunk.position, id);
    }

    fn apply_jitter3(value: &mut [f32; 3], id: u32) {
        value[0] = Self::apply_jitter(value[0], id);
        value[1] = Self::apply_jitter(value[1], id);
        value[2] = Self::apply_jitter(value[2], id);
    }

    fn apply_jitter(value: f32, id: u32) -> f32 {
        let base_value = if value == 0.0 { 0.00001 } else { value };

        let bits = base_value.to_bits();
        let jittered_bits = bits + (id & 0x7F);
        f32::from_bits(jittered_bits)
    }
}