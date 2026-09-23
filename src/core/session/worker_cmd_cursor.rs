use bevy::math::Vec3;
use crate::engine::{Camera, MiningState, Player};
use crate::core::inventory::HudZone;
use crate::world::VoxelWorld;

pub fn handle_update_cursor(
    aspect: f32,
    mouse_pos: (f32, f32),
    screen_size: (f32, f32),
    aspect_out: &mut f32,
    mouse_ndc: &mut (f32, f32),
    inventory_open: bool,
    current_hovered: &mut Option<(i32, i32, i32)>,
    mining_state: &mut MiningState,
    camera: &Camera,
    player: &Player,
    world: &VoxelWorld,
    aim_dir: &mut Option<Vec3>,
) {
    *aspect_out = aspect;
    *mouse_ndc = ((mouse_pos.0 / screen_size.0) * 2.0 - 1.0, 1.0 - (mouse_pos.1 / screen_size.1) * 2.0);
    if HudZone::is_point_in_hud(mouse_pos.0, mouse_pos.1, screen_size.0, screen_size.1, inventory_open) {
        *current_hovered = None;
        *aim_dir = None;
        if mining_state.is_holding_left { mining_state.set_mining(None, false); }
    } else {
        let (ro, rd) = camera.screen_to_ray(player.position, aspect, mouse_pos.0, mouse_pos.1, screen_size.0, screen_size.1);
        let hit = crate::world::raycast_world_precise(world, ro, rd, 150.0);
        *current_hovered = hit.map(|(b, _, _)| b);
        *aim_dir = Some(match hit {
            Some((b, _, _)) => {
                let eye = player.position + Vec3::new(0.0, 1.4, 0.0);
                (Vec3::new(b.0 as f32 + 0.5, b.1 as f32 + 0.5, b.2 as f32 + 0.5) - eye).normalize()
            }
            None => {
                let eye = player.position + Vec3::new(0.0, 1.4, 0.0);
                let t = (player.position.y - ro.y) / rd.y;
                (ro + rd * t - eye).normalize()
            }
        });
        if mining_state.is_holding_left && mining_state.target != *current_hovered {
            mining_state.set_mining(*current_hovered, current_hovered.is_some());
        }
    }
}
