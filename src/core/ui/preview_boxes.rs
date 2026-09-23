use bevy::math::Vec3;

#[derive(Clone, Copy)]
pub struct PreviewBox {
    pub origin: Vec3,
    pub size: Vec3,
    pub uvs: [[[f32; 4]; 6]; 1],
}

impl PreviewBox {
    pub fn new(origin: Vec3, size: Vec3, u: f32, v: f32) -> Self {
        let (w, h, d) = (size.x, size.y, size.z);
        let uv_face = |x0: f32, y0: f32, x1: f32, y1: f32| [x0, y0, x1, y1];

        // Java Box UV layout:
        // Top: [u + d, v, u + d + w, v + d]
        // Bottom: [u + d + w, v, u + d + 2w, v + d]
        // Front (+Z): [u + d, v + d, u + d + w, v + d + h]
        // Back (-Z): [u + 2d + w, v + d, u + 2d + 2w, v + d + h]
        // Right (+X): [u, v + d, u + d, v + d + h]
        // Left (-X): [u + d + w, v + d, u + 2d + w, v + d + h]
        let faces = [
            uv_face(u + d, v, u + d + w, v + d),                     // +Y Top
            uv_face(u + d + w, v, u + d + 2.0 * w, v + d),          // -Y Bottom
            uv_face(u + d, v + d, u + d + w, v + d + h),             // +Z Front
            uv_face(u + 2.0 * d + w, v + d, u + 2.0 * (d + w), v + d + h), // -Z Back
            uv_face(u, v + d, u + d, v + d + h),                     // +X Right
            uv_face(u + d + w, v + d, u + 2.0 * d + w, v + d + h),   // -X Left
        ];

        Self {
            origin,
            size,
            uvs: [faces],
        }
    }
}

pub fn get_steve_preview_boxes() -> [PreviewBox; 6] {
    [
        // Left leg
        PreviewBox::new(Vec3::new(-1.9, 12.0, 0.0), Vec3::new(4.0, 12.0, 4.0), 16.0, 48.0),
        // Right leg
        PreviewBox::new(Vec3::new(1.9, 12.0, 0.0), Vec3::new(4.0, 12.0, 4.0), 0.0, 16.0),
        // Torso
        PreviewBox::new(Vec3::new(0.0, 24.0, 0.0), Vec3::new(8.0, 12.0, 4.0), 16.0, 16.0),
        // Left arm
        PreviewBox::new(Vec3::new(-5.0, 24.0, 0.0), Vec3::new(4.0, 12.0, 4.0), 32.0, 48.0),
        // Right arm
        PreviewBox::new(Vec3::new(5.0, 24.0, 0.0), Vec3::new(4.0, 12.0, 4.0), 40.0, 16.0),
        // Head
        PreviewBox::new(Vec3::new(0.0, 28.0, 0.0), Vec3::new(8.0, 8.0, 8.0), 0.0, 0.0),
    ]
}
