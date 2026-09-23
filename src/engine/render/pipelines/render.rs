use crate::render::pipeline::Renderer;

const BG: wgpu::Color = wgpu::Color { r: 0.1, g: 0.12, b: 0.16, a: 1.0 };

fn draw_world_player(rp: &mut wgpu::RenderPass, r: &Renderer) {
    if r.world_index_count > 0 {
        rp.set_vertex_buffer(0, r.world_vertex_buffer.slice(..));
        rp.set_index_buffer(r.world_index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        rp.draw_indexed(0..r.world_index_count, 0, 0..1);
    }
    if r.player_index_count > 0 {
        rp.set_vertex_buffer(0, r.player_vertex_buffer.slice(..));
        rp.set_index_buffer(r.player_index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        rp.draw_indexed(0..r.player_index_count, 0, 0..1);
    }
}

pub fn execute_render_passes(r: &mut Renderer) -> Result<(), wgpu::SurfaceError> {
    let output = r.surface.get_current_texture()?;
    let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
    let mut enc = r.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("Enc") });

    {
        let mut sp = enc.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Shadow Pass"), color_attachments: &[],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &r.shadow_texture_view,
                depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(1.0), store: wgpu::StoreOp::Store }),
                stencil_ops: None,
            }),
            timestamp_writes: None, occlusion_query_set: None,
        });
        sp.set_pipeline(&r.shadow_pipeline);
        sp.set_bind_group(0, &r.camera_bind_group, &[]);
        draw_world_player(&mut sp, r);
    }

    {
        let mut mp = enc.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Main Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view, resolve_target: None,
                ops: wgpu::Operations { load: wgpu::LoadOp::Clear(BG), store: wgpu::StoreOp::Store },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &r.depth_texture,
                depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(1.0), store: wgpu::StoreOp::Store }),
                stencil_ops: None,
            }),
            timestamp_writes: None, occlusion_query_set: None,
        });
        mp.set_pipeline(&r.render_pipeline);
        mp.set_bind_group(0, &r.camera_bind_group, &[]);
        mp.set_bind_group(1, &r.shadow_bind_group, &[]);
        mp.set_bind_group(2, &r.texture_bind_group, &[]);
        draw_world_player(&mut mp, r);

        if r.overlay_index_count > 0 {
            mp.set_pipeline(&r.overlay_pipeline);
            mp.set_vertex_buffer(0, r.overlay_vertex_buffer.slice(..));
            mp.set_index_buffer(r.overlay_index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            mp.draw_indexed(0..r.overlay_index_count, 0, 0..1);
        }
    }

    {
        let mut hp = enc.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("HUD Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view, resolve_target: None,
                ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store },
            })],
            depth_stencil_attachment: None, timestamp_writes: None, occlusion_query_set: None,
        });
        r.hud_renderer.render(&mut hp);
    }

    r.queue.submit(std::iter::once(enc.finish()));
    output.present();
    Ok(())
}