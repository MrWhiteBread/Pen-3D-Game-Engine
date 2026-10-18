use crate::engine::back::render::selective_systems::dynamic_chunks::DynamicChunks;
use crate::engine::back::render::selective_systems::dynamic_chunks::registry::{ DataRegistryArray};
use crate::engine::back::types::vertex::Vertex;

#[allow(dead_code)]
impl DynamicChunks {
    pub fn add_mesh(&mut self, vertices: Vec<Vertex>, indices: Vec<u32>) -> [u32; 2] {
        fn add_array<T>(data: &mut DataRegistryArray<T>, array: Vec<T>) -> u32 {
            let id = data.ids.get_id();
            
            data.type_.insert(id, array);
            data.type_i.insert(id, [0, 0]);
            data.activators.insert(id, 0);
            
            id
        }

        let v_id = add_array(&mut self.data.vertices, vertices);
        let i_id = add_array(&mut self.data.indices, indices);
        
        [v_id, i_id]
    }

    pub fn remove_mesh(&mut self, mesh_ids: &[u32; 2]) -> (Vec<Vertex>, Vec<u32>) {
        fn remove_array<T>(data: &mut DataRegistryArray<T>, id: u32) -> Vec<T> {
            data.ids.remove(id);

            let type_ =data.type_.remove(&id).expect("type not found");
            data.type_i.remove(&id);
            data.activators.remove(&id);

            type_
        }

        if self.data.vertices.type_.get(&mesh_ids[0]).is_none() {
            Self::remove_data_array(
                mesh_ids[0], 
                &mut self.data.vertices, 
                &mut self.lid.vertices, 
                &mut |_| {});
        }

        if self.data.indices.type_.get(&mesh_ids[1]).is_none() {
            Self::remove_data_array(
                mesh_ids[1],
                &mut self.data.indices,
                &mut self.lid.indices,
                &mut |_| {});
        }

        let vertices = remove_array(&mut self.data.vertices, mesh_ids[0]);
        let indices = remove_array(&mut self.data.indices, mesh_ids[1]);

        (vertices, indices)
    }
}