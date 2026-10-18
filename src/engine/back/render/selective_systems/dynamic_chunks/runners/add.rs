use std::collections::HashSet;
use indexmap::IndexSet;
use crate::engine::back::render::selective_systems::dynamic_chunks::data::{Chunk, Data, LID};
use crate::engine::back::render::selective_systems::dynamic_chunks::DynamicChunks;
use crate::engine::back::render::selective_systems::dynamic_chunks::registry::{DataRegistry, DataRegistryArray, DataRegistryPlus, LidRegistry, LidRegistryArray};

impl DynamicChunks {
    pub fn iter_add_chunks<F>(&mut self, visible_chunks: &IndexSet<u32>, mut f: F) where F: FnMut(&mut LID, &mut Chunk, &mut Data) {
        let to_add = visible_chunks.difference(&self.lid.chunks).cloned().collect::<IndexSet<_>>();
        self.add_chunks(to_add, &mut f);

        let to_add = visible_chunks.intersection(&self.new_chunks).cloned().collect::<IndexSet<_>>();
        self.add_chunks(to_add, &mut f);
    }

    fn add_chunks<F>(&mut self, to_add: IndexSet<u32>, f: &mut F) where F: FnMut(&mut LID, &mut Chunk, &mut Data) {
        for chunk_id in &to_add {
            self.new_chunks.swap_remove(chunk_id);
            self.lid.chunks.insert(*chunk_id);

            let chunk = self.chunks.get_mut(chunk_id).expect("chunk not found");
            chunk.loaded = true;

            f(&mut self.lid, chunk, &mut self.data);
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