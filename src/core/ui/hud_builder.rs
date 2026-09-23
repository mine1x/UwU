use wgpu::util::DeviceExt;
use crate::render::hud_ui::bars::draw_bars_and_crosshair;
use crate::render::hud_ui::piechart::draw_profiler_piechart;
use crate::render::hud_ui::renderer::HudRenderer;
use crate::engine::items::ItemType;
use crate::engine::ResultField;
use super::container_window::draw_container_window;
use super::creative_inventory::draw_creative_inventory_window;
use super::inventory_window::{draw_hotbar_slots, draw_inventory_window};

pub fn update_hud_mesh(
    renderer: &mut HudRenderer,
    device: &wgpu::Device,
    health: f32,
    stamina: f32,
    hunger: f32,
    hotbar: &[Option<(ItemType, u32)>; 9],
    all_slots: &[Option<(ItemType, u32)>; 46],
    inventory_open: bool,
    selected_slot: usize,
    carried_item: Option<(ItemType, u32)>,
    mouse_ndc: (f32, f32),
    profiler_data: Option<(&[ResultField], &str)>,
    aspect: f32,
    is_creative: bool,
    container: Option<&crate::engine::ContainerSnapshot>,
) {
    let mut v = Vec::new();
    let mut i = Vec::new();

    draw_bars_and_crosshair(&mut v, &mut i, health, stamina, hunger, aspect);
    draw_hotbar_slots(&mut v, &mut i, hotbar, selected_slot, aspect);
    if inventory_open {
        if let Some(cs) = container {
            draw_container_window(&mut v, &mut i, cs, all_slots, carried_item, mouse_ndc, aspect);
        } else if is_creative {
            draw_creative_inventory_window(&mut v, &mut i, hotbar, carried_item, mouse_ndc, aspect);
        } else {
            draw_inventory_window(&mut v, &mut i, all_slots, carried_item, mouse_ndc, aspect);
        }
    }
    if let Some((fields, path)) = profiler_data {
        draw_profiler_piechart(&mut v, &mut i, fields, path, aspect);
    }

    renderer.vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("HUD VB"), contents: bytemuck::cast_slice(&v), usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
    });
    renderer.index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("HUD IB"), contents: bytemuck::cast_slice(&i), usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
    });
    renderer.index_count = i.len() as u32;
}
