use wgpu::{Device, Extent3d, Texture, TextureFormat, TextureUsages};
use wgpu::TextureView;

pub struct ScreenTextures {
    pub light_rays_view: TextureView,
    pub light_rays_view_scratch: TextureView,

    pub deferred_depth_view: TextureView,
    pub depth_view: TextureView,
    pub data_view: TextureView,
    pub color_view: TextureView,
    pub uv_view: TextureView,
}

impl ScreenTextures {
    pub fn new(device: &Device, width: u32, height: u32) -> Self {
        let light_rays_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Light rays texture"),
            size: Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: TextureFormat::Rgba16Float,
            usage: TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });

        let light_rays_texture_scratch = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Light rays texture"),
            size: Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: TextureFormat::Rgba16Float,
            usage: TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });

        let uv_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Uv Texture"),
            size: Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: TextureFormat::Rg16Float,
            usage: TextureUsages::TEXTURE_BINDING| TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });

        let color_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Color Texture"),
            size: Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: TextureFormat::Rgba16Float,
            usage: TextureUsages::TEXTURE_BINDING| TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });

        let data_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Data Texture"),
            size: Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: TextureFormat::Rgba32Float,
            usage: TextureUsages::TEXTURE_BINDING| TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });

        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Depth Texture"),
            size: Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: TextureFormat::Depth32Float,
            usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        let deferred_depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Depth Texture"),
            size: Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: TextureFormat::Depth24Plus,
            usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        let light_rays_view = light_rays_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let light_rays_view_scratch = light_rays_texture_scratch.create_view(&wgpu::TextureViewDescriptor::default());
        let uv_view = uv_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let color_view = color_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let data_view = data_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let deferred_depth_view = deferred_depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

        Self {
            light_rays_view,
            light_rays_view_scratch,

            deferred_depth_view,
            depth_view,
            data_view,
            color_view,
            uv_view,
        }
    }
}