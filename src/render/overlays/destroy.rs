use bevy::math::Vec3;
use crate::render::types::Vertex;

pub fn build_destroy_overlay_mesh(
    bx: i32,
    by: i32,
    bz: i32,
    stage: usize,
    exposed_faces: u8,
    cam_offset: Vec3,
) -> (Vec<Vertex>, Vec<u32>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let stage = stage.min(9);
    let layer = (18 + stage) as f32;
    let base = Vec3::new(bx as f32, by as f32, bz as f32);
    let d = 0.002;

    let faces: [(Vec3, [Vec3; 4], u8); 6] = [
        (-Vec3::X, [Vec3::new(-d, 0.0, 0.0), Vec3::new(-d, 0.0, 1.0), Vec3::new(-d, 1.0, 1.0), Vec3::new(-d, 1.0, 0.0)], 1 << 0),
        (Vec3::X, [Vec3::new(1.0 + d, 0.0, 1.0), Vec3::new(1.0 + d, 0.0, 0.0), Vec3::new(1.0 + d, 1.0, 0.0), Vec3::new(1.0 + d, 1.0, 1.0)], 1 << 1),
        (-Vec3::Y, [Vec3::new(0.0, -d, 0.0), Vec3::new(1.0, -d, 0.0), Vec3::new(1.0, -d, 1.0), Vec3::new(0.0, -d, 1.0)], 1 << 2),
        (Vec3::Y, [Vec3::new(0.0, 1.0 + d, 1.0), Vec3::new(1.0, 1.0 + d, 1.0), Vec3::new(1.0, 1.0 + d, 0.0), Vec3::new(0.0, 1.0 + d, 0.0)], 1 << 3),
        (-Vec3::Z, [Vec3::new(1.0, 0.0, -d), Vec3::new(0.0, 0.0, -d), Vec3::new(0.0, 1.0, -d), Vec3::new(1.0, 1.0, -d)], 1 << 4),
        (Vec3::Z, [Vec3::new(0.0, 0.0, 1.0 + d), Vec3::new(1.0, 0.0, 1.0 + d), Vec3::new(1.0, 1.0, 1.0 + d), Vec3::new(0.0, 1.0, 1.0 + d)], 1 << 5),
    ];

    let uvs = [[0.0, 1.0], [1.0, 1.0], [1.0, 0.0], [0.0, 0.0]];

    for (normal, corners, mask) in faces {
        if (exposed_faces & mask) == 0 || normal.dot(cam_offset) <= 0.0 {
            continue;
        }

        let s = vertices.len() as u32;
        for (i, &corner) in corners.iter().enumerate() {
            vertices.push(Vertex {
                position: (base + corner).to_array(),
                normal: normal.to_array(),
                uv: uvs[i],
                tex_layer: layer,
            });
        }
        indices.extend_from_slice(&[s, s + 1, s + 2, s, s + 2, s + 3]);
    }

    (vertices, indices)
}
