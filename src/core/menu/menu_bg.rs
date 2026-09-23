use crate::render::hud_ui::atlas::HudAtlas;
use crate::render::hud_ui::bars::{add_hud_quad, add_textured_quad};
use crate::render::hud_ui::renderer::HudVertex;

pub fn draw_tiled_menu_background(v: &mut Vec<HudVertex>, i: &mut Vec<u32>, aspect: f32) {
    let tile_size = 0.08;
    let tile_w = tile_size / aspect;
    let tile_h = tile_size;
    let col = [0.25, 0.25, 0.25, 1.0];
    let steps_x = (2.0 / tile_w).ceil() as i32 + 1;
    let steps_y = (2.0 / tile_h).ceil() as i32 + 1;

    for y_idx in 0..steps_y {
        let y = -1.0 + y_idx as f32 * tile_h;
        for x_idx in 0..steps_x {
            let x = -1.0 + x_idx as f32 * tile_w;
            add_textured_quad(v, i, x, y, tile_w, tile_h, HudAtlas::MENU_BG, col);
        }
    }
}

pub fn draw_dark_overlay(v: &mut Vec<HudVertex>, i: &mut Vec<u32>) {
    add_hud_quad(v, i, -1.0, -1.0, 2.0, 2.0, [0.0, 0.0, 0.0, 0.65]);
}
