mod runners;
mod registry;
mod data;
mod create;
mod types;

use std::collections::{BTreeSet, HashMap, HashSet};
use indexmap::{IndexMap, IndexSet};
use kiddo::SquaredEuclidean;
use wgpu::{Buffer, Extent3d, Queue};
use crate::engine::back::render::selective_systems::dynamic_chunks::data::*;
use crate::engine::back::render::selective_systems::id::{Id32};
use crate::engine::back::staging_buffers::StagingBuffers;

pub struct DynamicChunks {
    // lid - loaded ids (l-id)

    chunks_size: f32,

    tree: kiddo::float::kdtree::KdTree<f32, u32, 3, 64, u32>,
    ids: HashMap<u32, HashSet<u32>>,
    chunks: HashMap<u32, Chunk>,
    new_chunks: IndexMap<u32, HashSet<u32>>,
    new_chunks_ids: IndexSet<u32>,

    gods: HashMap<u32, [f32; 4]>, // x,  y,  z,  max distance

    data: Data,
    lid: LID,
    usage: Usage,

    tree_ids: Id32,
    chunks_ids: Id32,
    gods_ids: Id32,
}

impl DynamicChunks {
    pub fn new(chunks_size: f32) -> Self {
        let default_space = [
            1000,
            200_000,
            1000_000,
            1000_000,
        ];

        let atlas_pixel_size = 8192;
        let atlas_chunks_size = 128;

        let atlas_size = Extent3d {
            width: atlas_pixel_size / atlas_chunks_size,
            height: atlas_pixel_size / atlas_chunks_size,
            depth_or_array_layers: 1,
        };

        Self {
            chunks_size,

            tree: kiddo::float::kdtree::KdTree::new(),
            ids: HashMap::new(),
            chunks: HashMap::new(),
            new_chunks: IndexMap::new(),
            new_chunks_ids: IndexSet::new(),

            gods: HashMap::new(),

            data: Data::new(),
            lid: LID::new(default_space, atlas_size, atlas_chunks_size as usize),
            usage: Usage::new(),

            tree_ids: Id32::new(),
            chunks_ids: Id32::new(),
            gods_ids: Id32::new(),
        }
    }

    pub fn set_staging(&mut self, staging_buffers: &mut StagingBuffers) {
        let mut visible_chunks = IndexSet::new();

        for (_, god) in &self.gods {
            let center = [
                god[0],
                god[1],
                god[2],
            ];

            let radius_squared = god[3] * god[3];

            let sphere: BTreeSet<_> = self.tree
                .within::<SquaredEuclidean>(&center, radius_squared)
                .into_iter()
                .map(|id| id.item)
                .collect();

            visible_chunks.extend(sphere);
        }

        self.iter_remove_chunks(&visible_chunks,  |lid, data, chunk| {
            Self::iter_swap_remove_data(
                &mut chunk.lights,
                &mut data.lights,
                &mut lid.lights,
                &mut |_| {}
            );


            Self::iter_swap_remove_data_plus(
                &mut chunk.objects,
                &mut data.objects,
                &mut lid.objects,
                &mut |object| {
                    object.status = 0;
                }
            );


            Self::iter_remove_data_array(
                &mut chunk.vertices,
                &mut data.vertices,
                &mut lid.vertices,
                &mut |_| {}
            );


            Self::iter_remove_data_array(
                &mut chunk.indices,
                &mut data.indices,
                &mut lid.indices,
                &mut |_| {}
            );
        });

        self.iter_add_chunks(&visible_chunks,  |lid, chunk, data| {
            Self::iter_add_data(
                &mut chunk.lights,
                &mut data.lights,
                &mut lid.lights,
            );


            Self::iter_add_data_array(
                &mut chunk.vertices,
                &mut data.vertices,
                &mut lid.vertices,
            );


            Self::iter_add_data_array(
                &mut chunk.indices,
                &mut data.indices,
                &mut lid.indices,
            );


            Self::iter_add_data_plus(
                &mut chunk.objects,
                &mut data.objects,
                &mut lid.objects,
                &mut |object, ids| {
                    let ver = data.vertices.type_i.get(&ids[0]);
                    let ind = data.indices.type_i.get(&ids[1]);

                    if let Some(ver) = ver {
                        object.base_vertex = ver[0] as i32;
                    }

                    if let Some(ind) = ind {
                        object.first_index = ind[0] as u32;
                        object.index_count = (ind[1] - ind[0]) as u32;
                    }
                }
            );
        });

        fn write_data<T: bytemuck::Pod>(queue: &Queue, buffer: &Buffer, array: &Vec<T>, count: &mut u32, size: usize) {
            queue.write_buffer(
                buffer,
                0,
                bytemuck::cast_slice(&array[..size]),
            );

            *count = size as u32;
        }

        write_data(&staging_buffers.queue, &staging_buffers.light_staging, &self.lid.lights.type_, &mut staging_buffers.light_count, self.lid.lights.ids.size());
        write_data(&staging_buffers.queue, &staging_buffers.object_staging, &self.lid.objects.type_, &mut staging_buffers.object_count, self.lid.objects.ids.size());
        write_data(&staging_buffers.queue, &staging_buffers.vertices_staging, &self.lid.vertices.type_, &mut staging_buffers.vertices_count, self.lid.vertices.ids.size());
        write_data(&staging_buffers.queue, &staging_buffers.indices_staging, &self.lid.indices.type_, &mut staging_buffers.indices_count, self.lid.indices.ids.size());

        self.usage.vertices_len = self.lid.vertices.ids.len() as u32;
        self.usage.indices_len = self.lid.indices.ids.len() as u32;
        self.usage.objects_len = self.lid.objects.ids.len() as u32;
        self.usage.lights_len = self.lid.lights.ids.len() as u32;

        self.usage.vertices_size = self.lid.vertices.ids.size() as u32;
        self.usage.indices_size = self.lid.indices.ids.size() as u32;
        self.usage.objects_size = self.lid.objects.ids.size() as u32;
        self.usage.lights_size = self.lid.lights.ids.size() as u32;

        self.update();
    }
    
    pub fn get_usage(&self) -> Usage {
        self.usage.clone()
    }
}

#[allow(dead_code)]
pub struct Usage {
    pub vertices_len: u32,
    pub indices_len: u32,
    pub objects_len: u32,
    pub lights_len: u32,

    pub vertices_size: u32,
    pub indices_size: u32,
    pub objects_size: u32,
    pub lights_size: u32,
}

impl Usage {
    pub fn new() -> Self {
        Self {
            vertices_len: 0,
            indices_len: 0,
            objects_len: 0,
            lights_len: 0,

            vertices_size: 0,
            indices_size: 0,
            objects_size: 0,
            lights_size: 0,
        }
    }
    
    pub fn clone(&self) -> Self {
        Self {
            vertices_len: self.vertices_len,
            indices_len: self.indices_len,
            objects_len: self.objects_len,
            lights_len: self.lights_len,
            
            vertices_size: self.vertices_size,
            indices_size: self.indices_size,
            objects_size: self.objects_size,
            lights_size: self.lights_size,
        }
    }
}