use bevy::prelude::*;
use std::sync::{Arc, Mutex};
use crate::engine::ItemEntityManager;

#[derive(Resource, Clone)]
pub struct ItemEntityResource {
    pub manager: Arc<Mutex<ItemEntityManager>>,
}

pub struct ItemEntityPlugin;

impl Plugin for ItemEntityPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ItemEntityResource {
            manager: Arc::new(Mutex::new(ItemEntityManager::new())),
        });
        app.add_systems(Update, item_ecs_tick_system);
    }
}

fn item_ecs_tick_system(item_res: Res<ItemEntityResource>) {
    if let Ok(mut mgr) = item_res.manager.try_lock() {
        mgr.sync_items();
    }
}
