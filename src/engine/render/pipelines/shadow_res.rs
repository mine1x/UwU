pub fn create_shadow_resources(device: &wgpu::Device) -> (wgpu::TextureView, wgpu::BindGroup) {
    let shadow_texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Shadow Texture"),
        size: wgpu::Extent3d { width: 2048, height: 2048, depth_or_array_layers: 1 },
        mip_level_count: 1, sample_count: 1, dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Depth32Float,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
    });
    let shadow_texture_view = shadow_texture.create_view(&wgpu::TextureViewDescriptor::default());

    let shadow_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        label: Some("Shadow Sampler"),
        address_mode_u: wgpu::AddressMode::ClampToEdge, address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge, mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Linear, compare: Some(wgpu::CompareFunction::LessEqual),
        ..Default::default()
    });

    let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("shadow_bgl"),
        entries: &[
            wgpu::BindGroupLayoutEntry { binding: 0, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Texture { sample_type: wgpu::TextureSampleType::Depth, view_dimension: wgpu::TextureViewDimension::D2, multisampled: false }, count: None },
            wgpu::BindGroupLayoutEntry { binding: 1, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Comparison), count: None },
        ],
    });

    let bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("shadow_bg"), layout: &bgl,
        entries: &[
            wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&shadow_texture_view) },
            wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(&shadow_sampler) },
        ],
    });

    (shadow_texture_view, bg)
}