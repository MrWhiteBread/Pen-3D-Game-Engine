use std::collections::HashSet;
use indexmap::IndexSet;
use crate::engine::back::render::selective_systems::dynamic_chunks::data::{Chunk, Data, LID};
use crate::engine::back::render::selective_systems::dynamic_chunks::DynamicChunks;
use crate::engine::back::render::selective_systems::dynamic_chunks::registry::{DataRegistry, DataRegistryArray, DataRegistryPlus, LidRegistry, LidRegistryArray};

#[allow(dead_code)]
impl DynamicChunks {
    pub fn iter_remove_chunks<F>(&mut self, visible_chunks: &IndexSet<u32>, mut f: F) where F: FnMut(&mut LID, &mut Data, &mut Chunk) {
        let to_remove = self.lid.chunks.difference(visible_chunks).cloned().collect::<Vec<_>>();

        for id in &to_remove {
            let chunks = self.ids.get(id).expect("chunks not found");

            for chunk_id in chunks.iter() {
                let chunk = self.chunks.get_mut(chunk_id).expect("chunk not found");

                if chunk.loaded {
                    chunk.loaded = false;
                    self.lid.chunks.swap_remove(chunk_id);

                    f(&mut self.lid, &mut self.data, chunk);
                }
            }
        }
    }

    pub fn iter_remove_data<F, T: Clone>(chunk_type: &mut HashSet<u32>, data: &mut DataRegistry<T>, lid: &mut LidRegistry<T>, f: &mut F)
    where F: FnMut(&mut T) {
        for type_id in chunk_type.iter() {
            Self::remove_data(type_id, data, lid, f);
        }
    }
    
    pub fn remove_data<F, T: Clone>(type_id: &u32, data: &mut DataRegistry<T>, lid: &mut LidRegistry<T>, f: &mut F)
    where F: FnMut(&mut T) {
        let type_ = data.type_.get_mut(type_id).expect("type not found");
        let type_i = data.type_i.get(type_id).expect("type id not found");

        *type_ = Some(lid.type_[*type_i].clone());

        let type_b = &mut lid.type_[*type_i];
        f(type_b);

        lid.ids.remove(&(*type_i as u32));
        data.i_type.remove(type_i);
    }
    

    pub fn iter_remove_data_plus<F, T: Clone, const N: usize>(chunk_type: &mut HashSet<u32>, data: &mut DataRegistryPlus<T, N>, lid: &mut LidRegistry<T>, f: &mut F)
    where F: FnMut(&mut T) {
        for type_id in chunk_type.iter() {
            Self::remove_data_plus(type_id, data, lid, f);
        }
    }

    pub fn remove_data_plus<F, T: Clone, const N: usize>(type_id: &u32, data: &mut DataRegistryPlus<T, N>, lid: &mut LidRegistry<T>, f: &mut F)
    where F: FnMut(&mut T) {
        let type_ = data.type_.get_mut(type_id).expect("type not found");
        let type_i = data.type_i.get(type_id).expect("type id not found");

        type_.0 = Some(lid.type_[*type_i].clone());

        let type_b = &mut lid.type_[*type_i];
        f(type_b);

        lid.ids.remove(&(*type_i as u32));
        data.i_type.remove(type_i);
    }
    

    pub fn iter_swap_remove_data<F, T: Clone>(chunk_type: &mut HashSet<u32>, data: &mut DataRegistry<T>, lid: &mut LidRegistry<T>, f: &mut F)
    where F: FnMut(&mut T) {
        for type_id in chunk_type.iter() {
            Self::swap_remove_data(type_id, data, lid, f);
        }
    }

    pub fn swap_remove_data<F, T: Clone>(type_id: &u32, data: &mut DataRegistry<T>, lid: &mut LidRegistry<T>, f: &mut F)
    where F: FnMut(&mut T) {
        let type_ = data.type_.get_mut(type_id).expect("type not found");
        let type_i = data.type_i.get(type_id).expect("type id not found").clone();

        *type_ = Some(lid.type_[type_i].clone());

        let last = lid.ids.last() as usize;

        if last != type_i {
            lid.type_[type_i] = lid.type_[last].clone();

            let type_b = &mut lid.type_[last];
            f(type_b);

            let id = data.i_type.remove(&last).expect("last id not found");
            *data.type_i.get_mut(&id).expect("no no") = type_i;
            *data.i_type.get_mut(&type_i).expect("no no") = id;

            lid.ids.remove(&(last as u32));

        } else {
            let type_b = &mut lid.type_[type_i];
            f(type_b);

            lid.ids.remove(&(type_i as u32));
            data.i_type.remove(&type_i);
        }
    }
    
    

    pub fn iter_swap_remove_data_plus<F, T: Clone, const N: usize>(chunk_type: &mut HashSet<u32>, data: &mut DataRegistryPlus<T, N>, lid: &mut LidRegistry<T>, f: &mut F)
    where F: FnMut(&mut T) {
        for type_id in chunk_type.iter() {
            let type_ = &mut data.type_.get_mut(type_id).expect("type not found").0;
            let type_i = data.type_i.get(type_id).expect("type id not found").clone();

            *type_ = Some(lid.type_[type_i].clone());

            let last = lid.ids.last() as usize;

            if last != type_i {
                lid.type_[type_i] = lid.type_[last].clone();

                let type_b = &mut lid.type_[last];
                f(type_b);

                let id = data.i_type.remove(&last).expect("last id not found");
                *data.type_i.get_mut(&id).expect("no no") = type_i;
                *data.i_type.get_mut(&type_i).expect("no no") = id;

                lid.ids.remove(&(last as u32));

            } else {
                let type_b = &mut lid.type_[type_i];
                f(type_b);

                lid.ids.remove(&(type_i as u32));
                data.i_type.remove(&type_i);
            }
        }
    }

    pub fn swap_remove_data_plus<F, T: Clone, const N: usize>(type_id: &u32, data: &mut DataRegistryPlus<T, N>, lid: &mut LidRegistry<T>, f: &mut F)
    where F: FnMut(&mut T) {
        let type_ = &mut data.type_.get_mut(type_id).expect("type not found").0;
        let type_i = data.type_i.get(type_id).expect("type id not found").clone();

        *type_ = Some(lid.type_[type_i].clone());

        let last = lid.ids.last() as usize;

        if last != type_i {
            lid.type_[type_i] = lid.type_[last].clone();

            let type_b = &mut lid.type_[last];
            f(type_b);

            let id = data.i_type.remove(&last).expect("last id not found");
            *data.type_i.get_mut(&id).expect("no no") = type_i;
            *data.i_type.get_mut(&type_i).expect("no no") = id;

            lid.ids.remove(&(last as u32));

        } else {
            let type_b = &mut lid.type_[type_i];
            f(type_b);

            lid.ids.remove(&(type_i as u32));
            data.i_type.remove(&type_i);
        }
    }

    pub fn iter_remove_data_array<F, T: Clone>(chunk_type: &mut HashSet<u32>, data: &mut DataRegistryArray<T>,  lid: &mut LidRegistryArray<T>, f: &mut F)
    where F: FnMut(&mut T) {
        for type_id in chunk_type.iter() {
            Self::remove_data_array(type_id, data, lid, f);
        }
    }

    pub fn remove_data_array<F, T: Clone>(type_id: &u32, data: &mut DataRegistryArray<T>,  lid: &mut LidRegistryArray<T>, f: &mut F)
    where F: FnMut(&mut T) {
        let type_ = data.type_.get_mut(type_id).expect("type not found");
        let type_i = data.type_i.get(type_id).expect("type id not found");

        *type_ = Vec::new();

        for i in type_i[0]..type_i[1] {
            type_.push(lid.type_[i].clone());
            let type_b = &mut lid.type_[i];
            f(type_b);
        }
        lid.ids.remove(&[type_i[0] as u32, type_i[1] as u32]);
    }
}