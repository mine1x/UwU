use crate::render::hud_ui::bars::add_hud_quad;
use crate::render::hud_ui::font::draw_text;
use crate::render::hud_ui::renderer::HudVertex;
use super::menu_bg::draw_tiled_menu_background;
use super::menu_button::MenuButton;

pub fn get_direct_connect_buttons(aspect: f32) -> [MenuButton; 2] {
    let bw = 0.52 / aspect;
    let bh = 0.08;
    let bx = -bw * 0.5;
    [
        MenuButton::new(30, bx, -0.20, bw, bh, "Join Server"),
        MenuButton::new(31, bx, -0.32, bw, bh, "Cancel"),
    ]
}

pub fn draw_direct_connect_screen(
    v: &mut Vec<HudVertex>,
    i: &mut Vec<u32>,
    ip_text: &str,
    mouse_ndc: (f32, f32),
    aspect: f32,
) {
    draw_tiled_menu_background(v, i, aspect);

    // Title: "Direct Connection"
    let title = "Direct Connection";
    let t_size = 0.046;
    let char_w = (t_size / aspect) * 0.85;
    let tx = -(title.len() as f32 * char_w) * 0.5;
    draw_text(v, i, title, tx + 0.002 / aspect, 0.70 - 0.002, t_size, [0.25, 0.25, 0.25, 1.0], aspect);
    draw_text(v, i, title, tx, 0.70, t_size, [1.0, 1.0, 1.0, 1.0], aspect);

    // Subtitle: "Server Address"
    let label = "Server Address";
    let l_size = 0.030;
    let box_w = 0.52 / aspect;
    let box_x = -box_w * 0.5;
    draw_text(v, i, label, box_x, 0.25, l_size, [0.65, 0.65, 0.65, 1.0], aspect);

    // Input Box (black with border)
    let box_h = 0.075;
    let box_y = 0.12;
    add_hud_quad(v, i, box_x - 0.004 / aspect, box_y - 0.004, box_w + 0.008 / aspect, box_h + 0.008, [0.6, 0.6, 0.6, 1.0]);
    add_hud_quad(v, i, box_x, box_y, box_w, box_h, [0.0, 0.0, 0.0, 1.0]);

    // Input text inside box
    let display_text = format!("{}_", ip_text);
    draw_text(v, i, &display_text, box_x + 0.015 / aspect, box_y + 0.02, 0.034, [1.0, 1.0, 1.0, 1.0], aspect);

    // Action buttons: "Join Server" and "Cancel"
    for btn in get_direct_connect_buttons(aspect) {
        btn.draw(v, i, mouse_ndc, aspect);
    }
}
