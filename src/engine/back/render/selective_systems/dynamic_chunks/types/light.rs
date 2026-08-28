use kiddo::SquaredEuclidean;
use crate::engine::back::render::selective_systems::dynamic_chunks::data::Chunk;
use crate::engine::back::render::selective_systems::dynamic_chunks::DynamicChunks;
use crate::engine::back::render::selective_systems::dynamic_chunks::registry::{DataRegistry, LidRegistry};
use crate::engine::back::types::light::LightType;

impl DynamicChunks {
    pub fn add_light(&mut self, light: LightType) -> u32 {
        let position = [light.position[0], light.position[1], light.position[2]];
        let nearest = self.tree.nearest_one::<SquaredEuclidean>(&position);

        if nearest.distance <= self.chunks_size {
            for chunk_id in self.ids.get(&nearest.item).expect("Nearest not found").iter() {
                let chunk = self.chunks.get_mut(chunk_id).expect("chunk not found");

                if !chunk.private {
                    if chunk.loaded {
                        return self.add_loaded_l(*chunk_id, light);
                    }

                    return self.add_l(*chunk_id, light);
                }
            }
        }

        let chunk_id = self.new_chunk(position, false);
        self.add_l(chunk_id, light)
    }

    fn add_l(&mut self, chunk_id: u32, light: LightType) -> u32 {
        let chunk = self.chunks.get_mut(&chunk_id).expect("chunk not found");

        let id = self.data.lights.ids.get_id();

        chunk.lights.insert(id);
        self.data.lights.type_.insert(id, Some(light));
        self.data.lights.chunk.insert(id, chunk_id);
        self.data.lights.type_i.insert(id, 0);

        id
    }

    fn add_loaded_l(&mut self, chunk_id: u32, light: LightType) -> u32 {
        let chunk = self.chunks.get_mut(&chunk_id).expect("chunk not found");

        let id = self.data.lights.ids.get_id();

        let index = self.lid.lights.ids.get_id() as usize;
        self.lid.lights.type_[index] = light;

        chunk.lights.insert(id);
        self.data.lights.type_.insert(id, Some(light));
        self.data.lights.chunk.insert(id, chunk_id);
        self.data.lights.type_i.insert(id, index);
        self.data.lights.i_type.insert(index, id);

        id
    }

    pub fn light_changed(&mut self, id: &u32) {
        self.data.updater.lights.insert(*id);
    }

    pub fn get_light(&self, id: &u32,) -> *const LightType {
        Self::get_type(id, &self.data.lights, &self.lid.lights)
    }

    pub fn get_mut_light(&mut self, id: &u32,) -> *mut LightType {
        Self::get_mut_type(id, &mut self.data.lights, &mut self.lid.lights)
    }

    fn switch_light_chunk(data: &mut DataRegistry<LightType>, chunk: &mut Chunk, chunk_id: &u32, id: &u32, loaded: bool, lid: &mut LidRegistry<LightType>) {
        if loaded && !chunk.loaded {
            Self::swap_remove_data(id, data, lid, &mut |_| {});
        } else if !loaded && chunk.loaded {
            Self::add_data(id, data, lid);
        }

        data.chunk.insert(*id, *chunk_id);
        chunk.lights.insert(*id);
    }

    pub fn verify_light(&mut self, id: &u32) {
        let pos;

        unsafe {
            pos = (*self.get_light(id)).position;
        }

        let pos = [pos[0], pos[1], pos[2]];

        let chunk_id = self.data.lights.chunk.get(id).expect("chunk not found");

        let chunk = self.chunks.get_mut(chunk_id).expect("chunk not found");

        if chunk.position[0] + self.chunks_size < pos[0]
            || chunk.position[1] + self.chunks_size < pos[1]
            || chunk.position[2] + self.chunks_size < pos[2]
            || chunk.position[0] - self.chunks_size > pos[0]
            || chunk.position[1] - self.chunks_size > pos[1]
            || chunk.position[2] - self.chunks_size > pos[2] {

            let loaded = chunk.loaded;
            chunk.lights.remove(id);

            let nearest = self.tree.nearest_one::<SquaredEuclidean>(&pos);

            if nearest.distance <= self.chunks_size {
                for chunk_id in self.ids.get(&nearest.item).expect("Nearest not found").iter() {
                    let chunk = self.chunks.get_mut(chunk_id).expect("chunk not found");

                    if !chunk.private {
                        Self::switch_light_chunk(&mut self.data.lights, chunk, chunk_id, id, loaded, &mut self.lid.lights);
                        return;
                    }
                }
            }

            let chunk_id = self.new_chunk(pos, false);
            let chunk = self.chunks.get_mut(&chunk_id).expect("chunk not found");

            Self::switch_light_chunk(&mut self.data.lights, chunk, &chunk_id, id, loaded, &mut self.lid.lights);
        }
    }
}