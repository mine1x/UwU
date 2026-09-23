use bevy::math::Vec3;
use winit::keyboard::KeyCode;
use super::app_state::App;
use crate::input::LogicCommand;

pub fn send_movement_inputs(app: &App) {
    if let Some(tx) = &app.command_tx {
        let cam_rot = {
            let lock = app.latest_snapshot.lock().unwrap();
            lock.as_ref().map(|s| s.camera_rotation).unwrap_or(0.0)
        };
        let forward = Vec3::new(-cam_rot.sin(), 0.0, -cam_rot.cos()).normalize();
        let right = Vec3::new(-forward.z, 0.0, forward.x);

        let mut world_move = Vec3::ZERO;
        if *app.keys_pressed.get(&KeyCode::KeyW).unwrap_or(&false) { world_move += forward; }
        if *app.keys_pressed.get(&KeyCode::KeyS).unwrap_or(&false) { world_move -= forward; }
        if *app.keys_pressed.get(&KeyCode::KeyA).unwrap_or(&false) { world_move -= right; }
        if *app.keys_pressed.get(&KeyCode::KeyD).unwrap_or(&false) { world_move += right; }

        if world_move.length_squared() > 0.001 { world_move = world_move.normalize(); }
        let _ = tx.send(LogicCommand::MoveInput(world_move));

        let jump = *app.keys_pressed.get(&KeyCode::Space).unwrap_or(&false);
        let _ = tx.send(LogicCommand::Jump(jump));

        let sneak = *app.keys_pressed.get(&KeyCode::ShiftLeft).unwrap_or(&false)
            || *app.keys_pressed.get(&KeyCode::ShiftRight).unwrap_or(&false);
        let _ = tx.send(LogicCommand::Sneak(sneak));

        let sprint = *app.keys_pressed.get(&KeyCode::ControlLeft).unwrap_or(&false)
            || *app.keys_pressed.get(&KeyCode::ControlRight).unwrap_or(&false);
        let _ = tx.send(LogicCommand::Sprint(sprint));
    }
}
