use bevy::math::Vec3;
use image::GenericImageView;

#[derive(Clone, Debug)]
pub struct ModelQuad {
    pub corners: [Vec3; 4],
    pub normal: Vec3,
    pub uvs: [[f32; 2]; 4],
}

pub fn generate_extruded_item_model(png_bytes: &[u8]) -> Vec<ModelQuad> {
    let img = match image::load_from_memory(png_bytes) { Ok(i) => i, Err(_) => return Vec::new() };
    let (w, h) = (img.width().min(16), img.height().min(16));
    let is_opaque = |x: i32, y: i32| -> bool {
        if x < 0 || y < 0 || x >= w as i32 || y >= h as i32 { return false; }
        img.get_pixel(x as u32, y as u32).0[3] > 32
    };

    let mut quads = Vec::with_capacity(160);
    let (zf, zb) = (8.5 / 16.0, 7.5 / 16.0);

    for y in 0..h as i32 {
        for x in 0..w as i32 {
            if !is_opaque(x, y) { continue; }
            let x0 = x as f32 / w as f32;
            let x1 = (x + 1) as f32 / w as f32;
            let y0 = (h as i32 - 1 - y) as f32 / h as f32;
            let y1 = (h as i32 - y) as f32 / h as f32;
            let (u0, u1) = (x as f32 / w as f32, (x + 1) as f32 / w as f32);
            let (v0, v1) = (y as f32 / h as f32, (y + 1) as f32 / h as f32);
            let (uc, vc) = ((x as f32 + 0.5) / w as f32, (y as f32 + 0.5) / h as f32);

            // Front (+Z) & Back (-Z)
            quads.push(ModelQuad {
                corners: [Vec3::new(x0, y0, zf), Vec3::new(x1, y0, zf), Vec3::new(x1, y1, zf), Vec3::new(x0, y1, zf)],
                normal: Vec3::Z,
                uvs: [[u0, v1], [u1, v1], [u1, v0], [u0, v0]],
            });
            quads.push(ModelQuad {
                corners: [Vec3::new(x1, y0, zb), Vec3::new(x0, y0, zb), Vec3::new(x0, y1, zb), Vec3::new(x1, y1, zb)],
                normal: -Vec3::Z,
                uvs: [[u1, v1], [u0, v1], [u0, v0], [u1, v0]],
            });

            // Side edges (solid color at pixel center, zero bleeding)
            let side_uv = [[uc, vc]; 4];
            if !is_opaque(x, y - 1) { // UP
                quads.push(ModelQuad {
                    corners: [Vec3::new(x0, y1, zf), Vec3::new(x1, y1, zf), Vec3::new(x1, y1, zb), Vec3::new(x0, y1, zb)],
                    normal: Vec3::Y, uvs: side_uv,
                });
            }
            if !is_opaque(x, y + 1) { // DOWN
                quads.push(ModelQuad {
                    corners: [Vec3::new(x0, y0, zb), Vec3::new(x1, y0, zb), Vec3::new(x1, y0, zf), Vec3::new(x0, y0, zf)],
                    normal: -Vec3::Y, uvs: side_uv,
                });
            }
            if !is_opaque(x - 1, y) { // LEFT
                quads.push(ModelQuad {
                    corners: [Vec3::new(x0, y0, zb), Vec3::new(x0, y0, zf), Vec3::new(x0, y1, zf), Vec3::new(x0, y1, zb)],
                    normal: -Vec3::X, uvs: side_uv,
                });
            }
            if !is_opaque(x + 1, y) { // RIGHT
                quads.push(ModelQuad {
                    corners: [Vec3::new(x1, y0, zf), Vec3::new(x1, y0, zb), Vec3::new(x1, y1, zb), Vec3::new(x1, y1, zf)],
                    normal: Vec3::X, uvs: side_uv,
                });
            }
        }
    }
    quads
}
