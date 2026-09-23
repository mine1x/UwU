use super::block::BlockType;
use super::voxel::VoxelWorld;
use crate::engine::{TickPriority, TickSystem};

pub fn schedule_camera_chunk_fluids(
    world: &VoxelWorld,
    tick_system: &mut TickSystem<BlockType>,
    center_chunk: (i32, i32),
    radius: i32,
) {
    for cx in (center_chunk.0 - radius)..=(center_chunk.0 + radius) {
        for cz in (center_chunk.1 - radius)..=(center_chunk.1 + radius) {
            world.storage.get_chunk_read(&(cx, 0, cz), |chunk| {
                if chunk.section.fluid_count > 0 {
                    for lx in 0..16 {
                        for ly in 0..16 {
                            for lz in 0..16 {
                                let b = chunk.get_block(lx, ly, lz);
                                if b.is_fluid() {
                                    let gx = cx * 16 + lx as i32;
                                    let gy = ly as i32;
                                    let gz = cz * 16 + lz as i32;
                                    tick_system.schedule_tick(b, (gx, gy, gz), 5, TickPriority::Normal);
                                }
                            }
                        }
                    }
                }
            });
        }
    }
}
