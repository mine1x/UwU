use bevy::math::Vec3;
use crate::render::types::Vertex;

pub fn build_chunk_border_mesh(player_pos: Vec3) -> (Vec<Vertex>, Vec<u32>) {
    let mut v = Vec::new();
    let mut i = Vec::new();
    let (px, py, pz) = (player_pos.x.floor() as i32, player_pos.y.floor() as i32, player_pos.z.floor() as i32);
    let xs = (px.div_euclid(16) * 16) as f32;
    let zs = (pz.div_euclid(16) * 16) as f32;
    let (ymin, ymax) = (-16.0f32, 48.0f32);
    let sec_y = (py.div_euclid(16) * 16) as f32;

    // 1. Neighbor outer chunk vertical corners (-16, 0, 16, 32)
    for dx in [-16.0, 32.0] {
        for dz in [-16.0, 0.0, 16.0, 32.0] {
            add_box_line(&mut v, &mut i, Vec3::new(xs + dx, ymin, zs + dz), Vec3::new(xs + dx, ymax, zs + dz), 0.02, -5.0);
        }
    }
    for dz in [-16.0, 32.0] {
        for dx in [0.0, 16.0] {
            add_box_line(&mut v, &mut i, Vec3::new(xs + dx, ymin, zs + dz), Vec3::new(xs + dx, ymax, zs + dz), 0.02, -5.0);
        }
    }

    // 2. Sub-chunk vertical lines every 2 blocks
    for x_off in (0..=16).step_by(2) {
        for z_off in (0..=16).step_by(2) {
            if x_off == 0 || x_off == 16 || z_off == 0 || z_off == 16 {
                let is_corner = (x_off == 0 || x_off == 16) && (z_off == 0 || z_off == 16);
                let (thickness, layer) = if is_corner { (0.04, -3.0) } else { (0.015, -4.0) };
                add_box_line(&mut v, &mut i, Vec3::new(xs + x_off as f32, sec_y, zs + z_off as f32), Vec3::new(xs + x_off as f32, sec_y + 16.0, zs + z_off as f32), thickness, layer);
            }
        }
    }

    // 3. Sub-chunk horizontal lines every 2 blocks
    for y_off in (0..=16).step_by(2) {
        let is_sec_edge = y_off == 0 || y_off == 16;
        let (thick, layer) = if is_sec_edge { (0.04, -3.0) } else { (0.015, -4.0) };
        add_box_line(&mut v, &mut i, Vec3::new(xs, sec_y + y_off as f32, zs), Vec3::new(xs + 16.0, sec_y + y_off as f32, zs), thick, layer);
        add_box_line(&mut v, &mut i, Vec3::new(xs, sec_y + y_off as f32, zs + 16.0), Vec3::new(xs + 16.0, sec_y + y_off as f32, zs + 16.0), thick, layer);
        add_box_line(&mut v, &mut i, Vec3::new(xs, sec_y + y_off as f32, zs), Vec3::new(xs, sec_y + y_off as f32, zs + 16.0), thick, layer);
        add_box_line(&mut v, &mut i, Vec3::new(xs + 16.0, sec_y + y_off as f32, zs), Vec3::new(xs + 16.0, sec_y + y_off as f32, zs + 16.0), thick, layer);
    }

    // 4. Current chunk outer 4 yellow vertical pillars (-3.0)
    for dx in [0.0, 16.0] {
        for dz in [0.0, 16.0] {
            add_box_line(&mut v, &mut i, Vec3::new(xs + dx, ymin, zs + dz), Vec3::new(xs + dx, ymax, zs + dz), 0.035, -3.0);
        }
    }

    // 5. Red full-world chunk corner borders (-2.0)
    add_box_line(&mut v, &mut i, Vec3::new(xs, ymin, zs), Vec3::new(xs, ymax, zs), 0.045, -2.0);
    add_box_line(&mut v, &mut i, Vec3::new(xs + 16.0, ymin, zs), Vec3::new(xs + 16.0, ymax, zs), 0.045, -2.0);
    add_box_line(&mut v, &mut i, Vec3::new(xs, ymin, zs + 16.0), Vec3::new(xs, ymax, zs + 16.0), 0.045, -2.0);
    add_box_line(&mut v, &mut i, Vec3::new(xs + 16.0, ymin, zs + 16.0), Vec3::new(xs + 16.0, ymax, zs + 16.0), 0.045, -2.0);

    (v, i)
}

fn add_box_line(v: &mut Vec<Vertex>, i: &mut Vec<u32>, p0: Vec3, p1: Vec3, r: f32, layer: f32) {
    let min = Vec3::new(p0.x.min(p1.x) - r, p0.y.min(p1.y) - r, p0.z.min(p1.z) - r);
    let max = Vec3::new(p0.x.max(p1.x) + r, p0.y.max(p1.y) + r, p0.z.max(p1.z) + r);
    let s = v.len() as u32;
    let corners = [
        Vec3::new(min.x, min.y, min.z), Vec3::new(max.x, min.y, min.z),
        Vec3::new(max.x, max.y, min.z), Vec3::new(min.x, max.y, min.z),
        Vec3::new(min.x, min.y, max.z), Vec3::new(max.x, min.y, max.z),
        Vec3::new(max.x, max.y, max.z), Vec3::new(min.x, max.y, max.z),
    ];
    for c in corners {
        v.push(Vertex { position: c.to_array(), normal: [0.0, 1.0, 0.0], uv: [0.0, 0.0], tex_layer: layer });
    }
    let idx_list = [
        0, 1, 2, 0, 2, 3, 4, 6, 5, 4, 7, 6, 0, 4, 5, 0, 5, 1,
        2, 6, 7, 2, 7, 3, 0, 3, 7, 0, 7, 4, 1, 5, 6, 1, 6, 2,
    ];
    for &idx in &idx_list { i.push(s + idx); }
}