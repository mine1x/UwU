use std::collections::HashMap;
use bevy::prelude::Resource;
use super::chest::Chest;
use super::furnace::Furnace;
use super::crafting_table::CraftingTable;

#[derive(Resource, Default)]
pub struct BlockEntityManager {
    pub chests: HashMap<(i32, i32, i32), Chest>,
    pub furnaces: HashMap<(i32, i32, i32), Furnace>,
    pub crafting_tables: HashMap<(i32, i32, i32), CraftingTable>,
}

impl BlockEntityManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&mut self, dt: f32) {
        for furnace in self.furnaces.values_mut() {
            furnace.update(dt);
        }
    }

    pub fn clear(&mut self) {
        self.chests.clear();
        self.furnaces.clear();
        self.crafting_tables.clear();
    }
}
