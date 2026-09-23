use crate::engine::{Camera, Inventory, MiningState, Player, RenderSnapshot};
use crate::world::VoxelWorld;

pub fn update_continuous_mining(
    mining_state: &mut MiningState,
    world: &VoxelWorld,
    player: &Player,
    camera: &Camera,
    inventory: &Inventory,
    aspect: f32,
    mouse_ndc: (f32, f32),
    current_hovered: &mut Option<(i32, i32, i32)>,
) {
    if !mining_state.is_holding_left || inventory.is_open {
        return;
    }
    let (ro, rd) = camera.ndc_to_ray(player.position, aspect, mouse_ndc.0, mouse_ndc.1);
    let hit = crate::world::raycast_world_precise(world, ro, rd, 150.0);
    *current_hovered = hit.map(|(b, _, _)| b);
    if mining_state.target != *current_hovered {
        mining_state.set_mining(*current_hovered, current_hovered.is_some());
    }
}

pub fn compute_exposed_faces(world: &VoxelWorld, target: Option<(i32, i32, i32)>) -> u8 {
    let mut exposed = 0u8;
    if let Some((bx, by, bz)) = target {
        let offsets = [(-1, 0, 0), (1, 0, 0), (0, -1, 0), (0, 1, 0), (0, 0, -1), (0, 0, 1)];
        for (idx, (dx, dy, dz)) in offsets.iter().enumerate() {
            if world.get_block(bx + dx, by + dy, bz + dz).is_transparent() {
                exposed |= 1 << idx;
            }
        }
    }
    exposed
}

pub fn populate_snapshot_mining(
    snapshot: &mut RenderSnapshot,
    mining_state: &MiningState,
    world: &VoxelWorld,
) {
    snapshot.mining_target = if mining_state.is_active { mining_state.target } else { None };
    snapshot.mining_stage = mining_state.get_stage();
    snapshot.mining_exposed_faces = compute_exposed_faces(world, snapshot.mining_target);
    snapshot.mining_progress = if mining_state.is_active { Some(mining_state.progress) } else { None };
}
