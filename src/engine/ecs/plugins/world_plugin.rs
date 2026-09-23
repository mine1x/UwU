use bevy::prelude::*;
use std::sync::{Arc, Mutex};
use crate::world::ConcurrentChunkStorage;

#[derive(Resource, Clone)]
pub struct VoxelWorldResource {
    pub storage: Arc<Mutex<Option<Arc<ConcurrentChunkStorage>>>>,
}

pub struct VoxelWorldPlugin;

impl Plugin for VoxelWorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(VoxelWorldResource {
            storage: Arc::new(Mutex::new(None)),
        });
        app.add_systems(Update, update_world_status);
    }
}

fn update_world_status(_world_res: Res<VoxelWorldResource>) {
    // Bevy ECS system for world background tasks
}
