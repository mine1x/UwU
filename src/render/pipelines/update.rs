use bevy::math::{Mat4, Vec3};
use wgpu::util::DeviceExt;
use crate::render::hud_ui::update_hud_mesh;
use crate::render::overlays::build_hover_overlay_mesh;
use crate::render::pipeline::Renderer;
use crate::render::types::CameraUniform;
use crate::engine::RenderSnapshot;

pub fn update_renderer_snapshot(
    r: &mut Renderer,
    snapshot: &RenderSnapshot,
    remote_players: &[crate::network::RemotePlayer],
) {
    let aspect = r.config.width as f32 / r.config.height as f32;
    let (cp, sp) = (snapshot.camera_pitch.cos(), snapshot.camera_pitch.sin());
    let (cy, sy) = (snapshot.camera_rotation.cos(), snapshot.camera_rotation.sin());
    let offset = Vec3::new(snapshot.camera_distance * cp * sy, snapshot.camera_distance * sp, snapshot.camera_distance * cp * cy);

    let eye = snapshot.player_pos + offset;
    let target = snapshot.player_pos + Vec3::new(0.0, 0.9, 0.0);
    let view = Mat4::look_at_rh(eye, target, Vec3::Y);

    let d = snapshot.camera_distance;
    let ortho_size = d * 0.6;
    let (hw, hh) = (ortho_size * aspect * 0.5, ortho_size * 0.5);
    let proj = Mat4::orthographic_rh(-hw, hw, -hh, hh, -(d * 0.1).max(0.5), (d * 50.0).max(200.0));
    let view_proj = proj * view;

    let light_dir = Vec3::new(0.5, 1.2, 0.6).normalize();
    let light_view_proj = Mat4::orthographic_rh(-32.0, 32.0, -32.0, 32.0, 1.0, 80.0) * Mat4::look_at_rh(snapshot.player_pos + light_dir * 35.0, snapshot.player_pos, Vec3::Y);

    let uniform = CameraUniform { view_proj: view_proj.to_cols_array(), light_view_proj: light_view_proj.to_cols_array(), camera_pos: [eye.x, eye.y, eye.z, 1.0], fog: [d * 1.0, d * 2.0, 0.0, 0.0] };
    r.queue.write_buffer(&r.camera_buffer, 0, bytemuck::cast_slice(&[uniform]));

    // Player buffers (local Steve + remote players)
    let (pvb, pib, count) = super::player_buffer::update_player_buffers(&r.device, &snapshot.player_mesh, remote_players);
    r.player_vertex_buffer = pvb;
    r.player_index_buffer = pib;
    r.player_index_count = count;

    // World
    if let Some((ref wv, ref wi)) = snapshot.world_mesh {
        if !wv.is_empty() {
            r.world_vertex_buffer = r.device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("WV"), contents: bytemuck::cast_slice(wv), usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST });
            r.world_index_buffer = r.device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("WI"), contents: bytemuck::cast_slice(wi), usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST });
            r.world_index_count = wi.len() as u32;
        }
    }

    // Overlay (Destroy stage + Hover block + Chunk wireframe)
    let mut ov = Vec::new();
    let mut oi = Vec::new();

    if let (Some((bx, by, bz)), Some(st)) = (snapshot.mining_target, snapshot.mining_stage) {
        let (dv, di) = crate::render::build_destroy_overlay_mesh(bx, by, bz, st, snapshot.mining_exposed_faces, offset);
        let off = ov.len() as u32;
        ov.extend_from_slice(&dv);
        for &idx in &di { oi.push(off + idx); }
    }

    if let Some((bx, by, bz)) = snapshot.hovered_block {
        let (hv, hi) = build_hover_overlay_mesh(bx, by, bz, snapshot.hovered_exposed_faces, offset);
        let off = ov.len() as u32;
        ov.extend_from_slice(&hv);
        for &idx in &hi { oi.push(off + idx); }
    }

    if snapshot.show_chunk_borders {
        let (bv, bi) = crate::render::build_chunk_border_mesh(snapshot.player_pos);
        let off = ov.len() as u32;
        ov.extend_from_slice(&bv);
        for &idx in &bi { oi.push(off + idx); }
    }

    if !oi.is_empty() {
        r.overlay_vertex_buffer = r.device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("OV"), contents: bytemuck::cast_slice(&ov), usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST });
        r.overlay_index_buffer = r.device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("OI"), contents: bytemuck::cast_slice(&oi), usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST });
        r.overlay_index_count = oi.len() as u32;
    } else {
        r.overlay_index_count = 0;
    }

    let profiler_param = snapshot.profiler_piechart.as_ref().map(|(f, p)| (f.as_slice(), p.as_str()));
    let hud_health = if snapshot.game_mode.show_hud_bars() { snapshot.health } else { -1.0 };
    update_hud_mesh(
        &mut r.hud_renderer, &r.device, hud_health, snapshot.stamina, snapshot.hunger,
        &snapshot.hotbar_items, &snapshot.all_slots, snapshot.inventory_open, snapshot.selected_slot,
        snapshot.carried_item, snapshot.mouse_ndc, profiler_param, aspect,
        snapshot.game_mode.is_creative(),
        snapshot.container.as_ref(),
    );
}