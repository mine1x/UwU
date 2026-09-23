use super::block::BlockType;
use super::voxel::VoxelWorld;

pub fn is_hole(world: &VoxelWorld, x: i32, y: i32, z: i32) -> bool {
    let below = world.get_block(x, y - 1, z);
    below == BlockType::Air || (below.is_fluid() && below != BlockType::WaterSource)
}

fn can_pass_through(world: &VoxelWorld, x: i32, y: i32, z: i32) -> bool {
    let b = world.get_block(x, y, z);
    b == BlockType::Air || (b.is_fluid() && b != BlockType::WaterSource)
}

pub fn get_slope_distance(
    world: &VoxelWorld,
    x: i32,
    y: i32,
    z: i32,
    pass: i32,
    from_dir: (i32, i32),
) -> i32 {
    let mut lowest = 1000;
    let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    for (dx, dz) in dirs {
        if (dx, dz) != from_dir {
            let (nx, nz) = (x + dx, z + dz);
            if can_pass_through(world, nx, y, nz) {
                if is_hole(world, nx, y, nz) {
                    return pass;
                }
                if pass < 4 {
                    let v = get_slope_distance(world, nx, y, nz, pass + 1, (-dx, -dz));
                    if v < lowest {
                        lowest = v;
                    }
                }
            }
        }
    }
    lowest
}