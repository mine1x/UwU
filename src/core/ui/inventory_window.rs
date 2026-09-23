use crate::render::hud_ui::atlas::HudAtlas;
use crate::render::hud_ui::bars::add_textured_quad;
use crate::render::hud_ui::renderer::HudVertex;
use crate::render::hud_ui::slot_render::draw_slot_item_and_count;
use crate::engine::items::ItemType;
use super::player_preview::draw_player_in_inventory;
use super::slot_coords::get_slot_pos;

pub fn draw_hotbar_slots(
    v: &mut Vec<HudVertex>,
    i: &mut Vec<u32>,
    hotbar: &[Option<(ItemType, u32)>; 9],
    selected_slot: usize,
    aspect: f32,
) {
    let white = [1.0, 1.0, 1.0, 1.0];
    let (total_w, total_h) = (0.728 / aspect, 0.088);
    let (hx, hy) = (-total_w * 0.5, -0.96);

    // 1. Draw textured Minecraft hotbar (182x22 pixels)
    add_textured_quad(v, i, hx, hy, total_w, total_h, HudAtlas::HOTBAR, white);

    // 2. Draw item icons and counts inside slots
    let slot_w = total_w * (20.0 / 182.0);
    let (icon_pad_x, icon_pad_y) = (total_w * (3.0 / 182.0), total_h * (3.0 / 22.0));
    let (icon_w, icon_h) = (total_w * (16.0 / 182.0), total_h * (16.0 / 22.0));

    for idx in 0..9 {
        let sx = hx + (idx as f32) * slot_w;
        if let Some((item, count)) = hotbar[idx] {
            draw_slot_item_and_count(v, i, sx + icon_pad_x, hy + icon_pad_y, icon_w, icon_h, item, count, aspect);
        }
    }

    // 3. Selection highlight
    let (sel_w, sel_h) = (total_w * (24.0 / 182.0), total_h * (23.0 / 22.0));
    let sel_x = hx + (selected_slot as f32) * slot_w - total_w * (2.0 / 182.0);
    let sel_y = hy - total_h * (0.5 / 22.0);
    add_textured_quad(v, i, sel_x, sel_y, sel_w, sel_h, HudAtlas::SELECTION, white);
}

pub fn draw_inventory_window(
    v: &mut Vec<HudVertex>,
    i: &mut Vec<u32>,
    all_slots: &[Option<(ItemType, u32)>; 46],
    carried: Option<(ItemType, u32)>,
    mouse_ndc: (f32, f32),
    aspect: f32,
) {
    let white = [1.0, 1.0, 1.0, 1.0];
    let s = 0.0065f32;
    let (win_w, win_h) = (176.0 * s / aspect, 166.0 * s);
    let (win_x, win_y) = (-win_w * 0.5, -win_h * 0.5);

    add_textured_quad(v, i, win_x, win_y, win_w, win_h, HudAtlas::INVENTORY_BG, white);
    draw_player_in_inventory(v, i, win_x, win_y, mouse_ndc, aspect, s);

    let (icon_w, icon_h) = (16.0 * s / aspect, 16.0 * s);

    for idx in 0..46 {
        if let Some((gx, gy)) = get_slot_pos(idx) {
            if let Some((item, count)) = all_slots[idx] {
                let sx = win_x + (gx + 1.0) * s / aspect;
                let sy = win_y + (166.0 - gy - 18.0 + 1.0) * s;
                draw_slot_item_and_count(v, i, sx, sy, icon_w, icon_h, item, count, aspect);
            }
        }
    }

    if let Some((item, count)) = carried {
        draw_slot_item_and_count(v, i, mouse_ndc.0 - icon_w * 0.5, mouse_ndc.1 - icon_h * 0.5, icon_w, icon_h, item, count, aspect);
    }
}
