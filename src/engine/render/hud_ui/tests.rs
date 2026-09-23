#[cfg(test)]
mod tests {
    use crate::engine::creative_palette::get_creative_palette;
    use crate::engine::items::ItemType;
    use crate::render::hud_ui::atlas::HudAtlas;
    use crate::render::hud_ui::item_icon::get_item_sprite;
    use crate::world::block::BlockType;

    #[test]
    fn test_tools_distinct_sprites() {
        let grass_rect = HudAtlas::rect(0, 320, 64, 64);
        let tools = [
            ItemType::WoodenSword, ItemType::StoneSword, ItemType::IronSword, ItemType::DiamondSword,
            ItemType::WoodenPickaxe, ItemType::StonePickaxe, ItemType::IronPickaxe, ItemType::DiamondPickaxe,
            ItemType::WoodenAxe, ItemType::StoneAxe, ItemType::IronAxe, ItemType::DiamondAxe,
            ItemType::WoodenShovel, ItemType::StoneShovel, ItemType::IronShovel, ItemType::DiamondShovel,
            ItemType::WoodenHoe, ItemType::StoneHoe, ItemType::IronHoe, ItemType::DiamondHoe,
        ];
        for t in tools {
            let r = get_item_sprite(&t);
            assert_ne!((r.u0, r.v0), (grass_rect.u0, grass_rect.v0), "Tool {:?} should not have grass sprite", t);
        }
    }

    #[test]
    fn test_blocks_and_materials_sprites() {
        let grass_rect = HudAtlas::rect(0, 320, 64, 64);
        let non_grass_items = [
            ItemType::Block(BlockType::Cobblestone),
            ItemType::Block(BlockType::CraftingTable),
            ItemType::Block(BlockType::Furnace),
            ItemType::Block(BlockType::Chest),
            ItemType::Block(BlockType::Torch),
            ItemType::Block(BlockType::IronBlock),
            ItemType::Block(BlockType::CoalBlock),
            ItemType::Block(BlockType::DiamondBlock),
            ItemType::Coal,
            ItemType::Stick,
            ItemType::IronIngot,
            ItemType::Diamond,
            ItemType::Porkchop,
            ItemType::CookedPorkchop,
            ItemType::Beef,
            ItemType::CookedBeef,
            ItemType::Mutton,
            ItemType::CookedMutton,
            ItemType::WhiteWool,
            ItemType::Leather,
        ];
        for item in non_grass_items {
            let r = get_item_sprite(&item);
            assert_ne!((r.u0, r.v0), (grass_rect.u0, grass_rect.v0), "Item {:?} fell back to grass!", item);
        }
    }

    #[test]
    fn test_creative_palette_no_unintended_grass() {
        let palette = get_creative_palette();
        let grass_rect = HudAtlas::rect(0, 320, 64, 64);
        for item in palette {
            let r = get_item_sprite(&item);
            if item != ItemType::Block(BlockType::Grass) {
                assert_ne!((r.u0, r.v0), (grass_rect.u0, grass_rect.v0), "Palette item {:?} is displaying as grass block!", item);
            }
        }
    }
}
