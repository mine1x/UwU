use super::atlas::HudAtlas;
use super::bars::add_textured_quad;
use super::renderer::HudVertex;

pub fn draw_text(
    v: &mut Vec<HudVertex>,
    i: &mut Vec<u32>,
    text: &str,
    x: f32,
    y: f32,
    size: f32,
    col: [f32; 4],
    aspect: f32,
) {
    let char_w = size / aspect;
    let char_h = size;
    let mut cur_x = x;
    for b in text.bytes() {
        let rect = HudAtlas::ascii_char(b);
        add_textured_quad(v, i, cur_x, y, char_w, char_h, rect, col);
        cur_x += char_w * 0.85;
    }
}