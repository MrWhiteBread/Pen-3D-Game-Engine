use crate::engine::back::render::selective_systems::dynamic_chunks::DynamicChunks;

mod model;
mod light;
mod global_fn;

#[allow(dead_code)]
impl DynamicChunks {
    pub fn set_god_dist(&mut self, god: &u32, dist: &f32) {
        if !self.gods.contains_key(god) {
            return;
        }

        (*self.gods.get_mut(god).expect("no god??"))[3] = *dist;
    }

    pub fn get_god(&mut self, god: &u32) -> [f32; 4] {
        if !self.gods.contains_key(god) {
            return [0.0; 4];
        }

        self.gods.get(god).expect("no god??").clone()
    }

    pub fn set_god_pos(&mut self, god: &u32, pos: &[f32; 3]) {
        if !self.gods.contains_key(god) {
            return;
        }

        (*self.gods.get_mut(god).expect("no god??"))[0] = pos[0];
        (*self.gods.get_mut(god).expect("no god??"))[1] = pos[1];
        (*self.gods.get_mut(god).expect("no god??"))[2] = pos[2];
    }

    pub fn set_god(&mut self, god: &u32, value: &[f32; 4]) {
        if !self.gods.contains_key(god) {
            return;
        }

        *self.gods.get_mut(god).expect("no god??") = *value;
    }

    pub fn add_god(&mut self, god: &[f32; 4]) -> u32 {
        let id = self.gods_ids.get_id();

        self.gods.insert(id, *god);
        id
    }

    pub fn remove_god(&mut self, god: &u32) -> [f32; 4] {
        if !self.gods.contains_key(god) {
            return [0.0; 4];
        }

        let values = self.gods.get(god).expect("no god??").clone();
        self.gods.remove(&god);
        self.gods_ids.remove(&god);
        values
    }

    pub fn clear_gods(&mut self) {
        self.gods.clear();
        self.gods_ids.clear();
    }
}