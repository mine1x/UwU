use super::creative_hit::CreativeSlotAction;
use crate::engine::creative_palette::get_creative_palette;
use crate::engine::items::ItemStack;
use crate::engine::Inventory;

pub fn handle_creative_slot_click(inv: &mut Inventory, action: CreativeSlotAction, right: bool, shift: bool) {
    let palette = get_creative_palette();
    match action {
        CreativeSlotAction::Palette(idx) => {
            if idx < palette.len() {
                let item = palette[idx];
                let count = if item.is_tool() { 1 } else { 64 };
                let stack = ItemStack::new(item, count);
                if shift {
                    inv.add_stack(&stack);
                } else {
                    inv.carried_item = Some(stack);
                }
            }
        }
        CreativeSlotAction::Hotbar(slot) => {
            if slot < 9 {
                if let Some(carried) = inv.carried_item.take() {
                    let old_slot = inv.hotbar[slot].take();
                    inv.hotbar[slot] = Some(carried);
                    inv.carried_item = old_slot;
                } else if let Some(item) = inv.hotbar[slot] {
                    if right {
                        inv.carried_item = Some(item);
                    } else {
                        inv.carried_item = inv.hotbar[slot].take();
                    }
                }
            }
        }
        CreativeSlotAction::Destroy => {
            if shift {
                for s in &mut inv.hotbar { *s = None; }
            }
            inv.carried_item = None;
        }
    }
}
