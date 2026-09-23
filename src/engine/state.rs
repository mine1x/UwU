use bevy::math::Vec3;
use bevy::prelude::Resource;
use crate::engine::items::{ItemEntityManager, ItemType};
use crate::engine::{BlockEntityManager, Camera, ContainerRef, ContainerSnapshot, Inventory, Player};
use crate::render::types::Vertex;
use crate::world::VoxelWorld;

#[derive(Resource, Clone)]
pub struct RenderSnapshot {
    pub player_pos: Vec3,
    pub player_yaw: f32,
    pub is_sneaking: bool,
    pub is_sprinting: bool,
    pub mining_swing: f32,
    pub player_mesh: (Vec<Vertex>, Vec<u32>),
    pub world_mesh: Option<(Vec<Vertex>, Vec<u32>)>,
    pub camera_rotation: f32,
    pub camera_pitch: f32,
    pub camera_distance: f32,
    pub health: f32,
    pub stamina: f32,
    pub hunger: f32,
    pub saturation: f32,
    pub xp_level: u32,
    pub xp_progress: f32,
    pub game_mode: super::GameMode,
    pub hotbar_items: [Option<(ItemType, u32)>; 9],
    pub storage_items: [Option<(ItemType, u32)>; 27],
    pub all_slots: [Option<(ItemType, u32)>; 46],
    pub inventory_open: bool,
    pub selected_slot: usize,
    pub carried_item: Option<(ItemType, u32)>,
    pub mouse_ndc: (f32, f32),
    pub mining_progress: Option<f32>,
    pub mining_target: Option<(i32, i32, i32)>,
    pub mining_stage: Option<usize>,
    pub mining_exposed_faces: u8,
    pub hovered_block: Option<(i32, i32, i32)>,
    pub hovered_exposed_faces: u8,
    pub show_chunk_borders: bool,
    pub profiler_piechart: Option<(Vec<super::debug::ResultField>, String)>,
    pub container: Option<ContainerSnapshot>,
}

impl RenderSnapshot {
    pub fn capture(
        world: &mut VoxelWorld,
        player: &Player,
        camera: &Camera,
        inventory: &Inventory,
        items: &ItemEntityManager,
        block_entities: &BlockEntityManager,
        open_container: Option<ContainerRef>,
        force_mesh: bool,
    ) -> Self {
        let world_mesh = if force_mesh || world.needs_mesh_rebuild || world.cached_world_mesh.0.is_empty() {
            world.rebuild_all_dirty_chunks();
            Some(world.cached_world_mesh.clone())
        } else {
            None
        };

        let (hotbar_items, storage_items, all_slots) = super::state_slots::extract_inventory_slots(inventory);
        let mut player_mesh = player.mesh();
        items.build_mesh(&mut player_mesh.0, &mut player_mesh.1);

        let container = super::state_container::extract_container_snapshot(open_container, block_entities);

        Self {
            player_pos: player.position,
            player_yaw: player.yaw,
            is_sneaking: player.is_sneaking,
            is_sprinting: player.is_sprinting,
            mining_swing: player.mining_swing,
            player_mesh,
            world_mesh,
            camera_rotation: camera.rotation_angle, camera_pitch: camera.pitch_angle, camera_distance: camera.distance,
            health: player.health, stamina: player.stamina, hunger: player.hunger,
            saturation: player.saturation, xp_level: player.xp_level,
            xp_progress: player.xp_progress(), game_mode: player.game_mode,
            hotbar_items, storage_items, all_slots,
            carried_item: inventory.carried_item.as_ref().map(|s| (s.item, s.count)),
            mouse_ndc: (0.0, 0.0),
            mining_progress: None, mining_target: None, mining_stage: None, mining_exposed_faces: 0,
            inventory_open: inventory.is_open,
            selected_slot: inventory.selected_slot,
            hovered_block: None,
            hovered_exposed_faces: 0,
            show_chunk_borders: false,
            profiler_piechart: None,
            container,
        }
    }
}