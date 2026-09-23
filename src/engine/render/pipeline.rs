use std::sync::Arc;
use wgpu::util::DeviceExt;
use winit::{dpi::PhysicalSize, window::Window};

use super::hud_ui::HudRenderer;
use super::types::Vertex;
use crate::engine::RenderSnapshot;

pub struct Renderer {
    pub surface: wgpu::Surface<'static>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub render_pipeline: wgpu::RenderPipeline,
    pub shadow_pipeline: wgpu::RenderPipeline,
    pub overlay_pipeline: wgpu::RenderPipeline,
    pub hud_renderer: HudRenderer,
    pub camera_buffer: wgpu::Buffer,
    pub camera_bind_group: wgpu::BindGroup,
    pub depth_texture: wgpu::TextureView,
    pub shadow_texture_view: wgpu::TextureView,
    pub shadow_bind_group: wgpu::BindGroup,
    pub texture_bind_group: wgpu::BindGroup,
    pub world_vertex_buffer: wgpu::Buffer,
    pub world_index_buffer: wgpu::Buffer,
    pub world_index_count: u32,
    pub player_vertex_buffer: wgpu::Buffer,
    pub player_index_buffer: wgpu::Buffer,
    pub player_index_count: u32,
    pub overlay_vertex_buffer: wgpu::Buffer,
    pub overlay_index_buffer: wgpu::Buffer,
    pub overlay_index_count: u32,
    pub block_texture: wgpu::Texture,
    pub water_anim: super::water_anim::WaterAnimator,
    pub anim_time: f32,
    pub size: PhysicalSize<u32>,
}

impl Renderer {
    pub async fn new(window: Arc<Window>) -> Self {
        let size = window.inner_size();
        let p = super::pipelines::init::create_graphics_pipeline(window).await;
        let (d_v, d_i) = ([Vertex { position: [0.0; 3], normal: [0.0; 3], uv: [0.0; 2], tex_layer: 0.0 }], [0u32]);
        let mk_b = |lbl, u, bytes: &[u8]| p.device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some(lbl), contents: bytes, usage: u | wgpu::BufferUsages::COPY_DST });
        let v_bytes = bytemuck::cast_slice(&d_v);
        let i_bytes = bytemuck::cast_slice(&d_i);
        let (wvb, pvb, ovb) = (mk_b("WVB", wgpu::BufferUsages::VERTEX, v_bytes), mk_b("PVB", wgpu::BufferUsages::VERTEX, v_bytes), mk_b("OVB", wgpu::BufferUsages::VERTEX, v_bytes));
        let (wib, pib, oib) = (mk_b("WIB", wgpu::BufferUsages::INDEX, i_bytes), mk_b("PIB", wgpu::BufferUsages::INDEX, i_bytes), mk_b("OIB", wgpu::BufferUsages::INDEX, i_bytes));

        Self {
            surface: p.surface, device: p.device, queue: p.queue, config: p.config,
            render_pipeline: p.render_pipeline, shadow_pipeline: p.shadow_pipeline, overlay_pipeline: p.overlay_pipeline,
            hud_renderer: p.hud_renderer, camera_buffer: p.camera_buffer, camera_bind_group: p.camera_bind_group,
            depth_texture: p.depth_texture, shadow_texture_view: p.shadow_texture_view, shadow_bind_group: p.shadow_bind_group,
            texture_bind_group: p.texture_bind_group,
            world_vertex_buffer: wvb, world_index_buffer: wib, world_index_count: 0,
            player_vertex_buffer: pvb, player_index_buffer: pib, player_index_count: 0,
            overlay_vertex_buffer: ovb, overlay_index_buffer: oib, overlay_index_count: 0,
            block_texture: p.block_texture, water_anim: super::water_anim::WaterAnimator::new(),
            anim_time: 0.0, size,
        }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            self.depth_texture = super::pipelines::resources::create_depth_texture(&self.device, &self.config);
        }
    }

    pub fn update_from_snapshot(&mut self, snapshot: &RenderSnapshot, remote_players: &[crate::network::RemotePlayer], dt: f32) {
        self.anim_time += dt.min(0.1);
        self.water_anim.update_water_texture(&self.queue, &self.block_texture, self.anim_time);
        super::pipelines::update::update_renderer_snapshot(self, snapshot, remote_players);
    }

    pub fn update_menu(
        &mut self,
        state: crate::app::game_state::GameState,
        servers: &[crate::network::DiscoveredServer],
        ip_input: &str,
        mouse_ndc: (f32, f32),
    ) {
        let aspect = self.config.width as f32 / self.config.height as f32;
        crate::core::menu::update_menu_hud_mesh(
            &mut self.hud_renderer, &self.device, state, servers, ip_input, mouse_ndc, aspect,
        );
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        super::pipelines::render::execute_render_passes(self)
    }
}