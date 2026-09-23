use crate::render::hud_ui::atlas::HudAtlas;
use crate::render::hud_ui::bars::add_textured_quad;
use crate::render::hud_ui::font::draw_text;
use crate::render::hud_ui::renderer::HudVertex;

#[derive(Clone, Copy, Debug)]
pub struct MenuButton {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub text: &'static str,
}

impl MenuButton {
    pub const fn new(id: u32, x: f32, y: f32, w: f32, h: f32, text: &'static str) -> Self {
        Self { id, x, y, w, h, text }
    }

    pub fn is_hovered(&self, m: (f32, f32)) -> bool {
        m.0 >= self.x && m.0 <= self.x + self.w && m.1 >= self.y && m.1 <= self.y + self.h
    }

    pub fn draw(&self, v: &mut Vec<HudVertex>, i: &mut Vec<u32>, m: (f32, f32), aspect: f32) {
        let hovered = self.is_hovered(m);
        let sprite = if hovered { HudAtlas::BUTTON_HL } else { HudAtlas::BUTTON };
        let white = [1.0, 1.0, 1.0, 1.0];
        add_textured_quad(v, i, self.x, self.y, self.w, self.h, sprite, white);

        // Centered button text with Minecraft drop-shadow
        let text_len = self.text.len() as f32;
        let font_size = 0.038;
        let char_w = (font_size / aspect) * 0.85;
        let total_w = text_len * char_w;
        let tx = self.x + (self.w - total_w) * 0.5;
        let ty = self.y + (self.h - font_size) * 0.5;

        // Shadow: #3f3f3f
        let shadow_col = [0.24, 0.24, 0.24, 1.0];
        let offset = 0.003;
        draw_text(v, i, self.text, tx + offset / aspect, ty - offset, font_size, shadow_col, aspect);

        // Text: #ffffa0 when hovered, #e0e0e0 when normal
        let text_col = if hovered { [1.0, 1.0, 0.62, 1.0] } else { [0.88, 0.88, 0.88, 1.0] };
        draw_text(v, i, self.text, tx, ty, font_size, text_col, aspect);
    }
}
