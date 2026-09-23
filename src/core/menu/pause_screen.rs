use crate::render::hud_ui::font::draw_text;
use crate::render::hud_ui::renderer::HudVertex;
use super::menu_bg::draw_dark_overlay;
use super::menu_button::MenuButton;

pub fn get_pause_buttons(aspect: f32) -> [MenuButton; 3] {
    let bw = 0.52 / aspect;
    let bh = 0.08;
    let bx = -bw * 0.5;
    [
        MenuButton::new(20, bx, 0.12, bw, bh, "Back to Game"),
        MenuButton::new(21, bx, 0.00, bw, bh, "Open to LAN"),
        MenuButton::new(22, bx, -0.12, bw, bh, "Save and Quit to Title"),
    ]
}

pub fn draw_pause_screen(
    v: &mut Vec<HudVertex>,
    i: &mut Vec<u32>,
    mouse_ndc: (f32, f32),
    aspect: f32,
) {
    draw_dark_overlay(v, i);

    let title = "Game Menu";
    let t_size = 0.046;
    let char_w = (t_size / aspect) * 0.85;
    let tx = -(title.len() as f32 * char_w) * 0.5;
    draw_text(v, i, title, tx + 0.002 / aspect, 0.35 - 0.002, t_size, [0.25, 0.25, 0.25, 1.0], aspect);
    draw_text(v, i, title, tx, 0.35, t_size, [1.0, 1.0, 1.0, 1.0], aspect);

    for btn in get_pause_buttons(aspect) {
        btn.draw(v, i, mouse_ndc, aspect);
    }
}
