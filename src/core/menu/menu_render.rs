use wgpu::util::DeviceExt;
use super::lan_screen::draw_lan_screen;
use super::pause_screen::draw_pause_screen;
use super::title_screen::draw_title_screen;
use super::direct_connect_screen::draw_direct_connect_screen;
use crate::app::game_state::GameState;
use crate::network::DiscoveredServer;
use crate::render::hud_ui::renderer::HudRenderer;

pub fn update_menu_hud_mesh(
    renderer: &mut HudRenderer,
    device: &wgpu::Device,
    state: GameState,
    servers: &[DiscoveredServer],
    ip_input: &str,
    mouse_ndc: (f32, f32),
    aspect: f32,
) {
    let mut v = Vec::new();
    let mut i = Vec::new();

    match state {
        GameState::TitleScreen => {
            draw_title_screen(&mut v, &mut i, mouse_ndc, aspect);
        }
        GameState::LanLobby => {
            draw_lan_screen(&mut v, &mut i, servers, mouse_ndc, aspect);
        }
        GameState::DirectConnect => {
            draw_direct_connect_screen(&mut v, &mut i, ip_input, mouse_ndc, aspect);
        }
        GameState::Paused => {
            draw_pause_screen(&mut v, &mut i, mouse_ndc, aspect);
        }
        GameState::Playing => {}
    }

    renderer.vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Menu HUD VB"),
        contents: bytemuck::cast_slice(&v),
        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
    });
    renderer.index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Menu HUD IB"),
        contents: bytemuck::cast_slice(&i),
        usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
    });
    renderer.index_count = i.len() as u32;
}
