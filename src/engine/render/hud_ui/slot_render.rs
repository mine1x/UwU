use super::bars::add_textured_quad;
use super::font::draw_text;
use super::item_icon::get_item_sprite;
use super::renderer::HudVertex;
use crate::engine::items::ItemType;

pub fn draw_slot_item_and_count(
    v: &mut Vec<HudVertex>,
    i: &mut Vec<u32>,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    item: ItemType,
    count: u32,
    aspect: f32,
) {
    let white = [1.0, 1.0, 1.0, 1.0];
    let sprite = get_item_sprite(&item);
    add_textured_quad(v, i, x, y, w, h, sprite, white);

    if count > 1 {
        let text = format!("{}", count);
        let font_size = h * 0.42;
        let char_w = font_size / aspect * 0.85;
        let text_w = char_w * text.len() as f32;
        let tx = x + w - text_w - 0.001 / aspect;
        let ty = y + 0.002;
        draw_text(v, i, &text, tx + 0.0015 / aspect, ty - 0.0015, font_size, [0.24, 0.24, 0.24, 1.0], aspect);
        draw_text(v, i, &text, tx, ty, font_size, white, aspect);
    }
}
