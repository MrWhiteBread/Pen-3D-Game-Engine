use std::collections::HashSet;
use indexmap::IndexSet;
use crate::engine::back::render::selective_systems::dynamic_chunks::data::{Chunk, Data, LID};
use crate::engine::back::render::selective_systems::dynamic_chunks::DynamicChunks;
use crate::engine::back::render::selective_systems::dynamic_chunks::registry::{DataRegistry, DataRegistryArray, DataRegistryPlus, LidRegistry, LidRegistryArray};

impl DynamicChunks {
    pub fn add_chunk_data(lid: &mut LID, data: &mut Data, chunk: &mut Chunk) {
        Self::iter_add_data(
            &mut chunk.chunk_data.lights,
            &mut data.lights,
            &mut lid.lights,
        );

        Self::iter_add_data_plus(
            &mut chunk.chunk_data.objects,
            &mut data.objects,
            &mut lid.objects,
            &mut |object, payload| {
                Self::add_object_data(
                    object,
                    payload,
                    &mut data.vertices,
                    &mut data.indices,
                    &mut lid.vertices,
                    &mut lid.indices
                );
            }
        );
    }

    pub fn iter_add_chunks(&mut self, visible_chunks: &IndexSet<u32>) {
        let to_add = visible_chunks.difference(&self.lid.chunks).cloned().collect::<IndexSet<_>>();
        self.add_chunks(to_add);

        let to_add = visible_chunks.intersection(&self.new_chunks).cloned().collect::<IndexSet<_>>();
        self.add_chunks(to_add);
    }

    fn add_chunks(&mut self, to_add: IndexSet<u32>) {
        for chunk_id in &to_add {
            let chunk = self.chunks.get_mut(chunk_id).expect("chunk not found");

            if chunk.loaded {
                continue;
            }
            
            self.new_chunks.swap_remove(chunk_id);
            self.lid.chunks.insert(*chunk_id);

            chunk.loaded = true;

            Self::add_chunk_data(&mut self.lid, &mut self.data, chunk);
        }
    }

    pub fn iter_add_data<T>(chunk_type: &mut HashSet<u32>, data: &mut DataRegistry<T>, lid: &mut LidRegistry<T>) {
        for type_id in chunk_type.iter() {
            Self::add_data(*type_id, data, lid);
        }
    }

    pub fn add_data<T>(type_id: u32, data: &mut DataRegistry<T>, lid: &mut LidRegistry<T>) {
        let type_ = data.type_.get_mut(&type_id).expect("no type found").take().expect("no object found in option");
        let i = lid.ids.get_id() as usize;
        lid.type_[i] = type_;

        *data.type_i.get_mut(&type_id).expect("no no") = i;
        data.i_type.insert(i, type_id);
    }


    pub fn iter_add_data_plus<F, T, const N: usize>(chunk_type: &mut HashSet<u32>, data: &mut DataRegistryPlus<T, N>, lid: &mut LidRegistry<T>, f: &mut F)
    where F: FnMut(&mut T, &[Option<u32>; N]) {
        for type_id in chunk_type.iter() {
            Self::add_data_plus(*type_id, data, lid, f);
        }
    }

    pub fn add_data_plus<F, T, const N: usize>(type_id: u32, data: &mut DataRegistryPlus<T, N>, lid: &mut LidRegistry<T>, f: &mut F) 
    where F: FnMut(&mut T, &[Option<u32>; N]) {
        let data_ = data.type_.get_mut(&type_id).expect("no type found");
        let mut type_ = data_.take().expect("no type found in option");
        let payload = data.payload.get(&type_id).expect("no payload found");
        let i = lid.ids.get_id() as usize;

        f(&mut type_, payload);
        lid.type_[i] = type_;

        *data.type_i.get_mut(&type_id).expect("no no") = i;
        data.i_type.insert(i, type_id);
    }

    pub fn add_data_array<T>(type_id: u32, data: &mut DataRegistryArray<T>, lid: &mut LidRegistryArray<T>) {
        let act_count = data.activators.get_mut(&type_id).expect("activator not found");
        *act_count += 1;
        
        if *act_count != 1 {
            return;
        }
        
        let type_vec = data.type_.get_mut(&type_id).expect("no type found");

        let ids = lid.ids.get_id(type_vec.len() as u32);
        let mut start = ids.start;

        for type_ in type_vec.drain(..) {
            let i = start as usize;
            start += 1;

            lid.type_[i] = type_;
        }

        *data.type_i.get_mut(&type_id).expect("no no") = [ids.start as usize, ids.end as usize];
    }
}