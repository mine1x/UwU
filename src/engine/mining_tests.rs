#[cfg(test)]
mod tests {
    use bevy::math::Vec3;
    use crate::engine::items::ItemType;
    use crate::engine::mining::MiningState;
    use crate::render::overlays::build_destroy_overlay_mesh;
    use crate::world::block::BlockType;

    #[test]
    fn test_java_parity_block_breaking_speeds() {
        // Dirt by hand: hardness 0.5, speed 1.0, modifier 30.0 -> 0.75s
        let mut mining = MiningState::new();
        mining.set_mining(Some((0, 0, 0)), true);
        assert!(!mining.update(0.74, BlockType::Dirt, None));
        assert!(mining.update(0.02, BlockType::Dirt, None));

        // Grass by hand: hardness 0.6, speed 1.0, modifier 30.0 -> 0.9s
        mining.set_mining(Some((0, 0, 0)), true);
        assert!(!mining.update(0.89, BlockType::Grass, None));
        assert!(mining.update(0.02, BlockType::Grass, None));

        // Wood by hand: hardness 2.0, speed 1.0, modifier 30.0 -> 3.0s
        mining.set_mining(Some((0, 0, 0)), true);
        assert!(!mining.update(2.99, BlockType::Wood, None));
        assert!(mining.update(0.02, BlockType::Wood, None));

        // Leaves by hand: hardness 0.2, speed 1.0, modifier 30.0 -> 0.3s
        mining.set_mining(Some((0, 0, 0)), true);
        assert!(!mining.update(0.29, BlockType::OakLeaves, None));
        assert!(mining.update(0.02, BlockType::OakLeaves, None));

        // Leaves with sword: speed 1.5 -> 0.2s
        mining.set_mining(Some((0, 0, 0)), true);
        assert!(!mining.update(0.19, BlockType::OakLeaves, Some(ItemType::DiamondSword)));
        assert!(mining.update(0.02, BlockType::OakLeaves, Some(ItemType::DiamondSword)));
    }

    #[test]
    fn test_java_destroy_stages_and_overlay_mesh() {
        let mut mining = MiningState::new();
        assert_eq!(mining.get_stage(), None);

        mining.set_mining(Some((5, 10, 15)), true);
        assert_eq!(mining.get_stage(), None);

        mining.progress = 0.05;
        assert_eq!(mining.get_stage(), Some(0));

        mining.progress = 0.45;
        assert_eq!(mining.get_stage(), Some(4));

        mining.progress = 0.95;
        assert_eq!(mining.get_stage(), Some(9));

        // Test overlay mesh building for stage 4
        let cam_offset = Vec3::new(10.0, 10.0, 10.0);
        let (verts, indices) = build_destroy_overlay_mesh(5, 10, 15, 4, 0b111111, cam_offset);
        assert!(!verts.is_empty());
        assert!(!indices.is_empty());
        // Layer must be 18 + 4 = 22.0
        assert_eq!(verts[0].tex_layer, 22.0);

        // Verify vertex bounds around block (5, 10, 15)
        for v in &verts {
            assert!(v.position[0] >= 4.99 && v.position[0] <= 6.01);
            assert!(v.position[1] >= 9.99 && v.position[1] <= 11.01);
            assert!(v.position[2] >= 14.99 && v.position[2] <= 16.01);
        }
    }
}
