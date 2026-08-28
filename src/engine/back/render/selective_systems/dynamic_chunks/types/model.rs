use std::collections::HashSet;
use kiddo::SquaredEuclidean;
use crate::engine::back::render::selective_systems::dynamic_chunks::data::{Chunk, Data, LID};
use crate::engine::back::render::selective_systems::dynamic_chunks::DynamicChunks;
use crate::engine::back::render::selective_systems::dynamic_chunks::registry::{ DataRegistryArray, LidRegistryArray};
use crate::engine::back::types::object::Object;
use crate::engine::front::components::mesh::Mesh;

#[allow(dead_code)]
impl DynamicChunks {
    pub fn add_model(&mut self, object: Object, mesh: Mesh) -> u32 {
        let position = [object.center[0], object.center[1], object.center[2]];
        let nearest = self.tree.nearest_one::<SquaredEuclidean>(&position);

        if nearest.distance <= self.chunks_size {
            for chunk_id in self.ids.get(&nearest.item).expect("Nearest not found").iter() {
                let chunk = self.chunks.get_mut(chunk_id).expect("chunk not found");

                if !chunk.private {
                    if chunk.loaded {
                        return self.add_loaded(*chunk_id, object, mesh);
                    }

                    return self.add(*chunk_id, object, mesh);
                }
            }
        }

        let chunk_id = self.new_chunk(position, false);
        self.add(chunk_id, object, mesh)
    }

    fn add(&mut self, chunk_id: u32, object: Object, mesh: Mesh) -> u32 {
        let chunk = self.chunks.get_mut(&chunk_id).expect("chunk not found");
        let (vertices, indices) = mesh.unload();

        fn array_data<T>(data: &mut DataRegistryArray<T>, array: Vec<T>, chunk: &mut HashSet<u32>, chunk_id: &u32) -> u32 {
            let id = data.ids.get_id();

            chunk.insert(id);
            data.type_.insert(id, array);
            data.chunk.insert(id, *chunk_id);
            data.type_i.insert(id, [0, 0]);

            id
        }

        let v_id = array_data(&mut self.data.vertices, vertices, &mut chunk.vertices, &chunk_id);
        let i_id = array_data(&mut self.data.indices, indices, &mut chunk.indices, &chunk_id);

        let id = self.data.objects.ids.get_id();

        chunk.objects.insert(id);
        self.data.objects.type_.insert(id, (Some(object), [v_id, i_id]));
        self.data.objects.chunk.insert(id, chunk_id);
        self.data.objects.type_i.insert(id, 0);

        id
    }

    fn add_loaded(&mut self, chunk_id: u32, object: Object, mesh: Mesh) -> u32 {
        let chunk = self.chunks.get_mut(&chunk_id).expect("chunk not found");
        let (vertices, indices) = mesh.unload();

        fn array_data<T>(data: &mut DataRegistryArray<T>, lid: &mut LidRegistryArray<T>, mut array: Vec<T>, chunk: &mut HashSet<u32>, chunk_id: &u32) -> u32 {
            let id = data.ids.get_id();
            let index = lid.ids.get_id(array.len() as u32);
            let mut i = index.start as usize;

            chunk.insert(id);
            data.type_i.insert(id, [index.start as usize, index.end as usize]);
            data.chunk.insert(id, *chunk_id);
            data.type_.insert(id, Vec::new());

            for t in array.drain(..) {
                lid.type_[i] = t;
                i += 1;
            }

            id
        }

        let v_id = array_data(&mut self.data.vertices, &mut self.lid.vertices, vertices, &mut chunk.vertices, &chunk_id);
        let i_id = array_data(&mut self.data.indices, &mut self.lid.indices, indices, &mut chunk.indices, &chunk_id);

        let id = self.data.objects.ids.get_id();
        self.data.objects.type_.insert(id, (None, [v_id, i_id]));

        let index = self.lid.objects.ids.get_id() as usize;
        self.lid.objects.type_[index] = object;

        self.data.objects.type_i.insert(id, index);
        self.data.objects.chunk.insert(id, chunk_id);
        self.data.objects.i_type.insert(index, id);
        chunk.objects.insert(id);

        id
    }

    pub fn object_changed(&mut self, id: &u32) {
        self.data.updater.objects.insert(*id);
    }

    pub fn get_object(&self, id: &u32,) -> *const Object {
        Self::get_type_plus(id, &self.data.objects, &self.lid.objects)
    }

    pub fn get_mut_object(&mut self, id: &u32,) -> *mut Object {
        Self::get_mut_type_plus(id, &mut self.data.objects, &mut self.lid.objects)
    }

    fn switch_model_chunk(data: &mut Data, chunk: &mut Chunk, chunk_id: &u32, ids: &[u32; 3], loaded: bool, lid: &mut LID) {
        let id = ids[0];
        let v_id = ids[1];
        let i_id = ids[2];

        if loaded && !chunk.loaded {
            Self::swap_remove_data_plus(&id, &mut data.objects, &mut lid.objects, &mut |object| {
                object.status = 0;
            });
            Self::remove_data_array(&v_id, &mut data.vertices, &mut lid.vertices, &mut |_| {});
            Self::remove_data_array(&i_id, &mut data.indices, &mut lid.indices, &mut |_| {});

        } else if !loaded && chunk.loaded {
            Self::add_data_plus(&id, &mut data.objects, &mut lid.objects, &mut |object, ids| {
                let ver = data.vertices.type_i.get(&ids[0]);
                let ind = data.indices.type_i.get(&ids[1]);

                if let Some(ver) = ver {
                    object.base_vertex = ver[0] as i32;
                }

                if let Some(ind) = ind {
                    object.first_index = ind[0] as u32;
                    object.index_count = (ind[1] - ind[0]) as u32;
                }
            });
            Self::add_data_array(&v_id, &mut data.vertices, &mut lid.vertices);
            Self::add_data_array(&i_id, &mut data.indices, &mut lid.indices);
        }

        data.objects.chunk.insert(id, *chunk_id);
        chunk.objects.insert(id);
        chunk.vertices.insert(v_id);
        chunk.indices.insert(i_id);
    }

    pub fn verify_model(&mut self, id: &u32) {
        let pos;
        let (_, [v_id, i_id]) = self.data.objects.type_.get(id).expect("no object found");

        unsafe {
            pos = (*self.get_object(id)).center;
        }

        let pos = [pos[0], pos[1], pos[2]];

        let chunk_id = self.data.objects.chunk.get(id).expect("chunk not found");

        let chunk = self.chunks.get_mut(chunk_id).expect("chunk not found");

        if chunk.position[0] + self.chunks_size < pos[0]
            || chunk.position[1] + self.chunks_size < pos[1]
            || chunk.position[2] + self.chunks_size < pos[2]
            || chunk.position[0] - self.chunks_size > pos[0]
            || chunk.position[1] - self.chunks_size > pos[1]
            || chunk.position[2] - self.chunks_size > pos[2] {

            let loaded = chunk.loaded;
            chunk.objects.remove(id);
            chunk.vertices.remove(v_id);
            chunk.indices.remove(i_id);
            let ids = [*id, *v_id, *i_id];

            let nearest = self.tree.nearest_one::<SquaredEuclidean>(&pos);

            if nearest.distance <= self.chunks_size {
                for chunk_id in self.ids.get(&nearest.item).expect("Nearest not found").iter() {
                    let chunk = self.chunks.get_mut(chunk_id).expect("chunk not found");

                    if !chunk.private {
                        Self::switch_model_chunk(&mut self.data, chunk, chunk_id, &ids, loaded, &mut self.lid);
                        return;
                    }
                }
            }

            let chunk_id = self.new_chunk(pos, false);
            let chunk = self.chunks.get_mut(&chunk_id).expect("chunk not found");

            Self::switch_model_chunk(&mut self.data, chunk, &chunk_id, &ids, loaded, &mut self.lid);
        }
    }
}