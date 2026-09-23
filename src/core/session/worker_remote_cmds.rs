use bevy::math::Vec3;
use crate::input::LogicCommand;
use crate::world::{apply_chunk_runs, Chunk, VoxelWorld};

pub fn handle_remote_command(
    cmd: &LogicCommand,
    world: &mut VoxelWorld,
    items: &mut crate::engine::ItemEntityManager,
) -> bool {
    match cmd {
        LogicCommand::RemoteBlockChange { x, y, z, block_type } => {
            world.set_block(*x, *y, *z, crate::world::block_from_u8(*block_type));
            true
        }
        LogicCommand::RemoteChunkData { x, y, z, runs } => {
            let coords = (*x, *y, *z);
            let exists = world.storage.get_chunk_write(&coords, |chunk| {
                apply_chunk_runs(chunk, runs);
                chunk.disk_dirty = false;
                chunk.is_dirty = true;
            }).is_some();
            if !exists {
                let mut chunk = Chunk::new(coords);
                apply_chunk_runs(&mut chunk, runs);
                chunk.disk_dirty = false;
                chunk.is_dirty = true;
                world.storage.insert(coords, chunk);
            }
            world.needs_mesh_rebuild = true;
            true
        }
        LogicCommand::RemoteSpawnItem { entity_id, pos, item_type, count } => {
            if let Some(it) = crate::engine::items::item_from_u8(*item_type) {
                items.spawn_with_id(*entity_id, *pos, Vec3::ZERO, crate::engine::ItemStack::new(it, *count));
            }
            true
        }
        LogicCommand::RemoteRemoveEntities { entity_ids } => {
            items.remove_many_by_id(entity_ids);
            true
        }
        _ => false,
    }
}
