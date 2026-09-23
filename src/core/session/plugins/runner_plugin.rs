use bevy::prelude::*;
use winit::event_loop::{ControlFlow, EventLoop};
use crate::app::App as GameApp;

pub struct WgpuRunnerPlugin;

impl Plugin for WgpuRunnerPlugin {
    fn build(&self, app: &mut App) {
        app.set_runner(|mut app| {
            let event_loop = match EventLoop::new() {
                Ok(el) => el,
                Err(_) => return AppExit::from_code(1),
            };
            event_loop.set_control_flow(ControlFlow::Poll);
            let mut game = GameApp::new();
            app.update();
            let _ = event_loop.run_app(&mut game);
            app.update();
            AppExit::Success
        });
    }
}
