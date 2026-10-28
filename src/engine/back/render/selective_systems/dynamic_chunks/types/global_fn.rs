use std::collections::HashSet;
use glam::Vec3;
use crate::engine::back::render::selective_systems::dynamic_chunks::data::Chunk;
use crate::engine::back::render::selective_systems::dynamic_chunks::DynamicChunks;
use crate::engine::back::render::selective_systems::dynamic_chunks::registry::{DataRegistry, DataRegistryPlus, LidRegistry};

impl DynamicChunks {
    pub fn update(&mut self) {
        Self::updating_plus(&mut self.data.updater.objects, &mut self.data.objects, &mut self.lid.objects, |object| {
            object.old_center = object.center;
            object.old_rot = object.rot;
            object.old_size = object.size;
            object.old_color = object.color;
        });

        Self::updating(&mut self.data.updater.lights, &mut self.data.lights, &mut self.lid.lights, |light| {
            light.old_position = light.position;
            light.old_color = light.color;
            light.old_rotation = light.rotation;
            light.old_attributes = light.attributes;
        });
    }

    pub fn is_inside_chunk(chunks_size: f32, chunk: &Chunk, position: &[f32; 3]) -> bool {
        if chunk.manual {
            let chunk_req = chunk.chunk_req.as_ref().expect("trust me it works");
            let point = Vec3::from(*position);
            let distance = point.distance(Vec3::from(chunk.position));

            if chunk_req.radius >= distance {
                for plane in chunk_req.planes.iter() {
                    if plane.normal.dot(point) + plane.d > 0.0 {
                        return false;
                    }
                }

                return true;
            }

            false
        } else {
            let vec1 = Vec3::from(*position);
            let vec2 = Vec3::from(chunk.position);

            chunks_size >= vec1.distance(vec2)
        }
    }

    fn updating_plus<F, T, const N: usize>(updater: &mut HashSet<u32>, data: &mut DataRegistryPlus<T, N>, lid: &mut LidRegistry<T>, mut f: F) where F: FnMut(&mut T) {
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

    pub fn get_type_plus<T, const N: usize>(id: u32, data: &DataRegistryPlus<T, N>, lid: &LidRegistry<T>) -> *const T {
        let t;
        let option = &data.type_.get(&id).expect("type not found, you are useless if you somehow got this error, CRY");

        if let Some(ty) = option {
            t = ty

        } else {
            let index = data.type_i.get(&id).expect("naah");
            t = &lid.type_[*index];
        }

        t as *const T
    }

    pub fn get_type<T>(id: u32, data: &DataRegistry<T>, lid: &LidRegistry<T>) -> *const T {
        let t;
        let option = data.type_.get(&id).expect("type not found, you are useless if you somehow got this error, CRY");

        if let Some(ty) = option {
            t = ty;

        } else {
            let index = data.type_i.get(&id).expect("naah");
            t = &lid.type_[*index];
        }

        t as *const T
    }

    pub fn get_mut_type_plus<T, const N: usize>(id: u32, data: &mut DataRegistryPlus<T, N>, lid: &mut LidRegistry<T>) -> *mut T {
        let t;
        let option = &mut data.type_.get_mut(&id).expect("type not found, you are useless if you somehow got this error, CRY");

        if let Some(ty) = option {
            t = ty;

        } else {
            let index = data.type_i.get(&id).expect("naah");
            t = &mut lid.type_[*index];
        }

        t as *mut T
    }

    pub fn get_mut_type<T>(id: u32, data: &mut DataRegistry<T>, lid: &mut LidRegistry<T>) -> *mut T {
        let t;
        let option = data.type_.get_mut(&id).expect("type not found, you are useless if you somehow got this error, CRY");

        if let Some(ty) = option {
            t = ty;

        } else {
            let index = data.type_i.get(&id).expect("naah");
            t = &mut lid.type_[*index];
        }

        t as *mut T
    }
}
