use std::sync::{Arc, Mutex};
use crate::engine::{BlockEntityManager, Camera, ContainerRef, Inventory, ItemEntityManager, MiningState, Player, Profiler, ProfilerPieChartState, RenderSnapshot};
use crate::world::VoxelWorld;

pub fn update_snapshot(
    snapshot_shared: &Arc<Mutex<Option<RenderSnapshot>>>,
    world: &mut VoxelWorld,
    player: &mut Player,
    camera: &Camera,
    inventory: &Inventory,
    items: &ItemEntityManager,
    block_entities: &BlockEntityManager,
    open_container: Option<ContainerRef>,
    hovered: Option<(i32, i32, i32)>,
    mouse_ndc: (f32, f32),
    show_chunk_borders: bool,
    mining: &MiningState,
    profiler: &Profiler,
    pie: &ProfilerPieChartState,
) {
    player.held_item = inventory.hotbar[inventory.selected_slot].as_ref().map(|s| s.item);
    let mut snap = RenderSnapshot::capture(world, player, camera, inventory, items, block_entities, open_container, false);
    snap.hovered_block = hovered;
    snap.hovered_exposed_faces = super::mining_helper::compute_exposed_faces(world, hovered);
    snap.mouse_ndc = mouse_ndc;
    snap.show_chunk_borders = show_chunk_borders;
    super::mining_helper::populate_snapshot_mining(&mut snap, mining, world);
    if pie.is_open {
        snap.profiler_piechart = Some((profiler.get_times(&pie.current_path), pie.current_path.clone()));
    }
    let mut lock = snapshot_shared.lock().unwrap();
    if snap.world_mesh.is_none() {
        if let Some(prev) = lock.as_mut() {
            if prev.world_mesh.is_some() {
                snap.world_mesh = prev.world_mesh.take();
            }
        }
    }
    *lock = Some(snap);
}
