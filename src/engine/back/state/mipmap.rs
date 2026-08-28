use wgpu::{BindGroupDescriptor, BindGroupEntry, BindingResource, Device, RenderPassDescriptor, StoreOp, Texture};
use crate::engine::back::state::State;

impl<'a> State<'a> {
    pub fn mipmap_rendering(&mut self, encoder: &mut wgpu::CommandEncoder, mipmap_count: u32, texture_ptr: *const Texture) {
        let texture;
        unsafe {
            texture = &*texture_ptr;
        }
        
        for i in 1..mipmap_count {
            let prev_texture_view = texture.create_view(&wgpu::TextureViewDescriptor {
                base_mip_level: i - 1,
                mip_level_count: Some(1),
                ..Default::default()
            });

            let texture_view = texture.create_view(&wgpu::TextureViewDescriptor {
                base_mip_level: i,
                mip_level_count: Some(1),
                ..Default::default()
            });

            let bind_group = self.device.create_bind_group(&BindGroupDescriptor {
                layout: &self.pipelines.mipmap.bind_group_layout,
                entries: &[
                    BindGroupEntry {
                        binding: 0,
                        resource: BindingResource::TextureView(&prev_texture_view),
                    },
                    BindGroupEntry {
                        binding: 1,
                        resource: BindingResource::Sampler(&self.buffers.bilinear_mip_sampler),
                    }
                ],
                label: Some("Mipmap screen bind group"),
            });

            let mut render_pass_3d = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Mipmap render pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &texture_view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass_3d.set_bind_group(0, &bind_group, &[]);
            render_pass_3d.set_pipeline(&self.pipelines.mipmap.pipeline);

            render_pass_3d.set_vertex_buffer(0, self.black_vertices_buffer.slice(..));

            render_pass_3d.set_index_buffer(
                self.black_indices_buffer.slice(..),
                wgpu::IndexFormat::Uint32,
            );

            render_pass_3d.draw_indexed(0..6, 0, 0..1);
        }
    }
}