use bevy::math::Vec3;
use super::preview_boxes::get_steve_preview_boxes;
use crate::render::hud_ui::renderer::HudVertex;

pub fn draw_player_in_inventory(
    v: &mut Vec<HudVertex>,
    idx: &mut Vec<u32>,
    win_x: f32,
    win_y: f32,
    mouse_ndc: (f32, f32),
    aspect: f32,
    s: f32,
) {
    let mg_x = (mouse_ndc.0 - win_x) * aspect / s;
    let mg_y = 166.0 - (mouse_ndc.1 - win_y) / s;
    let (dx, dy) = (50.5 - mg_x, 43.0 - mg_y);
    let (x_angle, y_angle) = ((dx / 40.0).atan(), (dy / 40.0).atan());
    let (byaw, hyaw) = (x_angle * 20.0f32.to_radians(), x_angle * 40.0f32.to_radians());
    let hpitch = (-y_angle * 20.0f32.to_radians()).clamp(-0.7, 0.7);

    let (cby, sby) = (byaw.cos(), byaw.sin());
    let (chy, shy) = (hyaw.cos(), hyaw.sin());
    let (chp, shp) = (hpitch.cos(), hpitch.sin());

    let rot_b = |p: Vec3| Vec3::new(p.x * cby + p.z * sby, p.y, -p.x * sby + p.z * cby);
    let rot_h = |p: Vec3| {
        let y = Vec3::new(p.x * chy + p.z * shy, p.y, -p.x * shy + p.z * chy);
        Vec3::new(y.x, y.y * chp - y.z * shp, y.y * shp + y.z * chp)
    };

    let feet_x = win_x + 50.5 * s / aspect;
    let feet_y = win_y + 91.0 * s;
    let ms = 1.6875 * s;

    for (b_i, b) in get_steve_preview_boxes().iter().enumerate() {
        let is_head = b_i == 5;
        let (hx, hy, hz) = (b.size.x * 0.5, b.size.y * 0.5, b.size.z * 0.5);
        let faces: [(Vec3, [Vec3; 4], f32, usize); 6] = [
            (Vec3::Y, [Vec3::new(-hx, hy, -hz), Vec3::new(hx, hy, -hz), Vec3::new(hx, hy, hz), Vec3::new(-hx, hy, hz)], 1.0, 0),
            (-Vec3::Y, [Vec3::new(-hx, -hy, hz), Vec3::new(hx, -hy, hz), Vec3::new(hx, -hy, -hz), Vec3::new(-hx, -hy, -hz)], 0.6, 1),
            (Vec3::Z, [Vec3::new(-hx, hy, hz), Vec3::new(hx, hy, hz), Vec3::new(hx, -hy, hz), Vec3::new(-hx, -hy, hz)], 0.9, 2),
            (-Vec3::Z, [Vec3::new(hx, hy, -hz), Vec3::new(-hx, hy, -hz), Vec3::new(-hx, -hy, -hz), Vec3::new(hx, -hy, -hz)], 0.7, 3),
            (Vec3::X, [Vec3::new(hx, hy, hz), Vec3::new(hx, hy, -hz), Vec3::new(hx, -hy, -hz), Vec3::new(hx, -hy, hz)], 0.8, 4),
            (-Vec3::X, [Vec3::new(-hx, hy, -hz), Vec3::new(-hx, hy, hz), Vec3::new(-hx, -hy, hz), Vec3::new(-hx, -hy, -hz)], 0.8, 5),
        ];

        for (norm, corners, light, f_idx) in faces {
            let r_norm = if is_head { rot_h(norm) } else { rot_b(norm) };
            if r_norm.z <= 0.001 { continue; }

            let uv = b.uvs[0][f_idx];
            let (u0, v0) = ((512.0 + uv[0] * 64.0) / 1024.0, (uv[1] * 64.0) / 1024.0);
            let (u1, v1) = ((512.0 + uv[2] * 64.0) / 1024.0, (uv[3] * 64.0) / 1024.0);
            let uvs = [[u0, v0], [u1, v0], [u1, v1], [u0, v1]];
            let color = [light, light, light, 1.0];
            let base = v.len() as u32;

            for (i, p) in corners.iter().enumerate() {
                let wp = if is_head {
                    rot_h(b.origin + *p - Vec3::new(0.0, 24.0, 0.0)) + Vec3::new(0.0, 24.0, 0.0)
                } else {
                    rot_b(b.origin + *p)
                };
                let sx = feet_x + wp.x * ms / aspect;
                let sy = feet_y - wp.y * ms;
                v.push(HudVertex { position: [sx, sy], uv: uvs[i], color });
            }

            idx.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
        }
    }
}
