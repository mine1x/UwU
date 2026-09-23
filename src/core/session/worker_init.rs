use crate::engine::{Camera, Inventory, ItemEntityManager, Player, TickSystem};
use crate::world::{BlockType, FluidSimulator, VoxelWorld};

pub fn init_logic_entities(world: &VoxelWorld, spawn_pos: (i32, i32, i32)) -> (Player, Camera, Inventory, ItemEntityManager, TickSystem<BlockType>) {
    let (spawn_x, spawn_y, spawn_z) = spawn_pos;
    let player = Player::new(spawn_x as f32 + 0.5, spawn_y as f32, spawn_z as f32 + 0.5);
    let camera = Camera::new();
    let inventory = Inventory::new();
    let mut items = ItemEntityManager::new();
    let mut tick_system = TickSystem::new(20.0);
    FluidSimulator::schedule_initial_water(world, &mut tick_system);

    items.sync_items();

    (player, camera, inventory, items, tick_system)
}
