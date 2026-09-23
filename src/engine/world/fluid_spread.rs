use super::block::BlockType;
use super::voxel::VoxelWorld;

pub fn compute_fluid_update(
    world: &mut VoxelWorld,
    x: i32,
    y: i32,
    z: i32,
    current: BlockType,
) -> (BlockType, Vec<(i32, i32, i32)>) {
    let mut spread_positions = Vec::new();
    let below = (x, y - 1, z);
    let below_b = world.get_block(below.0, below.1, below.2);

    // 1. Check vertical downward flow
    if below_b == BlockType::Air || (below_b.is_fluid() && below_b != BlockType::WaterSource) {
        if below_b != (BlockType::FlowingWater { level: 7 }) {
            world.set_block(below.0, below.1, below.2, BlockType::FlowingWater { level: 7 });
            spread_positions.push(below);
        }
        if current != BlockType::WaterSource {
            return (current, spread_positions);
        }
    }

    // 2. Minecraft source conversion (2 or more adjacent sources on solid ground create source)
    let horizontal_neighbors = [(x + 1, y, z), (x - 1, y, z), (x, y, z + 1), (x, y, z - 1)];
    let mut source_count = 0;
    let mut max_neighbor_level = 0u8;

    for (nx, ny, nz) in horizontal_neighbors {
        let nb = world.get_block(nx, ny, nz);
        match nb {
            BlockType::WaterSource => {
                source_count += 1;
                max_neighbor_level = max_neighbor_level.max(8);
            }
            BlockType::FlowingWater { level } => {
                max_neighbor_level = max_neighbor_level.max(level);
            }
            _ => {}
        }
    }

    if current != BlockType::WaterSource && source_count >= 2 && (below_b.is_solid() || below_b == BlockType::WaterSource) {
        return (BlockType::WaterSource, spread_positions);
    }

    // 3. Flowing outward horizontally if on solid floor or source
    let cur_level = match current {
        BlockType::WaterSource => 8,
        BlockType::FlowingWater { level } => level,
        _ => 0,
    };
    super::fluid_spread_horiz::spread_horizontally(world, x, y, z, cur_level, &mut spread_positions);

    // 4. Check if current flowing water is disconnected from source
    if current != BlockType::WaterSource {
        let above_b = world.get_block(x, y + 1, z);
        if !above_b.is_fluid() && max_neighbor_level <= cur_level {
            if max_neighbor_level <= 1 {
                return (BlockType::Air, spread_positions);
            } else {
                return (BlockType::FlowingWater { level: max_neighbor_level - 1 }, spread_positions);
            }
        }
    }

    (current, spread_positions)
}