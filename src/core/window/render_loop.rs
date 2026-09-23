use winit::event_loop::ActiveEventLoop;
use super::app_state::App;
use super::game_state::GameState;

pub fn render_frame(app: &mut App, event_loop: &ActiveEventLoop, dt: f32) {
    if app.game_state == GameState::Playing {
        super::render_movement::send_movement_inputs(app);
    }
    if app.game_state == GameState::Playing || app.game_state == GameState::Paused {
        super::lan_sync::update_lan_network(app);
    }

    let is_in_world = app.game_state == GameState::Playing || app.game_state == GameState::Paused;
    let maybe_snapshot = {
        let mut lock = app.latest_snapshot.lock().unwrap();
        if let Some(s) = lock.as_mut() {
            let snap = s.clone();
            if is_in_world {
                s.world_mesh = None;
            }
            Some(snap)
        } else {
            None
        }
    };

    if let Some(r) = &mut app.renderer {
        let m_ndc = (
            (app.mouse_pos.0 / r.config.width as f32) * 2.0 - 1.0,
            1.0 - (app.mouse_pos.1 / r.config.height as f32) * 2.0,
        );
        match app.game_state {
            GameState::Playing => {
                if let Some(snapshot) = maybe_snapshot {
                    app.inventory_open = snapshot.inventory_open;
                    r.update_from_snapshot(&snapshot, &app.remote_players, dt);
                }
            }
            GameState::Paused => {
                if let Some(snapshot) = maybe_snapshot {
                    app.inventory_open = snapshot.inventory_open;
                    r.update_from_snapshot(&snapshot, &app.remote_players, dt);
                }
                r.update_menu(app.game_state, &[], &app.direct_ip_input, m_ndc);
            }
            GameState::TitleScreen | GameState::LanLobby | GameState::DirectConnect => {
                let servers = app.lan_detector.as_ref().map(|d| d.get_servers()).unwrap_or_default();
                r.update_menu(app.game_state, &servers, &app.direct_ip_input, m_ndc);
            }
        }
        match r.render() {
            Ok(_) => {}
            Err(wgpu::SurfaceError::Lost) => r.resize(r.size),
            Err(wgpu::SurfaceError::OutOfMemory) => event_loop.exit(),
            Err(e) => eprintln!("Vulkan Render Error: {:?}", e),
        }
    }
}