use super::block::BlockType;
use crate::render::types::Vertex;
use bevy::math::Vec3;

pub fn append_custom_block_mesh(
    block: BlockType,
    base: Vec3,
    v: &mut Vec<Vertex>,
    idx: &mut Vec<u32>,
) -> bool {
    match block {
        BlockType::Torch => {
            append_torch_mesh(base, v, idx);
            true
        }
        _ => false,
    }
}

fn append_torch_mesh(base: Vec3, v: &mut Vec<Vertex>, idx: &mut Vec<u32>) {
    let (x0, x1) = (base.x + 7.0 / 16.0, base.x + 9.0 / 16.0);
    let (y0, y1) = (base.y, base.y + 10.0 / 16.0);
    let (z0, z1) = (base.z + 7.0 / 16.0, base.z + 9.0 / 16.0);
    let t = 68.0;

    let faces: [(Vec3, [[f32; 3]; 4], [[f32; 2]; 4]); 6] = [
        // Top (+Y)
        (Vec3::Y, [[x0, y1, z1], [x1, y1, z1], [x1, y1, z0], [x0, y1, z0]], [[7.0/16.0, 8.0/16.0], [9.0/16.0, 8.0/16.0], [9.0/16.0, 6.0/16.0], [7.0/16.0, 6.0/16.0]]),
        // Bottom (-Y)
        (-Vec3::Y, [[x0, y0, z0], [x1, y0, z0], [x1, y0, z1], [x0, y0, z1]], [[7.0/16.0, 13.0/16.0], [9.0/16.0, 13.0/16.0], [9.0/16.0, 15.0/16.0], [7.0/16.0, 15.0/16.0]]),
        // North (-Z)
        (-Vec3::Z, [[x1, y0, z0], [x0, y0, z0], [x0, y1, z0], [x1, y1, z0]], [[9.0/16.0, 1.0], [7.0/16.0, 1.0], [7.0/16.0, 6.0/16.0], [9.0/16.0, 6.0/16.0]]),
        // South (+Z)
        (Vec3::Z, [[x0, y0, z1], [x1, y0, z1], [x1, y1, z1], [x0, y1, z1]], [[7.0/16.0, 1.0], [9.0/16.0, 1.0], [9.0/16.0, 6.0/16.0], [7.0/16.0, 6.0/16.0]]),
        // West (-X)
        (-Vec3::X, [[x0, y0, z0], [x0, y0, z1], [x0, y1, z1], [x0, y1, z0]], [[7.0/16.0, 1.0], [9.0/16.0, 1.0], [9.0/16.0, 6.0/16.0], [7.0/16.0, 6.0/16.0]]),
        // East (+X)
        (Vec3::X, [[x1, y0, z1], [x1, y0, z0], [x1, y1, z0], [x1, y1, z1]], [[7.0/16.0, 1.0], [9.0/16.0, 1.0], [9.0/16.0, 6.0/16.0], [7.0/16.0, 6.0/16.0]]),
    ];

    for (norm, corners, uvs) in faces {
        let base_idx = v.len() as u32;
        for i in 0..4 {
            v.push(Vertex { position: corners[i], normal: norm.to_array(), uv: uvs[i], tex_layer: t });
        }
        idx.extend_from_slice(&[base_idx, base_idx + 1, base_idx + 2, base_idx, base_idx + 2, base_idx + 3]);
    }
}
