use crate::render::hud_ui::font::draw_text;
use crate::render::hud_ui::renderer::HudVertex;
use crate::network::DiscoveredServer;
use super::menu_bg::draw_tiled_menu_background;
use super::menu_button::MenuButton;

pub fn get_lan_static_buttons(aspect: f32) -> [MenuButton; 3] {
    let bw = 0.52 / aspect;
    let bh = 0.08;
    let bx = -bw * 0.5;
    [
        MenuButton::new(10, bx, -0.48, bw, bh, "Host LAN World"),
        MenuButton::new(11, bx, -0.60, bw, bh, "Direct Connect"),
        MenuButton::new(12, bx, -0.72, bw, bh, "Cancel"),
    ]
}

pub fn draw_lan_screen(
    v: &mut Vec<HudVertex>,
    i: &mut Vec<u32>,
    servers: &[DiscoveredServer],
    mouse_ndc: (f32, f32),
    aspect: f32,
) {
    draw_tiled_menu_background(v, i, aspect);

    // Header text
    let title = "Play Multiplayer (LAN)";
    let t_size = 0.046;
    let char_w = (t_size / aspect) * 0.85;
    let tx = -(title.len() as f32 * char_w) * 0.5;
    draw_text(v, i, title, tx + 0.002 / aspect, 0.80 - 0.002, t_size, [0.25, 0.25, 0.25, 1.0], aspect);
    draw_text(v, i, title, tx, 0.80, t_size, [1.0, 1.0, 1.0, 1.0], aspect);

    // Status / server list
    let status = if servers.is_empty() {
        "Scanning for games on your local network..."
    } else {
        "Discovered LAN worlds:"
    };
    let s_size = 0.030;
    let s_char_w = (s_size / aspect) * 0.85;
    let sx = -(status.len() as f32 * s_char_w) * 0.5;
    draw_text(v, i, status, sx, 0.65, s_size, [0.75, 0.75, 0.75, 1.0], aspect);

    // List discovered servers
    let bw = 0.68 / aspect;
    let bh = 0.075;
    let bx = -bw * 0.5;
    for (idx, s) in servers.iter().take(4).enumerate() {
        let sy = 0.50 - idx as f32 * 0.10;
        let btn = MenuButton::new(100 + idx as u32, bx, sy, bw, bh, "Join LAN Server");
        btn.draw(v, i, mouse_ndc, aspect);
        let s_info = format!("{} ({})", s.motd, s.address);
        draw_text(v, i, &s_info, bx + 0.02 / aspect, sy + 0.02, 0.028, [1.0, 1.0, 0.5, 1.0], aspect);
    }

    // Static action buttons
    for btn in get_lan_static_buttons(aspect) {
        btn.draw(v, i, mouse_ndc, aspect);
    }
}
