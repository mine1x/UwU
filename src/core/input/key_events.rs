use winit::event_loop::ActiveEventLoop;
use winit::keyboard::KeyCode;
use super::app_state::App;
use crate::input::LogicCommand;

pub fn handle_keyboard_input(app: &mut App, key: KeyCode, pressed: bool, event_loop: &ActiveEventLoop) {
    app.keys_pressed.insert(key, pressed);
    if !pressed { return; }
    let shift_down = app.keys_pressed.get(&KeyCode::ShiftLeft).copied().unwrap_or(false)
        || app.keys_pressed.get(&KeyCode::ShiftRight).copied().unwrap_or(false);

    if app.game_state == super::game_state::GameState::DirectConnect {
        if key == KeyCode::Enter || key == KeyCode::NumpadEnter {
            let addr = app.direct_ip_input.clone();
            super::menu_action_handler::apply_menu_action(app, crate::core::menu::MenuAction::ConnectLan(addr), event_loop);
            return;
        }
        if key == KeyCode::Escape {
            app.game_state = super::game_state::GameState::LanLobby;
            return;
        }
        if super::text_input::handle_text_input(app, key, shift_down) {
            return;
        }
    }

    if let Some(tx) = &app.command_tx {
        if super::key_shortcuts::handle_hotbar_or_profiler(tx, key, shift_down) {
            return;
        }
        let f3_down = app.keys_pressed.get(&KeyCode::F3).copied().unwrap_or(false);
        if f3_down && key == KeyCode::F4 {
            let _ = tx.send(LogicCommand::ToggleGameMode);
            return;
        }
        match key {
            KeyCode::F3 => { let _ = tx.send(LogicCommand::TogglePieChart); }
            KeyCode::F4 => { let _ = tx.send(LogicCommand::ToggleGameMode); }
            KeyCode::KeyF => { let _ = tx.send(LogicCommand::ToggleFlight); }
            KeyCode::Space => {
                let now = std::time::Instant::now();
                if let Some(last) = app.last_space_time {
                    if now.duration_since(last).as_millis() < 300 {
                        let _ = tx.send(LogicCommand::ToggleFlight);
                        app.last_space_time = None;
                    } else {
                        app.last_space_time = Some(now);
                    }
                } else {
                    app.last_space_time = Some(now);
                }
            }
            KeyCode::KeyG => { let _ = tx.send(LogicCommand::ToggleChunkBorders); }
            KeyCode::KeyE => { let _ = tx.send(LogicCommand::ToggleInventory); }
            KeyCode::KeyZ => { let _ = tx.send(LogicCommand::PrevSlot); }
            KeyCode::KeyX => { let _ = tx.send(LogicCommand::NextSlot); }
            KeyCode::Escape => {
                match app.game_state {
                    super::game_state::GameState::TitleScreen => {
                        app.running.store(false, std::sync::atomic::Ordering::Relaxed);
                        event_loop.exit();
                    }
                    super::game_state::GameState::LanLobby => {
                        app.game_state = super::game_state::GameState::TitleScreen;
                    }
                    super::game_state::GameState::DirectConnect => {
                        app.game_state = super::game_state::GameState::LanLobby;
                    }
                    super::game_state::GameState::Playing => {
                        if app.inventory_open {
                            let _ = tx.send(LogicCommand::CloseInventory);
                        } else {
                            app.game_state = super::game_state::GameState::Paused;
                        }
                    }
                    super::game_state::GameState::Paused => {
                        app.game_state = super::game_state::GameState::Playing;
                    }
                }
            }
            _ => {}
        }
    }
}