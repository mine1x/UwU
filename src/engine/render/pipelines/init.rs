use std::sync::Arc;
use wgpu::util::DeviceExt;
use winit::window::Window;

use bevy::math::Mat4;
use crate::render::hud_ui::HudRenderer;
use crate::render::types::CameraUniform;

pub struct InitializedPipeline {
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
    pub block_texture: wgpu::Texture,
}

pub async fn create_graphics_pipeline(window: Arc<Window>) -> InitializedPipeline {
    let size = window.inner_size();
    let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
        backends: wgpu::Backends::VULKAN | wgpu::Backends::PRIMARY,
        ..Default::default()
    });

    let surface = instance.create_surface(window.clone()).expect("Failed surface");
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        })
        .await
        .expect("Failed adapter");

    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor::default(), None)
        .await
        .expect("Failed device");

    let surface_caps = surface.get_capabilities(&adapter);
    let surface_format = surface_caps.formats.iter().copied().find(|f| f.is_srgb()).unwrap_or(surface_caps.formats[0]);
    let present_mode = if surface_caps.present_modes.contains(&wgpu::PresentMode::Mailbox) {
        wgpu::PresentMode::Mailbox
    } else {
        wgpu::PresentMode::AutoNoVsync
    };

    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: size.width.max(1),
        height: size.height.max(1),
        present_mode,
        alpha_mode: surface_caps.alpha_modes[0],
        view_formats: vec![],
        desired_maximum_frame_latency: 1,
    };
    surface.configure(&device, &config);

    let depth_texture = super::resources::create_depth_texture(&device, &config);
    let (shadow_texture_view, shadow_bind_group) = super::shadow_res::create_shadow_resources(&device);
    let (block_texture, _t_view, _t_sampler, t_bgl, texture_bind_group) = super::texture_res::create_texture_atlas_resources(&device, &queue);

    let camera_uniform = CameraUniform { view_proj: Mat4::IDENTITY.to_cols_array(), light_view_proj: Mat4::IDENTITY.to_cols_array(), camera_pos: [0.0; 4], fog: [0.0; 4] };
    let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Cam Buffer"),
        contents: bytemuck::cast_slice(&[camera_uniform]),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    let (render_pipeline, overlay_pipeline, shadow_pipeline, camera_bind_group) =
        super::resources::build_pipelines(&device, &config, &camera_buffer, &t_bgl);

    let hud_renderer = HudRenderer::new(&device, &queue, config.format);

    InitializedPipeline {
        surface, device, queue, config, render_pipeline, shadow_pipeline, overlay_pipeline,
        hud_renderer, camera_buffer, camera_bind_group, depth_texture, shadow_texture_view, shadow_bind_group,
        texture_bind_group, block_texture,
    }
}