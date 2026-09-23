use crate::render::hud_ui::atlas::HudAtlas;
use crate::render::hud_ui::bars::add_textured_quad;
use crate::render::hud_ui::renderer::HudVertex;
use crate::render::hud_ui::slot_render::draw_slot_item_and_count;
use crate::engine::creative_palette::get_creative_palette;
use crate::engine::items::ItemType;

pub fn draw_creative_inventory_window(
    v: &mut Vec<HudVertex>,
    i: &mut Vec<u32>,
    hotbar: &[Option<(ItemType, u32)>; 9],
    carried: Option<(ItemType, u32)>,
    mouse_ndc: (f32, f32),
    aspect: f32,
) {
    let white = [1.0, 1.0, 1.0, 1.0];
    let s = 0.0065f32;
    let (win_w, win_h) = (195.0 * s / aspect, 136.0 * s);
    let (win_x, win_y) = (-win_w * 0.5, -win_h * 0.5);

    // 1. Creative GUI window background
    add_textured_quad(v, i, win_x, win_y, win_w, win_h, HudAtlas::CREATIVE_INVENTORY_BG, white);

    let (icon_w, icon_h) = (16.0 * s / aspect, 16.0 * s);
    let palette = get_creative_palette();

    // 2. 9x5 Creative item palette (45 slots)
    for row in 0..5 {
        for col in 0..9 {
            let idx = row * 9 + col;
            if idx < palette.len() {
                let item = palette[idx];
                let gx = 9.0 + (col as f32) * 18.0;
                let gy = 18.0 + (row as f32) * 18.0;
                let sx = win_x + (gx + 1.0) * s / aspect;
                let sy = win_y + (136.0 - gy - 18.0 + 1.0) * s;
                let count = if item.is_tool() { 1 } else { 64 };
                draw_slot_item_and_count(v, i, sx, sy, icon_w, icon_h, item, count, aspect);
            }
        }
    }

    // 3. Hotbar row (9 slots at bottom: y=112)
    for col in 0..9 {
        if let Some((item, count)) = hotbar[col] {
            let gx = 9.0 + (col as f32) * 18.0;
            let gy = 112.0;
            let sx = win_x + (gx + 1.0) * s / aspect;
            let sy = win_y + (136.0 - gy - 18.0 + 1.0) * s;
            draw_slot_item_and_count(v, i, sx, sy, icon_w, icon_h, item, count, aspect);
        }
    }

    // 4. Carried item on cursor
    if let Some((item, count)) = carried {
        draw_slot_item_and_count(v, i, mouse_ndc.0 - icon_w * 0.5, mouse_ndc.1 - icon_h * 0.5, icon_w, icon_h, item, count, aspect);
    }
}
