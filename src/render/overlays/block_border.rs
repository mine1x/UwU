use bevy::math::Vec3;
use crate::render::types::Vertex;

/// Builds hover bounding outline for a block, rendering only edges on exposed faces that are visible to the camera.
pub fn build_hover_overlay_mesh(
    bx: i32,
    by: i32,
    bz: i32,
    exposed_faces: u8,
    cam_offset: Vec3,
) -> (Vec<Vertex>, Vec<u32>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    let base = Vec3::new(bx as f32, by as f32, bz as f32);
    let min_off = -0.005;
    let max_off = 1.005;
    let thick = 0.04;

    // Face masks (Java parity order: -X, +X, -Y, +Y, -Z, +Z)
    // 0: -X (1 << 0)
    // 1: +X (1 << 1)
    // 2: -Y (1 << 2)
    // 3: +Y (1 << 3)
    // 4: -Z (1 << 4)
    // 5: +Z (1 << 5)
    let face_normals = [
        (-Vec3::X, 1 << 0),
        (Vec3::X, 1 << 1),
        (-Vec3::Y, 1 << 2),
        (Vec3::Y, 1 << 3),
        (-Vec3::Z, 1 << 4),
        (Vec3::Z, 1 << 5),
    ];

    let mut visible_face_mask = 0u8;
    for (norm, mask) in face_normals {
        if (exposed_faces & mask) != 0 && norm.dot(cam_offset) > 0.0 {
            visible_face_mask |= mask;
        }
    }

    // If no faces visible with dot product, fallback to exposed faces (or all if 0)
    if visible_face_mask == 0 {
        visible_face_mask = if exposed_faces != 0 { exposed_faces } else { 0b111111 };
    }

    // 12 edges with their 2 adjacent faces:
    // Bottom 4 edges (Y = min_off)
    // 0: X-edge at Z=min_off -> bottom (-Y, bit 2) and front (-Z, bit 4)
    // 1: X-edge at Z=max_off -> bottom (-Y, bit 2) and back (+Z, bit 5)
    // 2: Z-edge at X=min_off -> bottom (-Y, bit 2) and left (-X, bit 0)
    // 3: Z-edge at X=max_off -> bottom (-Y, bit 2) and right (+X, bit 1)
    // Top 4 edges (Y = max_off)
    // 4: X-edge at Z=min_off -> top (+Y, bit 3) and front (-Z, bit 4)
    // 5: X-edge at Z=max_off -> top (+Y, bit 3) and back (+Z, bit 5)
    // 6: Z-edge at X=min_off -> top (+Y, bit 3) and left (-X, bit 0)
    // 7: Z-edge at X=max_off -> top (+Y, bit 3) and right (+X, bit 1)
    // 4 Vertical Pillars
    // 8: Y-edge at X=min_off, Z=min_off -> left (-X, bit 0) and front (-Z, bit 4)
    // 9: Y-edge at X=max_off, Z=min_off -> right (+X, bit 1) and front (-Z, bit 4)
    // 10: Y-edge at X=min_off, Z=max_off -> left (-X, bit 0) and back (+Z, bit 5)
    // 11: Y-edge at X=max_off, Z=max_off -> right (+X, bit 1) and back (+Z, bit 5)
    let edges: [((Vec3, Vec3), u8); 12] = [
        // Bottom 4 edges
        ((Vec3::new(min_off, min_off, min_off), Vec3::new(max_off, min_off + thick, min_off + thick)), (1 << 2) | (1 << 4)),
        ((Vec3::new(min_off, min_off, max_off - thick), Vec3::new(max_off, min_off + thick, max_off)), (1 << 2) | (1 << 5)),
        ((Vec3::new(min_off, min_off, min_off), Vec3::new(min_off + thick, min_off + thick, max_off)), (1 << 2) | (1 << 0)),
        ((Vec3::new(max_off - thick, min_off, min_off), Vec3::new(max_off, min_off + thick, max_off)), (1 << 2) | (1 << 1)),
        // Top 4 edges
        ((Vec3::new(min_off, max_off - thick, min_off), Vec3::new(max_off, max_off, min_off + thick)), (1 << 3) | (1 << 4)),
        ((Vec3::new(min_off, max_off - thick, max_off - thick), Vec3::new(max_off, max_off, max_off)), (1 << 3) | (1 << 5)),
        ((Vec3::new(min_off, max_off - thick, min_off), Vec3::new(min_off + thick, max_off, max_off)), (1 << 3) | (1 << 0)),
        ((Vec3::new(max_off - thick, max_off - thick, min_off), Vec3::new(max_off, max_off, max_off)), (1 << 3) | (1 << 1)),
        // 4 Vertical Pillars
        ((Vec3::new(min_off, min_off, min_off), Vec3::new(min_off + thick, max_off, min_off + thick)), (1 << 0) | (1 << 4)),
        ((Vec3::new(max_off - thick, min_off, min_off), Vec3::new(max_off, max_off, min_off + thick)), (1 << 1) | (1 << 4)),
        ((Vec3::new(min_off, min_off, max_off - thick), Vec3::new(min_off + thick, max_off, max_off)), (1 << 0) | (1 << 5)),
        ((Vec3::new(max_off - thick, min_off, max_off - thick), Vec3::new(max_off, max_off, max_off)), (1 << 1) | (1 << 5)),
    ];

    let face_indices = [
        (0, 3, 2, 1, -Vec3::Z),
        (4, 5, 6, 7, Vec3::Z),
        (0, 1, 5, 4, -Vec3::Y),
        (3, 7, 6, 2, Vec3::Y),
        (0, 4, 7, 3, -Vec3::X),
        (1, 2, 6, 5, Vec3::X),
    ];

    for ((p0_rel, p1_rel), edge_faces) in edges {
        // Only render the edge if at least one of its adjacent faces is exposed and facing camera
        if (edge_faces & visible_face_mask) == 0 {
            continue;
        }

        let p0 = base + p0_rel;
        let p1 = base + p1_rel;

        let corners = [
            Vec3::new(p0.x, p0.y, p0.z),
            Vec3::new(p1.x, p0.y, p0.z),
            Vec3::new(p1.x, p1.y, p0.z),
            Vec3::new(p0.x, p1.y, p0.z),
            Vec3::new(p0.x, p0.y, p1.z),
            Vec3::new(p1.x, p0.y, p1.z),
            Vec3::new(p1.x, p1.y, p1.z),
            Vec3::new(p0.x, p1.y, p1.z),
        ];

        for (i0, i1, i2, i3, norm) in face_indices {
            let s = vertices.len() as u32;
            for &idx in &[i0, i1, i2, i3] {
                vertices.push(Vertex {
                    position: corners[idx].to_array(),
                    normal: norm.to_array(),
                    uv: [0.0, 0.0],
                    tex_layer: -1.0,
                });
            }
            indices.extend_from_slice(&[s, s + 1, s + 2, s, s + 2, s + 3]);
        }
    }

    (vertices, indices)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hover_overlay_face_culling() {
        let cam_offset = Vec3::new(10.0, 10.0, 10.0);
        // Only top (+Y) face exposed
        let exposed_top_only = 1 << 3;
        let (verts_top, _) = build_hover_overlay_mesh(0, 0, 0, exposed_top_only, cam_offset);

        // All 6 faces exposed
        let exposed_all = 0b111111;
        let (verts_all, _) = build_hover_overlay_mesh(0, 0, 0, exposed_all, cam_offset);

        // The top-only exposed block should render significantly fewer vertices than all-exposed
        assert!(verts_top.len() > 0);
        assert!(verts_top.len() < verts_all.len());
        // For top face, only 4 edges are adjacent to +Y (edges 4, 5, 6, 7). 4 edges * 6 quads * 4 verts = 96 verts
        assert_eq!(verts_top.len(), 4 * 6 * 4);
    }
}