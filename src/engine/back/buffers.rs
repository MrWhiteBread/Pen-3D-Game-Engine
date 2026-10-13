use std::sync::Arc;
use arc_swap::ArcSwap;
use wgpu::{AdapterInfo, AddressMode, Buffer, BufferAddress, BufferDescriptor, BufferUsages, Device, Extent3d, FilterMode, MipmapFilterMode, Queue, Sampler, TextureView};
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::wgt::SamplerDescriptor;
use crate::engine::back::staging_buffers::StagingBuffers;
use crate::engine::back::types::bone::Bone;
use crate::engine::back::types::camera::{CameraUniform, Plane};
use crate::engine::back::types::draw_indexed_indirect_args::DrawIndexedIndirectArgs;
use crate::engine::back::types::globals::Globals;
use crate::engine::back::types::light::LightType;
use crate::engine::back::types::object::Object;
use crate::engine::back::types::vertex::Vertex;
use crate::engine::front::components::camera::Camera;
use crate::engine::front::components::light::Light;

pub struct Buffers {
    pub command_buffer: Buffer,
    pub count_buffer: Buffer,
    pub vertices_buffer: Buffer,
    pub indices_buffer: Buffer,

    pub globals: Buffer,
    pub camera_buffer: Buffer,

    pub object_buffer: Buffer,
    pub object_count_buffer: Buffer,
    
    pub light_buffer: Buffer,
    pub light_count_buffer: Buffer,
    pub count_light_buffer: Buffer,
    pub light_ids_buffer: Buffer,

    pub bone_buffer: Buffer,
    pub bone_i_buffer: Buffer,

    pub object_ids_buffer: Buffer,
    pub bilinear_mip_sampler: Sampler,
}

impl Buffers {
    #[allow(unused_variables)]
    pub fn new(device: &Device, info: &AdapterInfo, queue: Arc<Queue>, color_texture_view: TextureView, color_texture_size: Extent3d)-> (Self, StagingBuffers) {
        //TODO: make buffers dynamic, until then i will limit
        let max_objects = 500_000;
        let max_diff_objects = 500_000;
        let max_diff_object_ids = 500_000;

        let max_vertices = 1_000_000;
        let max_indices = 10_000_000;

        let max_lights = 10_000;

        let max_bones = 1;

        let mut vram_usage = 16 + size_of::<Globals>();
        vram_usage += max_objects * size_of::<Object>() * 2;
        vram_usage += max_diff_objects * size_of::<DrawIndexedIndirectArgs>();
        vram_usage += max_diff_object_ids * 4;

        vram_usage += max_vertices * size_of::<Vertex>() * 2;
        vram_usage += max_indices * 16;

        vram_usage += max_lights * (size_of::<Light>() + 4) * 2;

        vram_usage += size_of::<Bone>() * max_bones * 2;

        println!("buffers vram usage: {} MB / {:.2} GB", vram_usage / 1_048_576, vram_usage as f64 / 1_073_741_824.0);


        let command_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("GPU Draw commands array"),
            size: (size_of::<DrawIndexedIndirectArgs>() * max_diff_objects) as u64,
            usage: BufferUsages::INDIRECT | BufferUsages::STORAGE,
            mapped_at_creation: false,
        });

        let count_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("GPU Atomic Counter Buffer"),
            size: 4,
            usage: BufferUsages::INDIRECT | BufferUsages::STORAGE | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let object_ids_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Object ID buffer"),
            size: 4 * max_diff_object_ids as u64,
            usage: BufferUsages::STORAGE | BufferUsages::VERTEX,
            mapped_at_creation: false,
        });


        let vertices_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Vertex buffer"),
            size: (size_of::<Vertex>() * max_vertices) as BufferAddress,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let vertices_staging = device.create_buffer(&BufferDescriptor {
            label: Some("Vertex staging"),
            size: (size_of::<Vertex>() * max_vertices) as BufferAddress,
            usage: BufferUsages::COPY_SRC | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });


        let indices_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Indices buffer"),
            size: (4 * max_indices) as BufferAddress,
            usage: BufferUsages::INDEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let indices_staging = device.create_buffer(&BufferDescriptor {
            label: Some("Index staging"),
            size: (4 * max_indices) as BufferAddress,
            usage: BufferUsages::COPY_SRC | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });




        let globals_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Globals buffer"),
            size: size_of::<Globals>() as BufferAddress,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });


        let default_camera_position = [0.0, 0.0, 0.0];
        let default_camera_position4 = [
            default_camera_position[0],
            default_camera_position[1],
            default_camera_position[2],
            1.0,
        ];


        let camera = Camera::new(&default_camera_position);
        let mat = camera.build_view_proj();
        let view_proj = mat.to_cols_array_2d();
        let inv_view_proj = mat.inverse().to_cols_array_2d();

        let camera_uniform = Arc::new(ArcSwap::new(Arc::new(CameraUniform {
            view_proj,
            inv_view_proj,
            position: default_camera_position4,
            planes: [Plane {normal: [0.0; 3], d: 0.0}; 6]
        })));

        let camera_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Camera buffer"),
            contents: bytemuck::cast_slice(&[*camera_uniform.load().as_ref()]),
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
        });
        
        let camera = camera.get_structure();


        let object_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Object buffer"),
            size: (size_of::<Object>() * max_objects) as BufferAddress,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let object_staging = device.create_buffer(&BufferDescriptor {
            label: Some("Object staging"),
            size: (size_of::<Object>() * max_objects) as BufferAddress,
            usage: BufferUsages::COPY_SRC | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });


        let object_count_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Object count buffer"),
            size: 4,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
       

        //let max_lights = safe_max_bytes / size_of::<LightType>();
        
        let light_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Light buffer"),
            size: (size_of::<LightType>() * max_lights) as BufferAddress,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        
        let light_staging = device.create_buffer(&BufferDescriptor {
            label: Some("Light staging"),
            size: (size_of::<LightType>() * max_lights) as BufferAddress,
            usage: BufferUsages::COPY_SRC | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let count_light_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("GPU Atomic Counter Buffer"),
            size: 4,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let light_ids_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Object ID buffer"),
            size: (max_lights * 4) as u64,
            usage: BufferUsages::STORAGE | BufferUsages::VERTEX,
            mapped_at_creation: false,
        });

        let light_count_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("light count buffer"),
            size: 4,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });



        //let max_bones = safe_max_bytes / size_of::<Bone>();
        let max_bones = 1;

        let bone_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Bone buffer"),
            size: (size_of::<Bone>() * max_bones) as BufferAddress,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bone_staging = device.create_buffer(&BufferDescriptor {
            label: Some("Bone staging"),
            size: (size_of::<Bone>() * max_bones) as BufferAddress,
            usage: BufferUsages::COPY_SRC | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bone_i_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Bone indices buffer"),
            size: (4 * max_indices) as BufferAddress,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bone_i_staging = device.create_buffer(&BufferDescriptor {
            label: Some("Bone indices staging"),
            size: (4 * max_indices) as BufferAddress,
            usage: BufferUsages::COPY_SRC | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bilinear_mip_sampler = device.create_sampler(&SamplerDescriptor {
            label: Some("light rays blur sampler"),
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Linear,
            mipmap_filter: MipmapFilterMode::Linear,

            ..Default::default()
        });


        (
            Self {
                command_buffer,
                count_buffer,
                vertices_buffer,
                indices_buffer,

                globals: globals_buffer,
                camera_buffer,
                object_buffer,
                object_count_buffer,

                light_buffer,
                light_count_buffer,
                count_light_buffer,
                light_ids_buffer,

                bone_buffer,
                bone_i_buffer,
                
                object_ids_buffer,
                bilinear_mip_sampler,
            },

            StagingBuffers {
                queue: queue.clone(),
                camera_structure: camera,
                
                vertices_staging,
                indices_staging,
                object_staging,
                light_staging,
                bone_staging,
                bone_i_staging,
                
                vertices_count: 0,
                indices_count: 0,
                object_count: 0,
                light_count: 0,
                bone_count: 0,
                bone_i_count: 0,
                
                scene_light: 1.0,
                old_scene_light: 1.0,
                
                vertices_dest_offset: 0,
                indices_dest_offset: 0,
                object_dest_offset: 0,
                light_dest_offset: 0,

                color_texture_view,
                color_texture_size,
            }
        )
    }
}