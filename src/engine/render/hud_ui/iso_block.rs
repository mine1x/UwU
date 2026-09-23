use image::GenericImageView;

pub fn render_isometric_block(
    atlas: &mut [u8],
    top_bytes: &[u8],
    side_bytes: &[u8],
    tint: Option<[f32; 3]>,
    dst_x: u32,
    dst_y: u32,
    atlas_w: u32,
) {
    let top = match image::load_from_memory(top_bytes) { Ok(i) => i, Err(_) => return };
    let side = match image::load_from_memory(side_bytes) { Ok(i) => i, Err(_) => return };
    let (tw, th) = (top.width() as f32, top.height() as f32);
    let (sw, sh) = (side.width() as f32, side.height() as f32);

    let mut set_px = |x: u32, y: u32, r: u8, g: u8, b: u8, a: u8| {
        let (ax, ay) = (dst_x + x, dst_y + y);
        let idx = ((ay * atlas_w + ax) * 4) as usize;
        if idx + 4 <= atlas.len() && a > 16 { atlas[idx..idx + 4].copy_from_slice(&[r, g, b, a]); }
    };

    let (cx, cy) = (31.5f32, 27.5f32);
    let (w, r, h_side) = (24.0f32, 12.0f32, 28.0f32);

    // 1. Top face (diamond): affine mapping u=(dv+du)/2, v=(dv-du)/2
    for y in 2..=27 {
        for x in 6..=57 {
            let du = (x as f32 + 0.5 - cx) / w;
            let dv = (y as f32 + 0.5 - (cy - 2.0 * r)) / r;
            let (u, v) = ((dv + du) * 0.5, (dv - du) * 0.5);
            if u >= 0.0 && u < 1.0 && v >= 0.0 && v < 1.0 {
                let p = top.get_pixel((u * (tw - 0.001)) as u32, (v * (th - 0.001)) as u32);
                let (r_col, g_col, b_col) = match tint {
                    Some(t) => (
                        ((p.0[0] as f32 / 255.0) * t[0] * 255.0) as u8,
                        ((p.0[1] as f32 / 255.0) * t[1] * 255.0) as u8,
                        ((p.0[2] as f32 / 255.0) * t[2] * 255.0) as u8,
                    ),
                    None => (p.0[0], p.0[1], p.0[2]),
                };
                set_px(x, y, r_col, g_col, b_col, p.0[3]);
            }
        }
    }

    // 2. Left face (shaded 0.60)
    for x in 6..=31 {
        let u = (x as f32 + 0.5 - (cx - w)) / w;
        let y_top = (cy - r) + u * r;
        for y in y_top.floor() as u32..=(y_top + h_side).ceil() as u32 {
            let v = (y as f32 + 0.5 - y_top) / h_side;
            if u >= 0.0 && u < 1.0 && v >= 0.0 && v < 1.0 && y <= 63 {
                let p = side.get_pixel((u * (sw - 0.001)) as u32, (v * (sh - 0.001)) as u32);
                set_px(x, y, (p.0[0] as f32 * 0.60) as u8, (p.0[1] as f32 * 0.60) as u8, (p.0[2] as f32 * 0.60) as u8, p.0[3]);
            }
        }
    }

    // 3. Right face (shaded 0.80)
    for x in 32..=57 {
        let u = (x as f32 + 0.5 - cx) / w;
        let y_top = cy - u * r;
        for y in y_top.floor() as u32..=(y_top + h_side).ceil() as u32 {
            let v = (y as f32 + 0.5 - y_top) / h_side;
            if u >= 0.0 && u < 1.0 && v >= 0.0 && v < 1.0 && y <= 63 {
                let p = side.get_pixel((u * (sw - 0.001)) as u32, (v * (sh - 0.001)) as u32);
                set_px(x, y, (p.0[0] as f32 * 0.80) as u8, (p.0[1] as f32 * 0.80) as u8, (p.0[2] as f32 * 0.80) as u8, p.0[3]);
            }
        }
    }
}
