use bevy::prelude::Component;
use crate::engine::items::{ItemStack, ItemType};
use crate::world::block::BlockType;

#[derive(Component, Clone, Debug)]
pub struct Inventory {
    pub hotbar: [Option<ItemStack>; 9],
    pub storage: [Option<ItemStack>; 27],
    pub armor: [Option<ItemStack>; 4],
    pub offhand: Option<ItemStack>,
    pub craft: [Option<ItemStack>; 4],
    pub result: Option<ItemStack>,
    pub carried_item: Option<ItemStack>,
    pub is_open: bool,
    pub selected_slot: usize,
}

impl Inventory {
    pub fn new() -> Self {
        let hotbar = [
            Some(ItemStack::new(ItemType::Block(BlockType::Grass), 64)),
            Some(ItemStack::new(ItemType::Block(BlockType::Dirt), 64)),
            Some(ItemStack::new(ItemType::Block(BlockType::Wood), 64)),
            Some(ItemStack::new(ItemType::Block(BlockType::Stone), 64)),
            Some(ItemStack::new(ItemType::Block(BlockType::WaterSource), 16)),
            Some(ItemStack::new(ItemType::Apple, 16)),
            Some(ItemStack::new(ItemType::Bread, 16)),
            Some(ItemStack::new(ItemType::DiamondSword, 1)),
            Some(ItemStack::new(ItemType::DiamondPickaxe, 1)),
        ];
        let storage = [
            Some(ItemStack::new(ItemType::Block(BlockType::OakLog), 64)),
            Some(ItemStack::new(ItemType::Block(BlockType::OakPlanks), 64)),
            Some(ItemStack::new(ItemType::Block(BlockType::OakLeaves), 64)),
            Some(ItemStack::new(ItemType::Bucket, 1)),
            Some(ItemStack::new(ItemType::WaterBucket, 1)),
            None, None, None, None,
            None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None,
        ];
        Self {
            hotbar, storage, armor: [None, None, None, None],
            offhand: None, craft: [None, None, None, None], result: None,
            carried_item: None, is_open: false, selected_slot: 0,
        }
    }

    pub fn toggle_open(&mut self) { self.is_open = !self.is_open; }
    pub fn select_slot(&mut self, slot: usize) { if slot < 9 { self.selected_slot = slot; } }
    pub fn next_slot(&mut self) { self.selected_slot = (self.selected_slot + 1) % 9; }
    pub fn prev_slot(&mut self) { self.selected_slot = if self.selected_slot == 0 { 8 } else { self.selected_slot - 1 }; }
    pub fn click_slot(&mut self, slot: usize, right: bool, shift: bool) {
        super::inventory_click::handle_slot_click(self, slot, right, shift);
    }

    pub fn add_stack(&mut self, stack: &ItemStack) -> u32 {
        let mut rem = stack.count;
        let all_slots = self.hotbar.iter_mut().chain(self.storage.iter_mut());
        for slot in all_slots {
            if let Some(existing) = slot {
                if existing.item == stack.item { rem = existing.add(rem); if rem == 0 { return 0; } }
            }
        }
        let all_slots2 = self.hotbar.iter_mut().chain(self.storage.iter_mut());
        for slot in all_slots2 {
            if slot.is_none() {
                let max = stack.item.max_stack_size();
                let take = rem.min(max);
                *slot = Some(ItemStack::new(stack.item, take));
                rem -= take;
                if rem == 0 { return 0; }
            }
        }
        rem
    }

    pub fn add_item(&mut self, block: BlockType, amount: u32) -> bool {
        self.add_stack(&ItemStack::new(ItemType::Block(block), amount)) == 0
    }

    pub fn consume_selected(&mut self) -> Option<BlockType> {
        if let Some(item) = &mut self.hotbar[self.selected_slot] {
            let res = match item.item {
                ItemType::Block(b) => Some(b),
                ItemType::WaterBucket => Some(BlockType::WaterSource),
                _ => None,
            };
            if res.is_some() {
                if item.count > 1 { item.count -= 1; } else { self.hotbar[self.selected_slot] = None; }
            }
            res
        } else {
            None
        }
    }

    pub fn clear(&mut self) {
        self.hotbar = [None, None, None, None, None, None, None, None, None];
        self.storage = [None; 27];
        self.armor = [None; 4];
        self.offhand = None;
        self.craft = [None; 4];
        self.result = None;
        self.carried_item = None;
        self.is_open = false;
        self.selected_slot = 0;
    }
}