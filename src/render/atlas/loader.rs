use image::GenericImageView;

pub const RESOLUTION: u32 = 64;

pub fn load_atlas_layer(bytes: &[u8], tint: Option<[f32; 3]>, fallback: [u8; 4]) -> Vec<u8> {
    let res = RESOLUTION as usize;
    if let Ok(img) = image::load_from_memory(bytes) {
        let mut out = Vec::with_capacity(res * res * 4);
        let (iw, ih) = (img.width(), img.height());
        for y in 0..RESOLUTION {
            for x in 0..RESOLUTION {
                let p = img.get_pixel((x * iw) / RESOLUTION, (y * ih) / RESOLUTION);
                let (r, g, b, a) = if p.0[3] <= 1 {
                    (0, 0, 0, 0)
                } else {
                    let (cr, cg, cb) = match tint {
                        Some(t) => (
                            ((p.0[0] as f32 / 255.0) * t[0] * 255.0).clamp(0.0, 255.0) as u8,
                            ((p.0[1] as f32 / 255.0) * t[1] * 255.0).clamp(0.0, 255.0) as u8,
                            ((p.0[2] as f32 / 255.0) * t[2] * 255.0).clamp(0.0, 255.0) as u8,
                        ),
                        None => (p.0[0], p.0[1], p.0[2]),
                    };
                    (cr, cg, cb, p.0[3])
                };
                out.extend_from_slice(&[r, g, b, a]);
            }
        }
        out
    } else {
        vec![fallback; res * res].into_iter().flatten().collect()
    }
}
