use std::collections::HashSet;
use indexmap::IndexSet;
use crate::engine::back::render::selective_systems::dynamic_chunks::data::{Chunk, Data, LID};
use crate::engine::back::render::selective_systems::dynamic_chunks::DynamicChunks;
use crate::engine::back::render::selective_systems::dynamic_chunks::registry::{DataRegistry, DataRegistryArray, DataRegistryPlus, LidRegistry, LidRegistryArray};

impl DynamicChunks {
    pub fn iter_add_chunks<F>(&mut self, visible_chunks: &IndexSet<u32>, mut f: F) where F: FnMut(&mut LID, &mut Chunk, &mut Data) {
        let to_add = visible_chunks.difference(&self.lid.chunks).cloned().collect::<IndexSet<_>>();

        for id in &to_add {
            let chunks = self.ids.get(id).expect("chunks not found");
            let mut is = self.new_chunks_ids.contains(id);

            for chunk_id in chunks.iter() {
                if is {
                    let cks = self.new_chunks.get_mut(id).expect("nahh man");

                    if cks.contains(chunk_id) {
                        cks.remove(chunk_id);
                        if cks.is_empty() {
                            self.new_chunks.swap_remove(id);
                            self.new_chunks_ids.swap_remove(id);
                            is = false;
                        }
                    }
                }

                let chunk = self.chunks.get_mut(chunk_id).expect("chunk not found");

                if !chunk.loaded {
                    chunk.loaded = true;
                    self.lid.chunks.insert(*chunk_id);

                    f(&mut self.lid, chunk, &mut self.data);
                }
            }
        }

        let to_add = visible_chunks.intersection(&self.new_chunks_ids).cloned().collect::<IndexSet<_>>();

        for id in &to_add {
            let chunks = self.new_chunks.get_mut(id).expect("chunk not found");

            for chunk_id in chunks.iter() {
                let chunk = self.chunks.get_mut(chunk_id).expect("chunk not found");

                if !chunk.loaded {
                    self.lid.chunks.insert(*chunk_id);

                    f(&mut self.lid, chunk, &mut self.data);
                }
            }
        }
    }

    pub fn iter_add_data<T>(chunk_type: &mut HashSet<u32>, data: &mut DataRegistry<T>, lid: &mut LidRegistry<T>) {
        for type_id in chunk_type.iter() {
            Self::add_data(type_id, data, lid);
        }
    }

    pub fn add_data<T>(type_id: &u32, data: &mut DataRegistry<T>, lid: &mut LidRegistry<T>) {
        let type_ = data.type_.get_mut(type_id).expect("no type found").take().expect("no object found in option");
        let i = lid.ids.get_id() as usize;
        lid.type_[i] = type_;

        *data.type_i.get_mut(type_id).expect("no no") = i;
        data.i_type.insert(i, *type_id);
    }


    pub fn iter_add_data_plus<F, T, const N: usize>(chunk_type: &mut HashSet<u32>, data: &mut DataRegistryPlus<T, N>, lid: &mut LidRegistry<T>, f: &mut F) where F: FnMut(&mut T, &[u32; N]) {
        for type_id in chunk_type.iter() {
            Self::add_data_plus(type_id, data, lid, f);
        }
    }

    pub fn add_data_plus<F, T, const N: usize>(type_id: &u32, data: &mut DataRegistryPlus<T, N>, lid: &mut LidRegistry<T>, f: &mut F) where F: FnMut(&mut T, &[u32; N]) {
        let data_ = data.type_.get_mut(type_id).expect("no type found");
        let mut type_ = data_.0.take().expect("no object found in option");
        let i = lid.ids.get_id() as usize;

        f(&mut type_, &data_.1);
        lid.type_[i] = type_;

        *data.type_i.get_mut(type_id).expect("no no") = i;
        data.i_type.insert(i, *type_id);
    }


    pub fn iter_add_data_array<T>(chunk_type: &mut HashSet<u32>, data: &mut DataRegistryArray<T>, lid: &mut LidRegistryArray<T>) {
        for type_id in chunk_type.iter() {
            Self::add_data_array(type_id, data, lid);
        }
    }

    pub fn add_data_array<T>(type_id: &u32, data: &mut DataRegistryArray<T>, lid: &mut LidRegistryArray<T>) {
        let type_vec = data.type_.get_mut(type_id).expect("no type found");

        let ids = lid.ids.get_id(type_vec.len() as u32);
        let mut start = ids.start;

        for type_ in type_vec.drain(..) {
            let i = start as usize;
            start += 1;

            lid.type_[i] = type_;
        }

        *data.type_i.get_mut(type_id).expect("no no") = [ids.start as usize, ids.end as usize];
    }
}