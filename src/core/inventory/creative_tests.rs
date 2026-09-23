#[cfg(test)]
mod tests {
    use crate::engine::creative_palette::get_creative_palette;
    use crate::engine::Inventory;
    use crate::core::inventory::handle_creative_slot_click;
    use crate::core::inventory::CreativeSlotAction;

    #[test]
    fn test_creative_palette_count_and_items() {
        let p = get_creative_palette();
        assert_eq!(p.len(), 45);
    }

    #[test]
    fn test_creative_slot_clicks() {
        let mut inv = Inventory::new();
        assert!(inv.carried_item.is_none());

        // 1. Click slot 0 in palette (Grass)
        handle_creative_slot_click(&mut inv, CreativeSlotAction::Palette(0), false, false);
        assert!(inv.carried_item.is_some());
        assert_eq!(inv.carried_item.unwrap().count, 64);

        // 2. Click hotbar slot 0 to place carried item
        handle_creative_slot_click(&mut inv, CreativeSlotAction::Hotbar(0), false, false);
        assert_eq!(inv.hotbar[0].unwrap().count, 64);

        // 3. Click Destroy slot to trash carried item
        inv.carried_item = Some(crate::engine::items::ItemStack::new(crate::engine::items::ItemType::Apple, 10));
        handle_creative_slot_click(&mut inv, CreativeSlotAction::Destroy, false, false);
        assert!(inv.carried_item.is_none());
    }
}
