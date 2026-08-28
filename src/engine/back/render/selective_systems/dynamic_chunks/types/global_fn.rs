use std::collections::HashSet;
use crate::engine::back::render::selective_systems::dynamic_chunks::DynamicChunks;
use crate::engine::back::render::selective_systems::dynamic_chunks::registry::{DataRegistry, DataRegistryPlus, LidRegistry};

impl DynamicChunks {
    pub fn update(&mut self) {
        Self::updating_plus(&mut self.data.updater.objects, &mut self.data.objects, &mut self.lid.objects, |object| {
            object.old_center = object.center;
            object.old_rot = object.rot;
            object.old_size = object.size;
        });

        Self::updating(&mut self.data.updater.lights, &mut self.data.lights, &mut self.lid.lights, |light| {
            light.old_position = light.position;
            light.old_color = light.color;
            light.old_rotation = light.rotation;
            light.old_attributes = light.attributes;
            light.old_attributes2 = light.attributes2;
            light.old_attributes3 = light.attributes3;
            light.old_attributes4 = light.attributes4;
        });
    }

    fn updating_plus<F, T, const N: usize>(updater: &mut HashSet<u32>, data: &mut DataRegistryPlus<T, N>, lid: &mut LidRegistry<T>, mut f: F) where F: FnMut(&mut T) {
        for i in updater.drain() {
            let option = &mut data.type_.get_mut(&i).expect("no no?").0;

            if let Some(type_) = option {
                f(type_);

            } else {
                let index = data.type_i.get(&i).expect("exists");
                let type_ = &mut lid.type_[*index];
                f(type_);
            }
        }
    }

    fn updating<F, T>(updater: &mut HashSet<u32>, data: &mut DataRegistry<T>, lid: &mut LidRegistry<T>, mut f: F) where F: FnMut(&mut T) {
        for i in updater.drain() {

            let option = &mut data.type_.get_mut(&i).expect("no no?");
            if let Some(type_) = option {
                f(type_);

            } else {
                let index = data.type_i.get(&i).expect("exists");
                let type_ = &mut lid.type_[*index];
                f(type_);
            }
        }
    }

    pub fn get_type_plus<T, const N: usize>(id: &u32, data: &DataRegistryPlus<T, N>, lid: &LidRegistry<T>) -> *const T {
        let t;
        let option = &data.type_.get(id).expect("type not found, you are useless if you somehow got this error, CRY").0;

        if let Some(ty) = option {
            t = ty

        } else {
            let index = data.type_i.get(id).expect("naah");
            t = &lid.type_[*index];
        }

        t as *const T
    }

    pub fn get_type<T>(id: &u32, data: &DataRegistry<T>, lid: &LidRegistry<T>) -> *const T {
        let t;
        let option = data.type_.get(id).expect("type not found, you are useless if you somehow got this error, CRY");

        if let Some(ty) = option {
            t = ty;

        } else {
            let index = data.type_i.get(id).expect("naah");
            t = &lid.type_[*index];
        }

        t as *const T
    }

    pub fn get_mut_type_plus<T, const N: usize>(id: &u32, data: &mut DataRegistryPlus<T, N>, lid: &mut LidRegistry<T>) -> *mut T {
        let t;
        let option = &mut data.type_.get_mut(id).expect("type not found, you are useless if you somehow got this error, CRY").0;

        if let Some(ty) = option {
            t = ty;

        } else {
            let index = data.type_i.get(id).expect("naah");
            t = &mut lid.type_[*index];
        }

        t as *mut T
    }

    pub fn get_mut_type<T>(id: &u32, data: &mut DataRegistry<T>, lid: &mut LidRegistry<T>) -> *mut T {
        let t;
        let option = data.type_.get_mut(id).expect("type not found, you are useless if you somehow got this error, CRY");

        if let Some(ty) = option {
            t = ty;

        } else {
            let index = data.type_i.get(id).expect("naah");
            t = &mut lid.type_[*index];
        }

        t as *mut T
    }
}
