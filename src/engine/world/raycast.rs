use bevy::math::Vec3;
use super::voxel::VoxelWorld;

pub fn raycast_world_precise(
    world: &VoxelWorld,
    ray_origin: Vec3,
    ray_dir: Vec3,
    max_dist: f32,
) -> Option<((i32, i32, i32), (i32, i32, i32), Vec3)> {
    let mut x = ray_origin.x.floor() as i32;
    let mut y = ray_origin.y.floor() as i32;
    let mut z = ray_origin.z.floor() as i32;

    let step_x = if ray_dir.x >= 0.0 { 1 } else { -1 };
    let step_y = if ray_dir.y >= 0.0 { 1 } else { -1 };
    let step_z = if ray_dir.z >= 0.0 { 1 } else { -1 };

    let delta_t_x = if ray_dir.x != 0.0 { (1.0 / ray_dir.x).abs() } else { f32::INFINITY };
    let delta_t_y = if ray_dir.y != 0.0 { (1.0 / ray_dir.y).abs() } else { f32::INFINITY };
    let delta_t_z = if ray_dir.z != 0.0 { (1.0 / ray_dir.z).abs() } else { f32::INFINITY };

    let mut t_max_x = if ray_dir.x > 0.0 { (x as f32 + 1.0 - ray_origin.x) / ray_dir.x } else if ray_dir.x < 0.0 { (ray_origin.x - x as f32) / (-ray_dir.x) } else { f32::INFINITY };
    let mut t_max_y = if ray_dir.y > 0.0 { (y as f32 + 1.0 - ray_origin.y) / ray_dir.y } else if ray_dir.y < 0.0 { (ray_origin.y - y as f32) / (-ray_dir.y) } else { f32::INFINITY };
    let mut t_max_z = if ray_dir.z > 0.0 { (z as f32 + 1.0 - ray_origin.z) / ray_dir.z } else if ray_dir.z < 0.0 { (ray_origin.z - z as f32) / (-ray_dir.z) } else { f32::INFINITY };

    let mut prev_voxel = (x, y, z);
    let mut current_t = 0.0;

    let start_block = world.get_block(x, y, z);
    if start_block.is_solid() {
        return Some(((x, y, z), prev_voxel, ray_origin));
    }

    while current_t < max_dist {
        prev_voxel = (x, y, z);
        if t_max_x < t_max_y {
            if t_max_x < t_max_z {
                x += step_x;
                current_t = t_max_x;
                t_max_x += delta_t_x;
            } else {
                z += step_z;
                current_t = t_max_z;
                t_max_z += delta_t_z;
            }
        } else {
            if t_max_y < t_max_z {
                y += step_y;
                current_t = t_max_y;
                t_max_y += delta_t_y;
            } else {
                z += step_z;
                current_t = t_max_z;
                t_max_z += delta_t_z;
            }
        }

        let block = world.get_block(x, y, z);
        if block.is_solid() {
            let hit_point = ray_origin + ray_dir * current_t;
            return Some(((x, y, z), prev_voxel, hit_point));
        }
    }

    None
}