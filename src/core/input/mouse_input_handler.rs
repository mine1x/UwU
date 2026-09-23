use winit::event::{ElementState, MouseButton};
use winit::event_loop::ActiveEventLoop;
use crate::core::session::GameState;
use crate::core::window::App;
use crate::core::menu::handle_menu_click;
use crate::input::LogicCommand;

pub fn handle_mouse_input(
    app: &mut App,
    state: ElementState,
    button: MouseButton,
    event_loop: &ActiveEventLoop,
) {
    if button == MouseButton::Middle {
        if state == ElementState::Pressed {
            app.is_middle_dragging = true;
            app.last_drag_pos = app.mouse_pos;
        } else {
            app.is_middle_dragging = false;
        }
        return;
    }

    if app.game_state != GameState::Playing {
        if state == ElementState::Pressed && button == MouseButton::Left {
            if let Some(r) = &app.renderer {
                let aspect = r.config.width as f32 / r.config.height as f32;
                let mouse_ndc = (
                    (app.mouse_pos.0 / r.config.width as f32) * 2.0 - 1.0,
                    1.0 - (app.mouse_pos.1 / r.config.height as f32) * 2.0,
                );
                let servers = app.lan_detector.as_ref().map(|d| d.get_servers()).unwrap_or_default();
                if let Some(action) = handle_menu_click(app.game_state, mouse_ndc, aspect, &servers, &app.direct_ip_input) {
                    super::menu_action_handler::apply_menu_action(app, action, event_loop);
                }
            }
        }
        return;
    }

    if let (Some(tx), Some(r)) = (&app.command_tx, &app.renderer) {
        let aspect = r.config.width as f32 / r.config.height as f32;
        let is_shift = app.keys_pressed.get(&winit::keyboard::KeyCode::ShiftLeft).copied().unwrap_or(false)
            || app.keys_pressed.get(&winit::keyboard::KeyCode::ShiftRight).copied().unwrap_or(false);
        let _ = tx.send(LogicCommand::MouseAction {
            button, is_pressed: state == ElementState::Pressed, aspect,
            mouse_pos: app.mouse_pos, screen_size: (r.config.width as f32, r.config.height as f32), is_shift,
        });
    }
}
