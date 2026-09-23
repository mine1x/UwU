use std::sync::atomic::Ordering;
use winit::event_loop::ActiveEventLoop;
use crate::core::session::GameState;
use crate::core::window::App;
use crate::core::menu::MenuAction;
use crate::network::{LanClient, LanServer, LanServerDetector};

pub fn apply_menu_action(app: &mut App, action: MenuAction, event_loop: &ActiveEventLoop) {
    match action {
        MenuAction::OpenCreateWorld => {
            app.game_state = GameState::CreateWorld;
        }
        MenuAction::ToggleCreateGameMode => {
            app.create_world_gamemode = match app.create_world_gamemode {
                crate::engine::GameMode::Survival => crate::engine::GameMode::Creative,
                crate::engine::GameMode::Creative => crate::engine::GameMode::Survival,
                _ => crate::engine::GameMode::Survival,
            };
        }
        MenuAction::SelectInputField(idx) => {
            app.active_create_field = idx;
        }
        MenuAction::CreateNewWorld => {
            // Parse or hash seed
            let seed: i64 = if app.world_seed_input.trim().is_empty() {
                let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis();
                (now as i64) ^ 0x5DEECE66D
            } else if let Ok(s) = app.world_seed_input.trim().parse::<i64>() {
                s
            } else {
                let mut h = 0i64;
                for b in app.world_seed_input.as_bytes() {
                    h = h.wrapping_mul(31).wrapping_add(*b as i64);
                }
                h
            };

            if let Some(tx) = &app.command_tx {
                let _ = tx.send(crate::input::LogicCommand::ResetWorld {
                    seed,
                    game_mode: app.create_world_gamemode,
                });
            }
            app.game_state = GameState::Playing;
        }
        MenuAction::StartSingleplayer => {
            app.game_state = GameState::Playing;
        }
        MenuAction::OpenLanLobby => {
            app.game_state = GameState::LanLobby;
            app.lan_detector = Some(LanServerDetector::new());
        }
        MenuAction::OpenDirectConnect => {
            app.game_state = GameState::DirectConnect;
        }
        MenuAction::HostLan => {
            app.lan_detector = None;
            app.local_player_id = 1;
            let srv = LanServer::bind("Steve's LAN World".into(), 25565, app.world_storage.clone()).ok();
            app.lan_server = srv;
            app.game_state = GameState::Playing;
        }
        MenuAction::ConnectLan(addr) => {
            app.lan_detector = None;
            let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis();
            app.local_player_id = (now as u64 & 0x7FFFFFFF) + 2;
            let cli = LanClient::connect(&addr, "Steve").ok();
            app.lan_client = cli;
            app.game_state = GameState::Playing;
        }
        MenuAction::ResumeGame => {
            app.game_state = GameState::Playing;
        }
        MenuAction::QuitToTitle => {
            app.lan_server = None;
            app.lan_client = None;
            app.lan_detector = None;
            app.remote_players.clear();
            app.game_state = GameState::TitleScreen;
        }
        MenuAction::QuitGame => {
            app.running.store(false, Ordering::Relaxed);
            event_loop.exit();
        }
    }
}
