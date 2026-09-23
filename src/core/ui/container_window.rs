use crate::render::hud_ui::atlas::HudAtlas;
use crate::render::hud_ui::bars::{add_hud_quad, add_textured_quad};
use crate::render::hud_ui::renderer::HudVertex;
use crate::render::hud_ui::slot_render::draw_slot_item_and_count;
use crate::engine::container::{ContainerSnapshot, SlotTarget, container_slot_positions};
use crate::engine::items::ItemType;
use super::slot_coords::get_slot_pos;

pub fn draw_container_window(
    v: &mut Vec<HudVertex>,
    i: &mut Vec<u32>,
    snapshot: &ContainerSnapshot,
    player_all_slots: &[Option<(ItemType, u32)>; 46],
    carried: Option<(ItemType, u32)>,
    mouse_ndc: (f32, f32),
    aspect: f32,
) {
    let white = [1.0, 1.0, 1.0, 1.0];
    let s = 0.0065f32;
    let (win_w, win_h) = (176.0 * s / aspect, 166.0 * s);
    let (win_x, win_y) = (-win_w * 0.5, -win_h * 0.5);

    // Background (reuse inventory bg; its top area will be masked)
    add_textured_quad(v, i, win_x, win_y, win_w, win_h, HudAtlas::INVENTORY_BG, white);

    // Mask the top region with a dark quad (covers armor/craft area from INVENTORY_BG)
    let mask_h = 84.0 * s;
    add_hud_quad(v, i, win_x, win_y + win_h - mask_h, win_w, mask_h, [0.12, 0.12, 0.13, 0.95]);

    let (icon_w, icon_h) = (16.0 * s / aspect, 16.0 * s);

    // Draw backend slots
    for (gx, gy, target) in container_slot_positions(snapshot.kind) {
        if let SlotTarget::Backend(b) = target {
            // Slot background
            let sx = win_x + (gx + 1.0) * s / aspect;
            let sy = win_y + (166.0 - gy - 18.0 + 1.0) * s;
            add_textured_quad(v, i, sx, sy, icon_w, icon_h, HudAtlas::SLOT, white);
            // Item icon + count
            if let Some((item, count)) = snapshot.items.get(b).and_then(|s| *s) {
                draw_slot_item_and_count(v, i, sx, sy, icon_w, icon_h, item, count, aspect);
            }
        }
    }

    // Draw player storage slots (0..27) + hotbar (27..36)
    for idx in 0..36 {
        if let Some((item, count)) = player_all_slots[idx] {
            if let Some((gx, gy)) = get_slot_pos(idx) {
                let sx = win_x + (gx + 1.0) * s / aspect;
                let sy = win_y + (166.0 - gy - 18.0 + 1.0) * s;
                draw_slot_item_and_count(v, i, sx, sy, icon_w, icon_h, item, count, aspect);
            }
        }
    }

    // Furnace progress bars
    if snapshot.kind == crate::engine::container::ContainerKind::Furnace {
        let bar_x = win_x + 72.0 * s / aspect;
        let bar_w = 14.0 * s / aspect;
        let bar_h = 14.0 * s;
        let bar_y = win_y + (166.0 - 36.0 - 14.0) * s;
        add_hud_quad(v, i, bar_x, bar_y, bar_w, bar_h, [0.2, 0.2, 0.2, 1.0]);
        if snapshot.prog_a > 0.0 {
            let fh = bar_h * snapshot.prog_a.clamp(0.0, 1.0);
            add_hud_quad(v, i, bar_x, bar_y, bar_w, fh, [0.95, 0.45, 0.1, 1.0]);
        }

        let arr_x = win_x + 79.0 * s / aspect;
        let arr_y = win_y + (166.0 - 35.0 - 16.0) * s;
        let arr_w = 24.0 * s / aspect;
        let arr_h = 16.0 * s;
        add_hud_quad(v, i, arr_x, arr_y, arr_w, arr_h, [0.2, 0.2, 0.2, 1.0]);
        if snapshot.prog_b > 0.0 {
            let fw = arr_w * snapshot.prog_b.clamp(0.0, 1.0);
            add_hud_quad(v, i, arr_x, arr_y, fw, arr_h, [0.95, 0.95, 0.95, 1.0]);
        }
    }

    if let Some((item, count)) = carried {
        draw_slot_item_and_count(v, i, mouse_ndc.0 - icon_w * 0.5, mouse_ndc.1 - icon_h * 0.5, icon_w, icon_h, item, count, aspect);
    }
}
