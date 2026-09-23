use wgpu::util::DeviceExt;
use crate::render::atlas::TextureAtlas;

pub fn create_texture_atlas_resources(device: &wgpu::Device, queue: &wgpu::Queue) -> (wgpu::Texture, wgpu::TextureView, wgpu::Sampler, wgpu::BindGroupLayout, wgpu::BindGroup) {
    let atlas_bytes = TextureAtlas::generate_pixel_atlas();
    let size = wgpu::Extent3d {
        width: TextureAtlas::RESOLUTION,
        height: TextureAtlas::RESOLUTION,
        depth_or_array_layers: TextureAtlas::LAYER_COUNT,
    };

    let texture = device.create_texture_with_data(
        queue,
        &wgpu::TextureDescriptor {
            label: Some("Block Texture Array"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        },
        wgpu::util::TextureDataOrder::LayerMajor,
        &atlas_bytes,
    );

    let view = texture.create_view(&wgpu::TextureViewDescriptor {
        dimension: Some(wgpu::TextureViewDimension::D2Array),
        ..Default::default()
    });

    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        label: Some("Block Sampler"),
        address_mode_u: wgpu::AddressMode::Repeat,
        address_mode_v: wgpu::AddressMode::Repeat,
        address_mode_w: wgpu::AddressMode::Repeat,
        mag_filter: wgpu::FilterMode::Nearest,
        min_filter: wgpu::FilterMode::Nearest,
        mipmap_filter: wgpu::FilterMode::Nearest,
        ..Default::default()
    });

    let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Texture Array BGL"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2Array,
                    multisampled: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
        ],
    });

    let bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Texture Array BG"),
        layout: &bgl,
        entries: &[
            wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&view) },
            wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(&sampler) },
        ],
    });

    (texture, view, sampler, bgl, bg)
}