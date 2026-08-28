use wgpu::{Device, Extent3d, FilterMode, Sampler, TextureFormat, TextureUsages, TextureView};

pub struct GlobalTextures {
    color_texture_size: Extent3d,
    color_texture_view: TextureView,
    
    color_nearest_sampler: Sampler,
    color_linear_sampler: Sampler,
}

impl GlobalTextures {
    pub fn new(device: &Device) -> Self {
        let max_size = device.limits().max_texture_dimension_2d;
        println!("Max texture size: {}", max_size);

        let color_texture_size = Extent3d {
            width: max_size,
            height: max_size,
            depth_or_array_layers: 1,
        };

        let color_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Color Texture"),
            size: color_texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            usage: TextureUsages::TEXTURE_BINDING| TextureUsages::COPY_DST,
            view_formats: &[],
        });

        let color_nearest_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("sampler"),
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            address_mode_w: wgpu::AddressMode::Repeat,
            mag_filter: FilterMode::Nearest,
            min_filter: FilterMode::Nearest,
            mipmap_filter: FilterMode::Nearest,
            ..Default::default()
        });

        let color_linear_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("sampler"),
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            address_mode_w: wgpu::AddressMode::Repeat,
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Linear,
            mipmap_filter: FilterMode::Linear,
            ..Default::default()
        });
        
        let color_texture_view = color_texture.create_view(&wgpu::TextureViewDescriptor::default());

        Self {
            color_texture_size,
            color_texture_view,
            
            color_nearest_sampler,
            color_linear_sampler
        }
    }
}