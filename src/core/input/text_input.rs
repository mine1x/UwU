use winit::keyboard::KeyCode;
use super::app_state::App;
use super::game_state::GameState;

pub fn handle_text_input(app: &mut App, key: KeyCode, shift: bool) -> bool {
    if app.game_state != GameState::DirectConnect {
        return false;
    }

    match key {
        KeyCode::Backspace => {
            app.direct_ip_input.pop();
            true
        }
        KeyCode::Period | KeyCode::NumpadDecimal => {
            if app.direct_ip_input.len() < 40 { app.direct_ip_input.push('.'); }
            true
        }
        KeyCode::Semicolon => {
            if shift && app.direct_ip_input.len() < 40 { app.direct_ip_input.push(':'); }
            true
        }
        KeyCode::Minus => {
            if app.direct_ip_input.len() < 40 { app.direct_ip_input.push('-'); }
            true
        }
        _ => {
            if let Some(c) = key_to_char(key) {
                if app.direct_ip_input.len() < 40 {
                    app.direct_ip_input.push(c);
                }
                true
            } else {
                false
            }
        }
    }
}

fn key_to_char(key: KeyCode) -> Option<char> {
    match key {
        KeyCode::Digit0 | KeyCode::Numpad0 => Some('0'),
        KeyCode::Digit1 | KeyCode::Numpad1 => Some('1'),
        KeyCode::Digit2 | KeyCode::Numpad2 => Some('2'),
        KeyCode::Digit3 | KeyCode::Numpad3 => Some('3'),
        KeyCode::Digit4 | KeyCode::Numpad4 => Some('4'),
        KeyCode::Digit5 | KeyCode::Numpad5 => Some('5'),
        KeyCode::Digit6 | KeyCode::Numpad6 => Some('6'),
        KeyCode::Digit7 | KeyCode::Numpad7 => Some('7'),
        KeyCode::Digit8 | KeyCode::Numpad8 => Some('8'),
        KeyCode::Digit9 | KeyCode::Numpad9 => Some('9'),
        KeyCode::KeyA => Some('a'),
        KeyCode::KeyB => Some('b'),
        KeyCode::KeyC => Some('c'),
        KeyCode::KeyD => Some('d'),
        KeyCode::KeyE => Some('e'),
        KeyCode::KeyF => Some('f'),
        KeyCode::KeyG => Some('g'),
        KeyCode::KeyH => Some('h'),
        KeyCode::KeyI => Some('i'),
        KeyCode::KeyJ => Some('j'),
        KeyCode::KeyK => Some('k'),
        KeyCode::KeyL => Some('l'),
        KeyCode::KeyM => Some('m'),
        KeyCode::KeyN => Some('n'),
        KeyCode::KeyO => Some('o'),
        KeyCode::KeyP => Some('p'),
        KeyCode::KeyQ => Some('q'),
        KeyCode::KeyR => Some('r'),
        KeyCode::KeyS => Some('s'),
        KeyCode::KeyT => Some('t'),
        KeyCode::KeyU => Some('u'),
        KeyCode::KeyV => Some('v'),
        KeyCode::KeyW => Some('w'),
        KeyCode::KeyX => Some('x'),
        KeyCode::KeyY => Some('y'),
        KeyCode::KeyZ => Some('z'),
        _ => None,
    }
}
