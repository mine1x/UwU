use super::block::BlockType;
use super::voxel::VoxelWorld;

pub fn spread_horizontally(
    world: &mut VoxelWorld,
    x: i32,
    y: i32,
    z: i32,
    cur_level: u8,
    spread_positions: &mut Vec<(i32, i32, i32)>,
) {
    if cur_level <= 1 { return; }
    let next_level = cur_level - 1;
    let mut lowest_dist = 1000;
    let mut valid_targets = Vec::with_capacity(4);

    for &(dx, dz) in &[(1, 0), (-1, 0), (0, 1), (0, -1)] {
        let (nx, nz) = (x + dx, z + dz);
        let nb = world.get_block(nx, y, nz);
        if nb == BlockType::Air || (nb.is_fluid() && nb != BlockType::WaterSource) {
            let d = if super::fluid_slope_dist::is_hole(world, nx, y, nz) {
                0
            } else {
                super::fluid_slope_dist::get_slope_distance(world, nx, y, nz, 1, (-dx, -dz))
            };
            if d < lowest_dist { lowest_dist = d; }
            valid_targets.push((nx, nz, d));
        }
    }

    for (nx, nz, d) in valid_targets {
        let should_flow = if lowest_dist < 1000 { d == lowest_dist } else { true };
        if should_flow {
            let nb = world.get_block(nx, y, nz);
            if nb == BlockType::Air {
                world.set_block(nx, y, nz, BlockType::FlowingWater { level: next_level });
                spread_positions.push((nx, y, nz));
            } else if let BlockType::FlowingWater { level: nl } = nb {
                if nl + 1 < cur_level {
                    world.set_block(nx, y, nz, BlockType::FlowingWater { level: next_level });
                    spread_positions.push((nx, y, nz));
                }
            }
        }
    }
}