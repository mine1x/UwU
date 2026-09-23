use crate::core::session::GameState;
use crate::network::DiscoveredServer;
use super::lan_screen::get_lan_static_buttons;
use super::menu_button::MenuButton;
use super::pause_screen::get_pause_buttons;
use super::title_screen::get_title_buttons;

#[derive(Clone, Debug, PartialEq)]
pub enum MenuAction {
    OpenCreateWorld,
    ToggleCreateGameMode,
    SelectInputField(usize),
    CreateNewWorld,
    StartSingleplayer,
    OpenLanLobby,
    OpenDirectConnect,
    HostLan,
    ConnectLan(String),
    ResumeGame,
    QuitToTitle,
    QuitGame,
}

pub fn handle_menu_click(
    state: GameState,
    mouse: (f32, f32),
    aspect: f32,
    servers: &[DiscoveredServer],
    direct_ip: &str,
    game_mode_name: &str,
) -> Option<MenuAction> {
    match state {
        GameState::TitleScreen => {
            for btn in get_title_buttons(aspect) {
                if btn.is_hovered(mouse) {
                    return match btn.id {
                        1 => Some(MenuAction::OpenCreateWorld),
                        2 => Some(MenuAction::OpenLanLobby),
                        3 => Some(MenuAction::QuitGame),
                        _ => None,
                    };
                }
            }
        }
        GameState::CreateWorld => {
            // Check text box clicks
            let box_w = 0.52 / aspect;
            let box_h = 0.070;
            let box_x = -box_w * 0.5;
            let (mx, my) = mouse;

            let y1 = 0.54;
            if mx >= box_x && mx <= box_x + box_w && my >= y1 && my <= y1 + box_h {
                return Some(MenuAction::SelectInputField(0));
            }

            let y2 = 0.28;
            if mx >= box_x && mx <= box_x + box_w && my >= y2 && my <= y2 + box_h {
                return Some(MenuAction::SelectInputField(1));
            }

            for btn in super::create_world_screen::get_create_world_buttons(aspect, game_mode_name == "Creative") {
                if btn.is_hovered(mouse) {
                    return match btn.id {
                        40 => Some(MenuAction::ToggleCreateGameMode),
                        41 => Some(MenuAction::CreateNewWorld),
                        42 => Some(MenuAction::QuitToTitle),
                        _ => None,
                    };
                }
            }
        }
        GameState::LanLobby => {
            for btn in get_lan_static_buttons(aspect) {
                if btn.is_hovered(mouse) {
                    return match btn.id {
                        10 => Some(MenuAction::HostLan),
                        11 => Some(MenuAction::OpenDirectConnect),
                        12 => Some(MenuAction::QuitToTitle),
                        _ => None,
                    };
                }
            }
            let bw = 0.68 / aspect;
            let bh = 0.075;
            let bx = -bw * 0.5;
            for (idx, srv) in servers.iter().take(4).enumerate() {
                let sy = 0.50 - idx as f32 * 0.10;
                let btn = MenuButton::new(100 + idx as u32, bx, sy, bw, bh, "");
                if btn.is_hovered(mouse) {
                    return Some(MenuAction::ConnectLan(srv.address.clone()));
                }
            }
        }
        GameState::DirectConnect => {
            for btn in super::direct_connect_screen::get_direct_connect_buttons(aspect) {
                if btn.is_hovered(mouse) {
                    return match btn.id {
                        30 => Some(MenuAction::ConnectLan(direct_ip.to_string())),
                        31 => Some(MenuAction::OpenLanLobby),
                        _ => None,
                    };
                }
            }
        }
        GameState::Paused => {
            for btn in get_pause_buttons(aspect) {
                if btn.is_hovered(mouse) {
                    return match btn.id {
                        20 => Some(MenuAction::ResumeGame),
                        21 => Some(MenuAction::HostLan),
                        22 => Some(MenuAction::QuitToTitle),
                        _ => None,
                    };
                }
            }
        }
        GameState::Playing => {}
    }
    None
}
