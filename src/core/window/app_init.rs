use std::sync::{mpsc, Arc};
use std::time::Instant;
use winit::dpi::PhysicalSize;
use winit::event_loop::ActiveEventLoop;
use winit::window::Window;

use super::app_state::App;
use super::logic_worker::LogicWorker;
use crate::input::LogicCommand;
use crate::render::Renderer;

pub fn init_app(app: &mut App, event_loop: &ActiveEventLoop) {
    let window_attrs = Window::default_attributes()
        .with_title("Zomboid Engine [E to open Inventory, WASD, Crisp Hover Overlay]")
        .with_inner_size(PhysicalSize::new(1280, 720));

    let window = Arc::new(event_loop.create_window(window_attrs).unwrap());
    let renderer = pollster::block_on(Renderer::new(window.clone()));
    app.renderer = Some(renderer);
    app.window = Some(window);

    let (cmd_tx, cmd_rx) = mpsc::channel::<LogicCommand>();
    app.command_tx = Some(cmd_tx);

    let (block_tx, block_rx) = mpsc::channel::<crate::network::Packet>();
    app.block_event_rx = Some(block_rx);

    let handle = LogicWorker::spawn(app.latest_snapshot.clone(), cmd_rx, block_tx, app.running.clone(), app.world_storage.clone());
    app.logic_thread = Some(handle);
    app.last_frame_time = Instant::now();
}