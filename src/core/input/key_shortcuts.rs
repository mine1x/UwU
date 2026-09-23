use winit::keyboard::KeyCode;
use std::sync::mpsc::Sender;
use crate::input::LogicCommand;

pub fn handle_hotbar_or_profiler(tx: &Sender<LogicCommand>, key: KeyCode, shift: bool) -> bool {
    let slot = match key {
        KeyCode::Digit1 => 0,
        KeyCode::Digit2 => 1,
        KeyCode::Digit3 => 2,
        KeyCode::Digit4 => 3,
        KeyCode::Digit5 => 4,
        KeyCode::Digit6 => 5,
        KeyCode::Digit7 => 6,
        KeyCode::Digit8 => 7,
        KeyCode::Digit9 => 8,
        KeyCode::Digit0 => {
            let _ = tx.send(LogicCommand::ProfilerNavigate(0));
            return true;
        }
        _ => return false,
    };
    if shift {
        let _ = tx.send(LogicCommand::ProfilerNavigate(slot + 1));
    } else {
        let _ = tx.send(LogicCommand::SelectSlot(slot));
    }
    true
}
