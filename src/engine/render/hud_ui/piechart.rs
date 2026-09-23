use std::f32::consts::PI;
use super::bars::add_hud_quad;
use super::font::draw_text;
use super::renderer::HudVertex;
use crate::engine::ResultField;

fn add_hud_tri(v: &mut Vec<HudVertex>, i: &mut Vec<u32>, p0: [f32; 2], p1: [f32; 2], p2: [f32; 2], c: [f32; 4]) {
    let s = v.len() as u32;
    v.push(HudVertex { position: p0, uv: [-1.0, -1.0], color: c });
    v.push(HudVertex { position: p1, uv: [-1.0, -1.0], color: c });
    v.push(HudVertex { position: p2, uv: [-1.0, -1.0], color: c });
    i.extend_from_slice(&[s, s + 1, s + 2]);
}

pub fn draw_profiler_piechart(v: &mut Vec<HudVertex>, i: &mut Vec<u32>, fields: &[ResultField], path: &str, aspect: f32) {
    if fields.is_empty() { return; }
    let (cx, cy) = (0.55f32, 0.45f32);
    let (rx, ry) = (0.28 / aspect, 0.14);
    let (pw, ph) = (0.72 / aspect, 1.05);
    let (left, top) = (cx - pw * 0.5, 0.88f32);

    add_hud_quad(v, i, left, top - ph, pw, ph, [0.03, 0.04, 0.06, 0.85]);

    let title = format!("[0] {}", path);
    draw_text(v, i, &title, left + 0.02, top - 0.05, 0.032, [1.0, 1.0, 1.0, 1.0], aspect);
    let gp_text = format!("{:.1}%", fields[0].global_percentage);
    draw_text(v, i, &gp_text, left + pw - 0.12 / aspect, top - 0.05, 0.032, [1.0, 0.85, 0.2, 1.0], aspect);

    let mut accum = 0.0f32;
    for field in fields.iter().skip(1) {
        let slice = field.percentage as f32;
        let col = field.get_color();
        let shade = [col[0] * 0.5, col[1] * 0.5, col[2] * 0.5, 1.0];
        let steps = ((slice / 4.0).floor() as usize + 1).max(2);

        for j in (1..=steps).rev() {
            let d0 = (accum + slice * (j as f32) / steps as f32) * (2.0 * PI) / 100.0;
            let d1 = (accum + slice * ((j - 1) as f32) / steps as f32) * (2.0 * PI) / 100.0;
            let p0 = [cx + d0.sin() * rx, cy + d0.cos() * ry];
            let p1 = [cx + d1.sin() * rx, cy + d1.cos() * ry];

            add_hud_tri(v, i, [cx, cy], p0, p1, col);

            if (p0[1] + p1[1]) * 0.5 <= cy {
                let p0_bot = [p0[0], p0[1] - 0.03];
                let p1_bot = [p1[0], p1[1] - 0.03];
                add_hud_tri(v, i, p0, p0_bot, p1_bot, shade);
                add_hud_tri(v, i, p0, p1_bot, p1, shade);
            }
        }
        accum += slice;
    }

    let list_top = cy - ry - 0.06;
    for (idx, field) in fields.iter().skip(1).take(8).enumerate() {
        let y = list_top - (idx as f32) * 0.040;
        let col = field.get_color();
        add_hud_quad(v, i, left + 0.02, y, 0.016 / aspect, 0.024, col);
        let line = format!("[{}] {}: {:.1}%", idx + 1, field.name, field.percentage);
        draw_text(v, i, &line, left + 0.02 + 0.025 / aspect, y, 0.025, col, aspect);
    }
}