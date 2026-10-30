mod runners;
mod registry;
mod data;
mod types;

use std::collections::{BTreeSet, HashMap, HashSet};
use indexmap::{IndexMap, IndexSet};
use kiddo::SquaredEuclidean;
use wgpu::{Buffer, BufferSize, Extent3d, Queue};
use crate::engine::back::render::selective_systems::dynamic_chunks::data::*;
use crate::engine::back::render::selective_systems::id::{Id32};
use crate::engine::back::shared_data::{SharedData, StagingBuffers};

pub struct DynamicChunks {
    // lid - loaded ids (l-id)

    chunks_size: f32,

    auto_tree: kiddo::float::kdtree::KdTree<f32, u32, 3, 64, u32>,
    non_protected_tree: kiddo::float::kdtree::KdTree<f32, u32, 3, 64, u32>,
    non_private_tree: kiddo::float::kdtree::KdTree<f32, u32, 3, 64, u32>,

    chunks: HashMap<u32, Chunk>,
    new_chunks: IndexSet<u32>,

    gods: HashMap<u32, [f32; 4]>, // x,  y,  z,  max distance

    data: Data,
    lid: LID,
    usage: Usage,

    chunks_ids: Id32,
    gods_ids: Id32,
}

impl DynamicChunks {
    pub fn new(chunks_size: f32) -> Self {
        let default_space = [
            1_000,
            1_500_000,
            5_000_000,
            10_000_000,
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

            auto_tree: kiddo::float::kdtree::KdTree::new(),
            non_protected_tree: kiddo::float::kdtree::KdTree::new(),
            non_private_tree: kiddo::float::kdtree::KdTree::new(),

            chunks: HashMap::new(),
            new_chunks: IndexSet::new(),

            gods: HashMap::new(),

            data: Data::new(),
            lid: LID::new(default_space, atlas_size, atlas_chunks_size as usize),
            usage: Usage::new(),

            chunks_ids: Id32::new(),
            gods_ids: Id32::new(),
        }
    }

    pub fn set_staging(&mut self, shared_data: &mut SharedData) {
        let mut visible_chunks = IndexSet::new();

        for (_, god) in &self.gods {
            let center = [
                god[0],
                god[1],
                god[2],
            ];

            let radius_squared = god[3] * god[3];

            let sphere1: BTreeSet<_> = self.auto_tree
                .within::<SquaredEuclidean>(&center, radius_squared)
                .into_iter()
                .map(|id| id.item)
                .collect();

            let sphere2: BTreeSet<_> = self.non_private_tree
                .within::<SquaredEuclidean>(&center, radius_squared)
                .into_iter()
                .map(|id| id.item)
                .collect();

            visible_chunks.extend(sphere1);
            visible_chunks.extend(sphere2);
        }

        self.iter_remove_chunks(&visible_chunks);
        self.iter_add_chunks(&visible_chunks);

        fn get_size(chunk_size: &usize, remaining: &mut usize) -> usize {
            if *remaining < *chunk_size {
                let rem = *remaining;
                *remaining = 0;

                return rem
            }

            *remaining -= *chunk_size;
            *chunk_size
        }

        fn write_data<T: bytemuck::Pod>(queue: &Queue, buffer: &Buffer, array: &[T], count: &mut u32, dest_offset: &mut u64, size: usize, offset: usize) {
            let byte_size = size * size_of::<T>();
            let byte_offset = (offset * size_of::<T>()) as u64;

            if byte_size > 0 && let Some(mut view) = queue.write_buffer_with(
                buffer,
                0,
                BufferSize::new(byte_size as u64).expect("could not create buffer size"),
            ) {
                view.copy_from_slice(bytemuck::cast_slice(&array[offset..size + offset]));
            }

            *dest_offset = byte_offset;
            *count = size as u32;
        }

        let lights_size = self.lid.lights.ids.size();
        let objects_size = self.lid.objects.ids.size();
        let vertices_size = self.lid.vertices.ids.size();
        let indices_size = self.lid.indices.ids.size();

        let lights_slice = &self.lid.lights.type_;
        let objects_slice = &self.lid.objects.type_;
        let vertices_slice = &self.lid.vertices.type_;
        let indices_slice = &self.lid.indices.type_;

        let lights_chunk_size = lights_size.div_ceil(shared_data.staging_buffers.len());
        let objects_chunk_size = objects_size.div_ceil(shared_data.staging_buffers.len());

        let mut remaining_lights = lights_size;
        let mut remaining_objects = objects_size;

        std::thread::scope(|scope| {
            for (i, staging_buffers) in shared_data.staging_buffers.iter_mut().enumerate() {
                let queue = shared_data.queue.clone();

                let lights_size = get_size(&lights_chunk_size, &mut remaining_lights);
                let objects_size = get_size(&objects_chunk_size, &mut remaining_objects);

                let lights_offset = i * lights_chunk_size;
                let objects_offset = i * objects_chunk_size;

                scope.spawn(move || {
                    write_data(
                        &queue,
                        &staging_buffers.light_staging,
                        lights_slice,

                        &mut staging_buffers.light_count,
                        &mut staging_buffers.light_dest_offset,

                        lights_size,
                        lights_offset
                    );

                    write_data(
                        &queue,
                        &staging_buffers.object_staging,
                        objects_slice,

                        &mut staging_buffers.object_count,
                        &mut staging_buffers.object_dest_offset,

                        objects_size,
                        objects_offset
                    );
                });
            }
        });

        let staging_buffers = shared_data.staging_buffers.get_mut(0).expect("no staging buffers???");

        write_data(
            &shared_data.queue,
            &staging_buffers.vertices_staging,
            vertices_slice,

            &mut staging_buffers.vertices_count,
            &mut staging_buffers.vertices_dest_offset,

            vertices_size,
            0,
        );

        write_data(
            &shared_data.queue,
            &staging_buffers.indices_staging,
            indices_slice,

            &mut staging_buffers.indices_count,
            &mut staging_buffers.indices_dest_offset,

            indices_size,
            0
        );

        shared_data.objects_count = objects_size as u32;
        shared_data.lights_count = lights_size as u32;

        self.usage.chunks_len = self.lid.chunks.len() as u32;

        self.usage.vertices_len = self.lid.vertices.ids.len() as u32;
        self.usage.indices_len = self.lid.indices.ids.len() as u32;
        self.usage.objects_len = self.lid.objects.ids.len() as u32;
        self.usage.lights_len = self.lid.lights.ids.len() as u32;

        self.usage.vertices_size = vertices_size as u32;
        self.usage.indices_size = indices_size as u32;
        self.usage.objects_size = objects_size as u32;
        self.usage.lights_size = lights_size as u32;

        self.update();
    }

    pub fn get_usage(&self) -> Usage {
        self.usage.clone()
    }
}

#[allow(dead_code)]
pub struct Usage {
    pub chunks_len: u32,

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
            chunks_len: 0,

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
            chunks_len: self.chunks_len,

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