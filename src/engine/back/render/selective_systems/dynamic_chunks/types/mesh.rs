use std::collections::HashSet;
use kiddo::SquaredEuclidean;
use crate::engine::back::render::selective_systems::dynamic_chunks::data::{Chunk, Data, LID};
use crate::engine::back::render::selective_systems::dynamic_chunks::DynamicChunks;
use crate::engine::back::render::selective_systems::dynamic_chunks::registry::{ DataRegistryArray, LidRegistryArray};
use crate::engine::back::types::object::Object;
use crate::engine::back::types::vertex::Vertex;
use crate::engine::front::components::mesh::Mesh;

#[allow(dead_code)]
impl DynamicChunks {
    pub fn add_mesh(&mut self, vertices: Vec<Vertex>, indices: Vec<u32>) -> [u32; 2] {
        fn array_data<T>(data: &mut DataRegistryArray<T>, array: Vec<T>) -> u32 {
            let id = data.ids.get_id();
            
            data.type_.insert(id, array);
            data.type_i.insert(id, [0, 0]);
            data.activators.insert(id, 0);
            
            id
        }

        let v_id = array_data(&mut self.data.vertices, vertices);
        let i_id = array_data(&mut self.data.indices, indices);
        
        [v_id, i_id]
    }
}