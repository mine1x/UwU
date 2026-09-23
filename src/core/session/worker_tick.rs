use std::sync::mpsc::Sender;
use crate::engine::{Player, TickSystem};
use crate::network::Packet;
use crate::world::{block_to_u8, BlockType, FluidSimulator, VoxelWorld};

pub fn tick_world_and_player(
    world: &mut VoxelWorld,
    player: &mut Player,
    tick_system: &mut TickSystem<BlockType>,
    block_tx: &Sender<Packet>,
) {
    let cx = (player.position.x / 16.0).floor() as i32;
    let cz = (player.position.z / 16.0).floor() as i32;
    crate::world::schedule_camera_chunk_fluids(world, tick_system, (cx, cz), 2);

    let mut fluid_changes = Vec::new();
    FluidSimulator::tick_fluids(world, tick_system, &mut fluid_changes);

    for (x, y, z, bt) in fluid_changes {
        let _ = block_tx.send(Packet::ClientboundBlockUpdate {
            x, y, z, block_type: block_to_u8(&bt),
        });
    }

    if player.stamina < 100.0 {
        player.stamina = (player.stamina + 0.5).min(100.0);
    }
}
