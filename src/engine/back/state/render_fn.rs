use wgpu::{BindGroupDescriptor, BindGroupEntry, BindingResource, RenderPassDescriptor, StoreOp};
use crate::engine::back::state::State;

impl<'a> State<'a> {
    pub fn deferred_rendering(&mut self, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView, object_count: &u32) {
        { //vertex pass
            let mut render_pass_3d = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("deferred vertex render pass 3D"),
                color_attachments: &[
                    Some(wgpu::RenderPassColorAttachment {
                        view: &self.screen_textures.data_view,
                        depth_slice: None,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                            store: StoreOp::Store,
                        },
                    }),
                    Some(wgpu::RenderPassColorAttachment {
                        view: &self.screen_textures.color_view,
                        depth_slice: None,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                            store: StoreOp::Store,
                        },
                    }),
                    Some(wgpu::RenderPassColorAttachment {
                        view: &&self.screen_textures.uv_view,
                        depth_slice: None,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                            store: StoreOp::Store,
                        },
                    })
                ],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.screen_textures.depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass_3d.set_bind_group(0, &self.pipelines.deferred_v.bind_group, &[]);
            render_pass_3d.set_pipeline(&self.pipelines.deferred_v.pipeline);

            render_pass_3d.set_vertex_buffer(0, self.buffers.vertices_buffer.slice(..));
            render_pass_3d.set_vertex_buffer(1, self.buffers.object_ids_buffer.slice(..));

            render_pass_3d.set_index_buffer(
                self.buffers.indices_buffer.slice(..),
                wgpu::IndexFormat::Uint32,
            );

            render_pass_3d.multi_draw_indexed_indirect_count(
                &self.buffers.command_buffer,
                0,
                &self.buffers.count_buffer,
                0,
                *object_count,
            );
        }

        { // color pass
            let mut render_pass_3d = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("deferred fragment render pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
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

            render_pass_3d.set_bind_group(0, &self.pipelines.deferred_f.bind_group, &[]);
            render_pass_3d.set_bind_group(1, &self.pipelines.deferred_f.screen_bind_group, &[]);
            render_pass_3d.set_pipeline(&self.pipelines.deferred_f.pipeline);

            render_pass_3d.set_vertex_buffer(0, self.black_vertices_buffer.slice(..));

            render_pass_3d.set_index_buffer(
                self.black_indices_buffer.slice(..),
                wgpu::IndexFormat::Uint32,
            );

            render_pass_3d.draw_indexed(0..6, 0, 0..1);
        }
    }

    pub fn forward_rendering(&mut self, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView, object_count: &u32) {
        { // per pixel forward pass
            let mut render_pass_3d = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("per pixel forward render pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
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

            render_pass_3d.set_bind_group(0, &self.pipelines.per_pixel_forward_pass.bind_group, &[]);
            render_pass_3d.set_bind_group(1, &self.pipelines.per_pixel_forward_pass.screen_bind_group, &[]);
            render_pass_3d.set_pipeline(&self.pipelines.per_pixel_forward_pass.pipeline);

            render_pass_3d.set_vertex_buffer(0, self.black_vertices_buffer.slice(..));

            render_pass_3d.set_index_buffer(
                self.black_indices_buffer.slice(..),
                wgpu::IndexFormat::Uint32,
            );

            render_pass_3d.draw_indexed(0..6, 0, 0..1);
        }

        { // render pass
            let mut render_pass_3d = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("forward render pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.screen_textures.deferred_depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass_3d.set_bind_group(0, &self.pipelines.forward_pass.bind_group, &[]);
            render_pass_3d.set_bind_group(1, &self.pipelines.forward_pass.screen_bind_group, &[]);
            render_pass_3d.set_pipeline(&self.pipelines.forward_pass.pipeline);

            render_pass_3d.set_vertex_buffer(0, self.buffers.vertices_buffer.slice(..));
            render_pass_3d.set_vertex_buffer(1, self.buffers.object_ids_buffer.slice(..));

            render_pass_3d.set_index_buffer(
                self.buffers.indices_buffer.slice(..),
                wgpu::IndexFormat::Uint32,
            );

            render_pass_3d.multi_draw_indexed_indirect_count(
                &self.buffers.command_buffer,
                0,
                &self.buffers.count_buffer,
                0,
                *object_count,
            );
        }
    }

    pub fn light_rays_rendering(&mut self, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView) {
        {
            let mut render_pass_3d = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Light rays render pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.screen_textures.light_rays_view_scratch,
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

            render_pass_3d.set_bind_group(0, &self.pipelines.light_rays_pass.bind_group1, &[]);
            render_pass_3d.set_pipeline(&self.pipelines.light_rays_pass.pipeline1);

            render_pass_3d.set_vertex_buffer(0, self.black_vertices_buffer.slice(..));

            render_pass_3d.set_index_buffer(
                self.black_indices_buffer.slice(..),
                wgpu::IndexFormat::Uint32,
            );

            render_pass_3d.draw_indexed(0..6, 0, 0..1);
        }

        {
            let mut render_pass_3d = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Light rays render pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass_3d.set_bind_group(0, &self.pipelines.light_rays_pass.bind_group2, &[]);
            render_pass_3d.set_pipeline(&self.pipelines.light_rays_pass.pipeline2);

            render_pass_3d.set_vertex_buffer(0, self.black_vertices_buffer.slice(..));

            render_pass_3d.set_index_buffer(
                self.black_indices_buffer.slice(..),
                wgpu::IndexFormat::Uint32,
            );

            render_pass_3d.draw_indexed(0..6, 0, 0..1);
        }
    }
}