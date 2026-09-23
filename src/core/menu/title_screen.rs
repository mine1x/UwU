use crate::render::hud_ui::atlas::HudAtlas;
use crate::render::hud_ui::bars::add_textured_quad;
use crate::render::hud_ui::font::draw_text;
use crate::render::hud_ui::renderer::HudVertex;
use super::menu_bg::draw_tiled_menu_background;
use super::menu_button::MenuButton;

pub fn get_title_buttons(aspect: f32) -> [MenuButton; 3] {
    let bw = 0.52 / aspect;
    let bh = 0.08;
    let bx = -bw * 0.5;
    [
        MenuButton::new(1, bx, 0.05, bw, bh, "Singleplayer"),
        MenuButton::new(2, bx, -0.07, bw, bh, "Multiplayer (LAN)"),
        MenuButton::new(3, bx, -0.19, bw, bh, "Quit Game"),
    ]
}

pub fn draw_title_screen(
    v: &mut Vec<HudVertex>,
    i: &mut Vec<u32>,
    mouse_ndc: (f32, f32),
    aspect: f32,
) {
    draw_tiled_menu_background(v, i, aspect);

    // Minecraft Title Logo (256x44 native aspect: 5.818)
    let logo_h = 0.18;
    let logo_w = (logo_h * 5.818) / aspect;
    let logo_x = -logo_w * 0.5;
    let logo_y = 0.42;
    add_textured_quad(v, i, logo_x, logo_y, logo_w, logo_h, HudAtlas::LOGO, [1.0, 1.0, 1.0, 1.0]);

    // Buttons
    for btn in get_title_buttons(aspect) {
        btn.draw(v, i, mouse_ndc, aspect);
    }

    // Version text: bottom-left
    let v_text = "Minecraft 1.21.4 (Rust Engine)";
    let v_shadow = [0.25, 0.25, 0.25, 1.0];
    let v_white = [1.0, 1.0, 1.0, 1.0];
    let v_size = 0.026;
    draw_text(v, i, v_text, -0.98 + 0.002 / aspect, -0.96 - 0.002, v_size, v_shadow, aspect);
    draw_text(v, i, v_text, -0.98, -0.96, v_size, v_white, aspect);

    // Copyright text: bottom-right
    let c_text = "Copyright Mojang AB. Do not distribute!";
    let char_w = (v_size / aspect) * 0.85;
    let c_x = 0.98 - (c_text.len() as f32 * char_w);
    draw_text(v, i, c_text, c_x + 0.002 / aspect, -0.96 - 0.002, v_size, v_shadow, aspect);
    draw_text(v, i, c_text, c_x, -0.96, v_size, v_white, aspect);
}
