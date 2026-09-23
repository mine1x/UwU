use crate::engine::render::hud_ui::bars::add_hud_quad;
use crate::engine::render::hud_ui::font::draw_text;
use crate::engine::render::hud_ui::renderer::HudVertex;
use super::menu_bg::draw_tiled_menu_background;
use super::menu_button::MenuButton;

pub fn get_create_world_buttons(aspect: f32, is_creative: bool) -> [MenuButton; 3] {
    let bw = 0.52 / aspect;
    let bh = 0.08;
    let bx = -bw * 0.5;
    let gm_text = if is_creative { "Game Mode: Creative" } else { "Game Mode: Survival" };
    [
        MenuButton::new(40, bx, -0.10, bw, bh, gm_text),
        MenuButton::new(41, bx, -0.26, bw, bh, "Create New World"),
        MenuButton::new(42, bx, -0.38, bw, bh, "Cancel"),
    ]
}

pub fn draw_create_world_screen(
    v: &mut Vec<HudVertex>,
    i: &mut Vec<u32>,
    world_name: &str,
    world_seed: &str,
    game_mode: crate::engine::GameMode,
    active_field: usize,
    mouse_ndc: (f32, f32),
    aspect: f32,
) {
    draw_tiled_menu_background(v, i, aspect);

    // Title: "Create New World"
    let title = "Create New World";
    let t_size = 0.046;
    let char_w = (t_size / aspect) * 0.85;
    let tx = -(title.len() as f32 * char_w) * 0.5;
    draw_text(v, i, title, tx + 0.002 / aspect, 0.80 - 0.002, t_size, [0.25, 0.25, 0.25, 1.0], aspect);
    draw_text(v, i, title, tx, 0.80, t_size, [1.0, 1.0, 1.0, 1.0], aspect);

    let box_w = 0.52 / aspect;
    let box_h = 0.070;
    let box_x = -box_w * 0.5;
    let l_size = 0.028;

    // Field 1: World Name
    let label1 = "World Name";
    draw_text(v, i, label1, box_x, 0.64, l_size, [0.70, 0.70, 0.70, 1.0], aspect);
    let border_color1 = if active_field == 0 { [1.0, 1.0, 1.0, 1.0] } else { [0.5, 0.5, 0.5, 1.0] };
    let y1 = 0.54;
    add_hud_quad(v, i, box_x - 0.004 / aspect, y1 - 0.004, box_w + 0.008 / aspect, box_h + 0.008, border_color1);
    add_hud_quad(v, i, box_x, y1, box_w, box_h, [0.0, 0.0, 0.0, 1.0]);

    let display_name = if active_field == 0 { format!("{}_", world_name) } else { world_name.to_string() };
    draw_text(v, i, &display_name, box_x + 0.012 / aspect, y1 + 0.018, 0.032, [1.0, 1.0, 1.0, 1.0], aspect);

    // Field 2: Seed for the World Generator
    let label2 = "Seed for the World Generator";
    draw_text(v, i, label2, box_x, 0.38, l_size, [0.70, 0.70, 0.70, 1.0], aspect);
    let border_color2 = if active_field == 1 { [1.0, 1.0, 1.0, 1.0] } else { [0.5, 0.5, 0.5, 1.0] };
    let y2 = 0.28;
    add_hud_quad(v, i, box_x - 0.004 / aspect, y2 - 0.004, box_w + 0.008 / aspect, box_h + 0.008, border_color2);
    add_hud_quad(v, i, box_x, y2, box_w, box_h, [0.0, 0.0, 0.0, 1.0]);

    let seed_empty_hint = "Leave blank for a random seed";
    if world_seed.is_empty() && active_field != 1 {
        draw_text(v, i, seed_empty_hint, box_x + 0.012 / aspect, y2 + 0.018, 0.028, [0.45, 0.45, 0.45, 1.0], aspect);
    } else {
        let display_seed = if active_field == 1 { format!("{}_", world_seed) } else { world_seed.to_string() };
        draw_text(v, i, &display_seed, box_x + 0.012 / aspect, y2 + 0.018, 0.032, [1.0, 1.0, 1.0, 1.0], aspect);
    }

    // Buttons
    for btn in get_create_world_buttons(aspect, game_mode.is_creative()) {
        btn.draw(v, i, mouse_ndc, aspect);
    }
}
