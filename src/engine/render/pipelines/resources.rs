
use crate::render::types::Vertex;

pub fn create_depth_texture(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> wgpu::TextureView {
    let size = wgpu::Extent3d { width: config.width.max(1), height: config.height.max(1), depth_or_array_layers: 1 };
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("depth"), size, mip_level_count: 1, sample_count: 1, dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Depth32Float, usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING, view_formats: &[],
    });
    texture.create_view(&wgpu::TextureViewDescriptor::default())
}

pub fn build_pipelines(
    device: &wgpu::Device, config: &wgpu::SurfaceConfiguration, camera_buffer: &wgpu::Buffer, texture_bgl: &wgpu::BindGroupLayout
) -> (wgpu::RenderPipeline, wgpu::RenderPipeline, wgpu::RenderPipeline, wgpu::BindGroup) {
    
    let cam_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("cam_bgl"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0, visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None }, count: None,
        }],
    });
    let cam_bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("cam_bg"), layout: &cam_bgl,
        entries: &[wgpu::BindGroupEntry { binding: 0, resource: camera_buffer.as_entire_binding() }],
    });

    let shadow_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("sbgl"),
        entries: &[
            wgpu::BindGroupLayoutEntry { binding: 0, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Texture { sample_type: wgpu::TextureSampleType::Depth, view_dimension: wgpu::TextureViewDimension::D2, multisampled: false }, count: None },
            wgpu::BindGroupLayoutEntry { binding: 1, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Comparison), count: None },
        ],
    });

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Shader"), source: wgpu::ShaderSource::Wgsl(include_str!("../../shader.wgsl").into()),
    });
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("PL"), bind_group_layouts: &[&cam_bgl, &shadow_bgl, texture_bgl], push_constant_ranges: &[],
    });

    let rp = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("3D"), layout: Some(&layout),
        vertex: wgpu::VertexState { module: &shader, entry_point: Some("vs_main"), buffers: &[Vertex::desc()], compilation_options: Default::default() },
        fragment: Some(wgpu::FragmentState { module: &shader, entry_point: Some("fs_main"), targets: &[Some(wgpu::ColorTargetState { format: config.format, blend: Some(wgpu::BlendState::REPLACE), write_mask: wgpu::ColorWrites::ALL })], compilation_options: Default::default() }),
        primitive: wgpu::PrimitiveState { cull_mode: Some(wgpu::Face::Back), ..Default::default() },
        depth_stencil: Some(wgpu::DepthStencilState { format: wgpu::TextureFormat::Depth32Float, depth_write_enabled: true, depth_compare: wgpu::CompareFunction::Less, stencil: wgpu::StencilState::default(), bias: wgpu::DepthBiasState::default() }),
        multisample: wgpu::MultisampleState::default(), multiview: None, cache: None,
    });

    let op = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Overlay"), layout: Some(&layout),
        vertex: wgpu::VertexState { module: &shader, entry_point: Some("vs_main"), buffers: &[Vertex::desc()], compilation_options: Default::default() },
        fragment: Some(wgpu::FragmentState { module: &shader, entry_point: Some("fs_main"), targets: &[Some(wgpu::ColorTargetState { format: config.format, blend: Some(wgpu::BlendState::ALPHA_BLENDING), write_mask: wgpu::ColorWrites::ALL })], compilation_options: Default::default() }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: Some(wgpu::DepthStencilState { format: wgpu::TextureFormat::Depth32Float, depth_write_enabled: false, depth_compare: wgpu::CompareFunction::Always, stencil: wgpu::StencilState::default(), bias: wgpu::DepthBiasState::default() }),
        multisample: wgpu::MultisampleState::default(), multiview: None, cache: None,
    });

    let s_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: Some("SL"), bind_group_layouts: &[&cam_bgl], push_constant_ranges: &[] });
    let sp = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Shadow"), layout: Some(&s_layout),
        vertex: wgpu::VertexState { module: &shader, entry_point: Some("vs_shadow"), buffers: &[Vertex::desc()], compilation_options: Default::default() },
        fragment: None, primitive: wgpu::PrimitiveState { cull_mode: Some(wgpu::Face::Back), ..Default::default() },
        depth_stencil: Some(wgpu::DepthStencilState { format: wgpu::TextureFormat::Depth32Float, depth_write_enabled: true, depth_compare: wgpu::CompareFunction::LessEqual, stencil: wgpu::StencilState::default(), bias: wgpu::DepthBiasState { constant: 2, slope_scale: 2.0, clamp: 0.0 } }),
        multisample: wgpu::MultisampleState::default(), multiview: None, cache: None,
    });

    (rp, op, sp, cam_bg)
}