use super::block::BlockType;
use super::voxel::VoxelWorld;
use crate::engine::{TickPriority, TickSystem};

pub struct FluidSimulator;

impl FluidSimulator {
    pub const WATER_TICK_DELAY: u64 = 5;

    pub fn schedule_initial_water(world: &VoxelWorld, tick_system: &mut TickSystem<BlockType>) {
        world.for_each_chunk(|&coords, chunk| {
            for lx in 0..16 {
                for ly in 0..16 {
                    for lz in 0..16 {
                        let block = chunk.get_block(lx, ly, lz);
                        if block.is_fluid() {
                            let gx = coords.0 * 16 + lx as i32;
                            let gy = coords.1 * 16 + ly as i32;
                            let gz = coords.2 * 16 + lz as i32;
                            tick_system.schedule_tick(block, (gx, gy, gz), Self::WATER_TICK_DELAY, TickPriority::Normal);
                        }
                    }
                }
            }
        });
    }

    pub fn tick_fluids(
        world: &mut VoxelWorld,
        tick_system: &mut TickSystem<BlockType>,
        changed_out: &mut Vec<(i32, i32, i32, BlockType)>,
    ) -> bool {
        let ready_ticks = tick_system.drain_current_ticks();
        let mut changed = false;

        for tick in ready_ticks {
            let (x, y, z) = tick.pos;
            let current = world.get_block(x, y, z);
            if !current.is_fluid() { continue; }

            let (new_liquid, spread_positions) = super::fluid_spread::compute_fluid_update(world, x, y, z, current);

            if new_liquid != current {
                world.set_block(x, y, z, new_liquid);
                changed = true;
                changed_out.push((x, y, z, new_liquid));
                if new_liquid.is_fluid() {
                    tick_system.schedule_tick(new_liquid, (x, y, z), Self::WATER_TICK_DELAY, TickPriority::Normal);
                }
                Self::schedule_neighbors(world, tick_system, x, y, z);
            }

            for p in spread_positions {
                let nb = world.get_block(p.0, p.1, p.2);
                changed = true;
                changed_out.push((p.0, p.1, p.2, nb));
                tick_system.schedule_tick(nb, p, Self::WATER_TICK_DELAY, TickPriority::Normal);
            }
        }

        changed
    }

    pub fn schedule_neighbors(world: &VoxelWorld, tick_system: &mut TickSystem<BlockType>, x: i32, y: i32, z: i32) {
        let offsets = [(0, -1, 0), (0, 1, 0), (1, 0, 0), (-1, 0, 0), (0, 0, 1), (0, 0, -1)];
        for (dx, dy, dz) in offsets {
            let (nx, ny, nz) = (x + dx, y + dy, z + dz);
            let b = world.get_block(nx, ny, nz);
            if b.is_fluid() {
                tick_system.schedule_tick(b, (nx, ny, nz), Self::WATER_TICK_DELAY, TickPriority::Normal);
            }
        }
    }
}