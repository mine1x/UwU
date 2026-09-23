use bevy::math::Vec3;
use crate::engine::player_ctrl::mesh::build_player_mesh;
use crate::engine::player_ctrl::Player;
use crate::render::hud_ui::renderer::HudVertex;

pub fn draw_player_in_inventory(
    v: &mut Vec<HudVertex>,
    idx: &mut Vec<u32>,
    win_x: f32,
    win_y: f32,
    mouse_ndc: (f32, f32),
    aspect: f32,
    s: f32,
) {
    // 1. Tính góc nhìn theo con trỏ chuột:
    // Tâm khung đen preview trong GUI space: x = 51.0, y = 43.0
    let mg_x = (mouse_ndc.0 - win_x) * aspect / s;
    let mg_y = 166.0 - (mouse_ndc.1 - win_y) / s;
    let (dx, dy) = (51.0 - mg_x, 43.0 - mg_y);
    let (x_angle, y_angle) = ((dx / 40.0).atan(), (dy / 40.0).atan());

    // Trong engine gốc: Mặt trước (+Z) nhìn thẳng ra màn hình khi yaw = 0.
    // Nghiêng nhẹ ~-15 độ để nhìn rõ cả mặt trước lẫn một bên thân,
    // và xoay theo vị trí chuột.
    let base_yaw = -15.0f32.to_radians();
    let byaw = base_yaw - x_angle * 25.0f32.to_radians();
    let hyaw = base_yaw - x_angle * 45.0f32.to_radians();
    let hpitch = (-y_angle * 25.0f32.to_radians()).clamp(-0.7, 0.7);

    // 2. Tạo player ảo để gọi trực tiếp logic build_player_mesh của engine
    let mut preview_player = Player::new(0.0, 0.0, 0.0);
    preview_player.yaw = byaw;
    preview_player.head_yaw = hyaw;
    preview_player.head_pitch = hpitch;

    let (player_verts, player_indices) = build_player_mesh(&preview_player);

    // Chân của Steve đặt ở đáy ô preview (y=75 trong GUI space)
    let feet_x = win_x + 51.0 * s / aspect;
    let feet_y = win_y + (166.0 - 75.0) * s;
    let scale_factor = 28.0 * s;

    let light_dir = Vec3::new(0.5, 1.2, 0.8).normalize();
    let skin_u_offset = 512.0 / 1024.0;
    let skin_scale = 64.0 / 1024.0;

    // 3. Gom và sắp xếp các tam giác theo độ sâu Z (Painter's algorithm)
    // Người xem nhìn từ +Z nhìn về -Z. Mặt trước có Z lớn hơn (ở gần người xem hơn).
    // Nên tam giác có Z nhỏ hơn (ở xa) cần được vẽ trước, Z lớn hơn vẽ sau.
    struct Triangle {
        depth: f32,
        verts: [HudVertex; 3],
    }

    let mut triangles = Vec::with_capacity(player_indices.len() / 3);

    for chunk in player_indices.chunks_exact(3) {
        let v0 = &player_verts[chunk[0] as usize];
        let v1 = &player_verts[chunk[1] as usize];
        let v2 = &player_verts[chunk[2] as usize];

        let p0 = Vec3::from_array(v0.position);
        let p1 = Vec3::from_array(v1.position);
        let p2 = Vec3::from_array(v2.position);

        let edge1 = p1 - p0;
        let edge2 = p2 - p0;
        let face_norm = edge1.cross(edge2).normalize_or_zero();

        // Mặt quay về phía người xem phải có thành phần pháp tuyến Z > 0.
        // Cắt bỏ (back-face culling) các mặt quay ra sau lưng (normal.z <= 0).
        if face_norm.z <= 0.001 {
            continue;
        }

        let diff = face_norm.dot(light_dir).max(0.0);
        let light = 0.4 + diff * 0.6;
        let color = [light, light, light, 1.0];

        let avg_z = (p0.z + p1.z + p2.z) / 3.0;

        let hud_verts = [v0, v1, v2].map(|orig_v| {
            let wp = Vec3::from_array(orig_v.position);
            let sx = feet_x + wp.x * scale_factor / aspect;
            let sy = feet_y + wp.y * scale_factor;
            let uv = [
                skin_u_offset + orig_v.uv[0] * skin_scale,
                orig_v.uv[1] * skin_scale,
            ];
            HudVertex {
                position: [sx, sy],
                uv,
                color,
            }
        });

        triangles.push(Triangle {
            depth: avg_z,
            verts: hud_verts,
        });
    }

    // Sắp xếp vẽ từ xa (Z nhỏ) đến gần (Z lớn)
    triangles.sort_unstable_by(|a, b| a.depth.partial_cmp(&b.depth).unwrap_or(std::cmp::Ordering::Equal));

    for tri in triangles {
        let base = v.len() as u32;
        v.extend_from_slice(&tri.verts);
        idx.extend_from_slice(&[base, base + 1, base + 2]);
    }
}
