use super::renderer::HudVertex;
use super::atlas::{HudAtlas, SpriteRect};

pub fn add_textured_quad(
    v: &mut Vec<HudVertex>,
    i: &mut Vec<u32>,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    rect: SpriteRect,
    col: [f32; 4],
) {
    let s = v.len() as u32;
    v.push(HudVertex { position: [x, y], uv: [rect.u0, rect.v1], color: col });
    v.push(HudVertex { position: [x + w, y], uv: [rect.u1, rect.v1], color: col });
    v.push(HudVertex { position: [x + w, y + h], uv: [rect.u1, rect.v0], color: col });
    v.push(HudVertex { position: [x, y + h], uv: [rect.u0, rect.v0], color: col });
    i.extend_from_slice(&[s, s + 1, s + 2, s, s + 2, s + 3]);
}

pub fn add_hud_quad(v: &mut Vec<HudVertex>, i: &mut Vec<u32>, x: f32, y: f32, w: f32, h: f32, col: [f32; 4]) {
    let s = v.len() as u32;
    v.push(HudVertex { position: [x, y], uv: [-1.0, -1.0], color: col });
    v.push(HudVertex { position: [x + w, y], uv: [-1.0, -1.0], color: col });
    v.push(HudVertex { position: [x + w, y + h], uv: [-1.0, -1.0], color: col });
    v.push(HudVertex { position: [x, y + h], uv: [-1.0, -1.0], color: col });
    i.extend_from_slice(&[s, s + 1, s + 2, s, s + 2, s + 3]);
}

pub fn draw_bars_and_crosshair(v: &mut Vec<HudVertex>, i: &mut Vec<u32>, health: f32, stamina: f32, hunger: f32, aspect: f32) {
    let white = [1.0, 1.0, 1.0, 1.0];

    // 1. Crosshair center (15x15 sprite)
    let (cw, ch) = (0.026 / aspect, 0.026);
    add_textured_quad(v, i, -cw * 0.5, -ch * 0.5, cw, ch, HudAtlas::CROSSHAIR, white);

    // 2. XP Bar (182x5 sprite)
    let (xw, xh) = (0.728 / aspect, 0.020);
    let (xx, xy) = (-xw * 0.5, -0.83);
    add_textured_quad(v, i, xx, xy, xw, xh, HudAtlas::XP_BG, white);
    let frac = (stamina / 100.0).clamp(0.0, 1.0);
    if frac > 0.005 {
        let mut fg_rect = HudAtlas::XP_FG;
        fg_rect.u1 = fg_rect.u0 + (fg_rect.u1 - fg_rect.u0) * frac;
        add_textured_quad(v, i, xx, xy, xw * frac, xh, fg_rect, white);
    }

    // 3. Minecraft 10-heart bar and 10-food hunger bar (hidden in Creative)
    if health >= 0.0 {
        let (hw, hh, sp) = (0.028 / aspect, 0.028, 0.003 / aspect);
        let h_start_x = -xw * 0.5;
        let h_start_y = -0.79;
        let half_hearts = if health <= 20.0 { (health.round() as i32).clamp(0, 20) } else { ((health / 5.0).round() as i32).clamp(0, 20) };

        for idx in 0..10 {
            let x = h_start_x + (idx as f32) * (hw + sp);
            add_textured_quad(v, i, x, h_start_y, hw, hh, HudAtlas::HEART_BG, white);
            let heart_val = half_hearts - idx * 2;
            if heart_val >= 2 {
                add_textured_quad(v, i, x, h_start_y, hw, hh, HudAtlas::HEART_FULL, white);
            } else if heart_val == 1 {
                add_textured_quad(v, i, x, h_start_y, hw, hh, HudAtlas::HEART_HALF, white);
            }
        }

        let food_count = if hunger <= 20.0 { ((hunger / 2.0).ceil() as i32).clamp(0, 10) } else { ((hunger / 10.0).ceil() as i32).clamp(0, 10) };
        let f_start_x = xw * 0.5 - hw;
        for idx in 0..10 {
            let x = f_start_x - (idx as f32) * (hw + sp);
            add_textured_quad(v, i, x, h_start_y, hw, hh, HudAtlas::FOOD_BG, white);
            if idx < food_count {
                add_textured_quad(v, i, x, h_start_y, hw, hh, HudAtlas::FOOD_FULL, white);
            }
        }
    }
}