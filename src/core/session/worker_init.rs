use crate::engine::{Camera, Inventory, ItemEntityManager, Player, TickSystem};
use crate::world::{BlockType, FluidSimulator, VoxelWorld};

pub fn init_logic_entities(world: &VoxelWorld) -> (Player, Camera, Inventory, ItemEntityManager, TickSystem<BlockType>) {
    let player = Player::new(7.0, 5.0, 7.0);
    let camera = Camera::new();
    let inventory = Inventory::new();
    let mut items = ItemEntityManager::new();
    let mut tick_system = TickSystem::new(20.0);
    FluidSimulator::schedule_initial_water(world, &mut tick_system);

    items.sync_items();

    (player, camera, inventory, items, tick_system)
}
