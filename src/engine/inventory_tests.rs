#[cfg(test)]
mod tests {
    use bevy::math::Vec3;
    use crate::engine::inventory::Inventory;
    use crate::engine::items::{ItemStack, ItemType};
    use crate::core::ui::get_steve_preview_boxes;
    use crate::core::ui::get_slot_pos;
    use crate::world::block::BlockType;

    #[test]
    fn test_46_slot_coordinates_java_parity() {
        for idx in 0..46 {
            assert!(get_slot_pos(idx).is_some(), "Slot {} should have coordinates", idx);
        }
        assert!(get_slot_pos(46).is_none());

        assert_eq!(get_slot_pos(0), Some((8.0, 84.0)));
        assert_eq!(get_slot_pos(27), Some((8.0, 142.0)));
        assert_eq!(get_slot_pos(36), Some((8.0, 8.0)));
        assert_eq!(get_slot_pos(37), Some((8.0, 26.0)));
        assert_eq!(get_slot_pos(38), Some((8.0, 44.0)));
        assert_eq!(get_slot_pos(39), Some((8.0, 62.0)));
        assert_eq!(get_slot_pos(40), Some((77.0, 62.0)));
        assert_eq!(get_slot_pos(41), Some((98.0, 18.0)));
        assert_eq!(get_slot_pos(45), Some((154.0, 28.0)));
    }

    #[test]
    fn test_crafting_and_result_pickup() {
        let mut inv = Inventory::new();
        inv.carried_item = Some(ItemStack::new(ItemType::Block(BlockType::OakLog), 1));
        inv.click_slot(41, false, false);
        assert!(inv.carried_item.is_none());
        assert_eq!(inv.craft[0].as_ref().unwrap().item, ItemType::Block(BlockType::OakLog));
        assert_eq!(inv.result.as_ref().unwrap().item, ItemType::Block(BlockType::OakPlanks));
        assert_eq!(inv.result.as_ref().unwrap().count, 4);

        inv.click_slot(45, false, false);
        assert_eq!(inv.carried_item.as_ref().unwrap().item, ItemType::Block(BlockType::OakPlanks));
        assert_eq!(inv.carried_item.as_ref().unwrap().count, 4);
        assert!(inv.craft[0].is_none());
        assert!(inv.result.is_none());
    }

    #[test]
    fn test_steve_preview_boxes_geometry() {
        let boxes = get_steve_preview_boxes();
        assert_eq!(boxes.len(), 6);
        assert_eq!(boxes[5].size, Vec3::new(8.0, 8.0, 8.0));
        assert_eq!(boxes[5].origin, Vec3::new(0.0, 28.0, 0.0));
        assert_eq!(boxes[2].size, Vec3::new(8.0, 12.0, 4.0));
    }

    #[test]
    fn test_crouch_player_mesh_and_item_attachment() {
        let mut p = crate::engine::Player::new(0.0, 0.0, 0.0);
        p.held_item = Some(ItemType::DiamondSword);
        let (verts, indices) = p.mesh();
        assert_eq!(verts.len(), 1136);
        assert_eq!(indices.len(), 216 + 1488);
        let sword_v = &verts[144..];
        let (min_x, max_x) = sword_v.iter().fold((f32::MAX, f32::MIN), |(mn, mx), v| (mn.min(v.position[0]), mx.max(v.position[0])));
        let sword_center_x = (min_x + max_x) * 0.5;
        assert!((sword_center_x - -0.375).abs() < 0.05, "Sword X center at right hand center, got {}", sword_center_x);

        p.held_item = Some(ItemType::Apple);
        let (verts_apple, _) = p.mesh();
        let apple_v = &verts_apple[144..];
        let (apple_min_x, apple_max_x) = apple_v.iter().fold((f32::MAX, f32::MIN), |(mn, mx), v| (mn.min(v.position[0]), mx.max(v.position[0])));
        let apple_center_x = (apple_min_x + apple_max_x) * 0.5;
        assert!((apple_center_x - -0.375).abs() < 0.05, "Apple X center at right hand center, got {}", apple_center_x);
    }
}
