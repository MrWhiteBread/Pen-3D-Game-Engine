use std::collections::HashSet;
use kiddo::SquaredEuclidean;
use crate::engine::back::render::selective_systems::dynamic_chunks::data::Chunk;
use crate::engine::back::render::selective_systems::dynamic_chunks::DynamicChunks;

impl DynamicChunks {
    pub fn new_chunk(&mut self, position: [f32; 3], private: bool) -> u32 {
        let id = self.chunks_ids.get_id();
        let chunk = Chunk {
            private,
            loaded: false,
            position,

            lights: HashSet::new(),
            objects: HashSet::new(),
            vertices: HashSet::new(),
            indices: HashSet::new(),
        };

        self.chunks.insert(id, chunk);
        let nearest = self.tree.nearest_one::<SquaredEuclidean>(&position);
        let global_id;

        if nearest.distance == 0.0 {
            self.ids.get_mut(&nearest.item).expect("id doesn t exist").insert(id);
            global_id = nearest.item;
        } else {
            let chunks_id = self.create_tree_chunk(position);
            self.ids.get_mut(&chunks_id).expect("id doesn exist").insert(id);
            global_id = chunks_id;
        }
        self.new_chunks.entry(global_id).or_default().insert(id);

        id
    }

    fn apply_jitter(value: &f32, id: &u32) -> f32 {
        let base_value = if *value == 0.0 { 0.00001 } else { *value };

        let bits = base_value.to_bits();
        let jittered_bits = bits + (id & 0x7F);
        f32::from_bits(jittered_bits)
    }

    fn create_tree_chunk(&mut self, mut position: [f32; 3]) -> u32 {
        let chunks = HashSet::new();

        let chunks_id = self.tree_ids.get_id();
        position[0] = Self::apply_jitter(&position[0], &chunks_id);
        position[1] = Self::apply_jitter(&position[1], &chunks_id);
        position[2] = Self::apply_jitter(&position[2], &chunks_id);

        self.ids.insert(chunks_id, chunks);

        self.tree.add(&position, chunks_id);
        chunks_id
    }

}