use std::collections::HashSet;
use kiddo::SquaredEuclidean;
use crate::engine::back::render::selective_systems::dynamic_chunks::data::Chunk;
use crate::engine::back::render::selective_systems::dynamic_chunks::DynamicChunks;

impl DynamicChunks {
    pub fn new_chunk(&mut self, mut position: [f32; 3], private: bool) -> u32 {
        let id = self.chunks_ids.get_id();
        let chunk = Chunk {
            private,
            loaded: false,
            position,

            lights: HashSet::new(),
            objects: HashSet::new(),
            
            textures: HashSet::new(),
        };

        self.chunks.insert(id, chunk);
        self.new_chunks.insert(id);

        Self::apply_jitter3(&mut position, id);
        self.tree.add(&position, id);
        
        id
    }

    pub(crate) fn remove_empty_chunk(&mut self, id: u32) {
        let chunk = self.chunks.get(&id).expect("chunk doesnt exist");

        if chunk.objects.is_empty() && chunk.textures.is_empty() && chunk.lights.is_empty() {
            self.remove_chunk(id);
        }
    }

    fn remove_chunk(&mut self, id: u32) {
        self.chunks_ids.remove(id);
        self.lid.chunks.swap_remove(&id);
        self.new_chunks.swap_remove(&id);

        let mut chunk = self.chunks.remove(&id).expect("chunk doesn't exist");

        self.tree.nearest_one::<SquaredEuclidean>(&chunk.position);

        Self::apply_jitter3(&mut chunk.position, id);
        self.tree.remove(&chunk.position, id);
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