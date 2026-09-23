#[cfg(test)]
mod tests {
    use bevy::math::Vec3;
    use crate::engine::items::ItemType;
    use crate::world::block::BlockType;
    use super::super::model_cache::get_item_model;

    #[test]
    fn test_extruded_item_model_generation() {
        let sword_quads = get_item_model(ItemType::DiamondSword);
        assert!(!sword_quads.is_empty());
        assert!(sword_quads.iter().any(|q| q.normal == Vec3::Z));
        assert!(sword_quads.iter().any(|q| q.normal == -Vec3::Z));
        assert!(sword_quads.iter().any(|q| q.normal == Vec3::Y));
        assert!(sword_quads.iter().any(|q| q.normal == -Vec3::Y));
        assert!(sword_quads.iter().any(|q| q.normal == Vec3::X));
        assert!(sword_quads.iter().any(|q| q.normal == -Vec3::X));
    }

    #[test]
    fn test_wood_and_leaves_blocks() {
        assert!(BlockType::OakLog.is_solid());
        assert!(!BlockType::OakLog.is_transparent());
        assert_eq!(BlockType::OakLog.tex_layer(), 14.0);

        assert!(BlockType::OakPlanks.is_solid());
        assert!(!BlockType::OakPlanks.is_transparent());
        assert_eq!(BlockType::OakPlanks.tex_layer(), 2.0);

        assert!(BlockType::OakLeaves.is_solid());
        assert!(BlockType::OakLeaves.is_transparent());
        assert_eq!(BlockType::OakLeaves.tex_layer(), 17.0);
    }
}
