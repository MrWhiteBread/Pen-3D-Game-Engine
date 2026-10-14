use std::collections::HashSet;
use kiddo::SquaredEuclidean;
use crate::engine::back::render::selective_systems::dynamic_chunks::data::{Chunk, Data, LID};
use crate::engine::back::render::selective_systems::dynamic_chunks::DynamicChunks;
use crate::engine::back::render::selective_systems::dynamic_chunks::registry::{ DataRegistryArray, LidRegistryArray};
use crate::engine::back::types::object::Object;
use crate::engine::front::components::mesh::Mesh;

#[allow(dead_code)]
impl DynamicChunks {
    pub fn add_object(&mut self, object: Object) -> u32 {
        let position = [object.center[0], object.center[1], object.center[2]];
        let nearest = self.tree.nearest_one::<SquaredEuclidean>(&position);

        if nearest.distance <= self.chunks_size {
            for chunk_id in self.ids.get(&nearest.item).expect("Nearest not found").iter() {
                let chunk = self.chunks.get_mut(chunk_id).expect("chunk not found");

                if !chunk.private {
                    if chunk.loaded {
                        return self.add_loaded(*chunk_id, object);
                    }

                    return self.add(*chunk_id, object);
                }
            }
        }

        let chunk_id = self.new_chunk(position, false);
        self.add(chunk_id, object)
    }

    fn add(&mut self, chunk_id: u32, object: Object) -> u32 {
        let chunk = self.chunks.get_mut(&chunk_id).expect("chunk not found");

        let id = self.data.objects.ids.get_id();

        chunk.objects.insert(id);
        self.data.objects.type_.insert(id, Some(object));
        self.data.objects.chunk.insert(id, chunk_id);
        self.data.objects.type_i.insert(id, 0);
        self.data.objects.payload.insert(id, Vec::from([Vec::new(), Vec::new()]));

        id
    }

    fn add_loaded(&mut self, chunk_id: u32, object: Object) -> u32 {
        let chunk = self.chunks.get_mut(&chunk_id).expect("chunk not found");

        let id = self.data.objects.ids.get_id();
        self.data.objects.type_.insert(id, None);

        let index = self.lid.objects.ids.get_id() as usize;
        self.lid.objects.type_[index] = object;

        self.data.objects.type_i.insert(id, index);
        self.data.objects.chunk.insert(id, chunk_id);
        self.data.objects.i_type.insert(index, id);
        self.data.objects.payload.insert(id, Vec::from([Vec::new(), Vec::new()]));
        chunk.objects.insert(id);

        id
    }

    pub fn set_object_mesh(&mut self, id: u32, mesh_ids: &[u32; 2]) {
        let vec = self.data.objects.payload.get_mut(&id).expect("object not found");
        vec[0] = Vec::from([mesh_ids[0]]);
        vec[1] = Vec::from([mesh_ids[1]]);

        let chunk_id = self.data.objects.chunk.get(&id).expect("chunk id not found");
        let chunk = self.chunks.get(chunk_id).expect("chunk not found");

        if chunk.loaded {
            Self::add_data_array(
                mesh_ids[0],
                &mut self.data.vertices,
                &mut self.lid.vertices,
            );

            Self::add_data_array(
                mesh_ids[1],
                &mut self.data.indices,
                &mut self.lid.indices,
            );

            let index = self.data.objects.type_i.get(&id).expect("type not found");
            let object = &mut self.lid.objects.type_[*index];

            let ver = self.data.vertices.type_i.get(&mesh_ids[0]).expect("no vertex id");
            object.base_vertex = ver[0] as i32;

            let ind = self.data.indices.type_i.get(&mesh_ids[1]).expect("no index id");

            object.first_index = ind[0] as u32;
            object.index_count = (ind[1] - ind[0]) as u32;
        }
    }

    pub fn object_changed(&mut self, id: u32) {
        self.data.updater.objects.insert(id);
    }

    pub fn get_object(&self, id: u32) -> *const Object {
        Self::get_type_plus(id, &self.data.objects, &self.lid.objects)
    }

    pub fn get_mut_object(&mut self, id: u32) -> *mut Object {
        Self::get_mut_type_plus(id, &mut self.data.objects, &mut self.lid.objects)
    }

    fn switch_object_chunk(data: &mut Data, chunk: &mut Chunk, chunk_id: u32, id: u32, loaded: bool, lid: &mut LID) {
        if loaded && !chunk.loaded {
            Self::swap_remove_data_plus(id, &mut data.objects, &mut lid.objects, &mut |object, payload| {
                object.status = 0;

                if let Some(vertices_id) = payload[0].get(0) {
                    Self::remove_data_array(
                        *vertices_id,
                        &mut data.vertices,
                        &mut lid.vertices,
                        &mut |_| {}
                    );
                }

                if let Some(indices_id) = payload[1].get(0) {
                    Self::remove_data_array(
                        *indices_id,
                        &mut data.indices,
                        &mut lid.indices,
                        &mut |_| {}
                    );
                }
            });
        } else if !loaded && chunk.loaded {
            Self::add_data_plus(id, &mut data.objects, &mut lid.objects, &mut |object, payload| {
                if let Some(vertices_id) = payload[0].get(0) {
                    let ver = data.vertices.type_i.get(&vertices_id).expect("no vertex id");
                    object.base_vertex = ver[0] as i32;

                    Self::add_data_array(
                        *vertices_id,
                        &mut data.vertices,
                        &mut lid.vertices,
                    );
                }

                if let Some(indices_id) = payload[1].get(0) {
                    let ind = data.indices.type_i.get(&indices_id).expect("no index id");

                    object.first_index = ind[0] as u32;
                    object.index_count = (ind[1] - ind[0]) as u32;

                    Self::add_data_array(
                        *indices_id,
                        &mut data.indices,
                        &mut lid.indices,
                    );
                }
            });
        }

        data.objects.chunk.insert(id, chunk_id);
        chunk.objects.insert(id);
    }

    pub fn verify_model(&mut self, id: u32) {
        let pos;

        unsafe {
            pos = (*self.get_object(id)).center;
        }

        let pos = [pos[0], pos[1], pos[2]];

        let chunk_id = self.data.objects.chunk.get(&id).expect("chunk not found");

        let chunk = self.chunks.get_mut(chunk_id).expect("chunk not found");

        if chunk.position[0] + self.chunks_size < pos[0]
            || chunk.position[1] + self.chunks_size < pos[1]
            || chunk.position[2] + self.chunks_size < pos[2]
            || chunk.position[0] - self.chunks_size > pos[0]
            || chunk.position[1] - self.chunks_size > pos[1]
            || chunk.position[2] - self.chunks_size > pos[2] {

            let loaded = chunk.loaded;
            chunk.objects.remove(&id);

            let nearest = self.tree.nearest_one::<SquaredEuclidean>(&pos);

            if nearest.distance <= self.chunks_size {
                for chunk_id in self.ids.get(&nearest.item).expect("Nearest not found").iter() {
                    let chunk = self.chunks.get_mut(chunk_id).expect("chunk not found");

                    if !chunk.private {
                        Self::switch_object_chunk(&mut self.data, chunk, *chunk_id, id, loaded, &mut self.lid);
                        return;
                    }
                }
            }

            let chunk_id = self.new_chunk(pos, false);
            let chunk = self.chunks.get_mut(&chunk_id).expect("chunk not found");

            Self::switch_object_chunk(&mut self.data, chunk, chunk_id, id, loaded, &mut self.lid);
        }
    }
}