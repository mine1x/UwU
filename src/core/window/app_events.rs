use std::sync::atomic::Ordering;
use std::time::Instant;
use winit::application::ApplicationHandler;
use winit::event::{ElementState, KeyEvent, MouseScrollDelta, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::PhysicalKey;
use winit::window::WindowId;

use super::app_state::App;
use crate::input::LogicCommand;

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.renderer.is_none() {
            super::app_init::init_app(self, event_loop);
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                self.running.store(false, Ordering::Relaxed);
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                if let Some(r) = &mut self.renderer { r.resize(size); }
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let scroll_y = match delta {
                    MouseScrollDelta::LineDelta(_, y) => y,
                    MouseScrollDelta::PixelDelta(pos) => (pos.y as f32) * 0.05,
                };
                if let Some(tx) = &self.command_tx {
                    let _ = tx.send(LogicCommand::ZoomCamera(-scroll_y * 2.0));
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                let cur = (position.x as f32, position.y as f32);
                self.mouse_pos = cur;
                if self.is_middle_dragging {
                    let dx = cur.0 - self.last_drag_pos.0;
                    self.last_drag_pos = cur;
                    if let Some(tx) = &self.command_tx {
                        let _ = tx.send(LogicCommand::RotateCamera(-dx * 0.008));
                    }
                }
                if let (Some(tx), Some(r)) = (&self.command_tx, &self.renderer) {
                    let aspect = r.config.width as f32 / r.config.height as f32;
                    let _ = tx.send(LogicCommand::UpdateCursor {
                        aspect,
                        mouse_pos: self.mouse_pos,
                        screen_size: (r.config.width as f32, r.config.height as f32),
                    });
                }
            }
            WindowEvent::MouseInput { state: m_state, button, .. } => {
                super::mouse_input_handler::handle_mouse_input(self, m_state, button, event_loop);
            }
            WindowEvent::KeyboardInput { event: KeyEvent { physical_key: PhysicalKey::Code(key), state: k_state, .. }, .. } => {
                super::key_events::handle_keyboard_input(self, key, k_state == ElementState::Pressed, event_loop);
            }
            WindowEvent::RedrawRequested => {
                let now = Instant::now();
                let dt = now.duration_since(self.last_frame_time).as_secs_f32().min(0.05);
                self.last_frame_time = now;
                super::render_loop::render_frame(self, event_loop, dt);
            }
            _ => (),
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(w) = &self.window { w.request_redraw(); }
    }
}