use image::GenericImageView;

pub struct WaterAnimator {
    still_frames: Vec<Vec<u8>>,
    flow_frames: Vec<Vec<u8>>,
}

impl WaterAnimator {
    pub fn new() -> Self {
        let water_tint = [0.247f32, 0.463f32, 0.894f32];
        let w_still = include_bytes!("../../assets/textures/block/water_still.png");
        let w_flow = include_bytes!("../../assets/textures/block/water_flow.png");

        let still_frames = Self::load_frames(w_still, 16, 16, water_tint);
        let flow_frames = Self::load_frames(w_flow, 32, 32, water_tint);

        Self { still_frames, flow_frames }
    }

    fn load_frames(bytes: &[u8], fw: u32, fh: u32, tint: [f32; 3]) -> Vec<Vec<u8>> {
        let mut frames = Vec::new();
        if let Ok(img) = image::load_from_memory(bytes) {
            let frame_count = (img.height() / fh).max(1);
            for f in 0..frame_count {
                let mut buf = vec![0u8; 64 * 64 * 4];
                let y_off = f * fh;
                for y in 0..64 {
                    for x in 0..64 {
                        let sx = (x * fw) / 64;
                        let sy = y_off + (y * fh) / 64;
                        let p = img.get_pixel(sx % img.width(), sy % img.height());
                        let r = ((p.0[0] as f32 / 255.0) * tint[0] * 255.0).clamp(0.0, 255.0) as u8;
                        let g = ((p.0[1] as f32 / 255.0) * tint[1] * 255.0).clamp(0.0, 255.0) as u8;
                        let b = ((p.0[2] as f32 / 255.0) * tint[2] * 255.0).clamp(0.0, 255.0) as u8;
                        let idx = ((y * 64 + x) * 4) as usize;
                        buf[idx..idx + 4].copy_from_slice(&[r, g, b, p.0[3]]);
                    }
                }
                frames.push(buf);
            }
        }
        if frames.is_empty() {
            frames.push(vec![0u8; 64 * 64 * 4]);
        }
        frames
    }

    pub fn get_frames(&self, t: f32) -> (&[u8], &[u8]) {
        let sf_idx = ((t * 20.0) as usize) % self.still_frames.len();
        let ff_idx = ((t * 20.0) as usize) % self.flow_frames.len();
        (&self.still_frames[sf_idx], &self.flow_frames[ff_idx])
    }

    pub fn update_water_texture(&self, queue: &wgpu::Queue, texture: &wgpu::Texture, t: f32) {
        let (sf, ff) = self.get_frames(t);
        let layout = wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(256), rows_per_image: Some(64) };
        let size = wgpu::Extent3d { width: 64, height: 64, depth_or_array_layers: 1 };
        queue.write_texture(
            wgpu::TexelCopyTextureInfo { texture, mip_level: 0, origin: wgpu::Origin3d { x: 0, y: 0, z: 4 }, aspect: wgpu::TextureAspect::All },
            sf, layout, size,
        );
        queue.write_texture(
            wgpu::TexelCopyTextureInfo { texture, mip_level: 0, origin: wgpu::Origin3d { x: 0, y: 0, z: 5 }, aspect: wgpu::TextureAspect::All },
            ff, layout, size,
        );
    }
}