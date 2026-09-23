use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use bevy::math::Vec3;
use crate::core::world::{load_world_from_disk, save_world_to_disk, WorldAutosaver};
use crate::engine::RenderSnapshot;
use crate::input::LogicCommand;
use crate::world::VoxelWorld;

pub struct LogicWorker;

impl LogicWorker {
    pub fn spawn(
        snapshot_shared: Arc<Mutex<Option<RenderSnapshot>>>,
        cmd_rx: mpsc::Receiver<LogicCommand>,
        block_tx: mpsc::Sender<crate::network::Packet>,
        running: Arc<AtomicBool>,
        world_storage: Arc<Mutex<Option<Arc<crate::world::ConcurrentChunkStorage>>>>,
    ) -> thread::JoinHandle<()> {
        thread::Builder::new()
            .name("logic-worker".into())
            .spawn(move || {
                let mut world = VoxelWorld::new();
                if !load_world_from_disk(&mut world) {
                    world = VoxelWorld::new_island();
                    save_world_to_disk(&world);
                }
                *world_storage.lock().unwrap() = Some(world.storage.clone());
                let autosaver = WorldAutosaver::start(world.storage.clone());
                let (mut player, mut camera, mut inventory, mut items, mut tick_system) = super::worker_init::init_logic_entities(&world);
                let mut block_entities = crate::engine::BlockEntityManager::new();
                let mut open_container: Option<crate::engine::ContainerRef> = None;

                player.held_item = inventory.hotbar[inventory.selected_slot].as_ref().map(|s| s.item);
                let snap = RenderSnapshot::capture(&mut world, &player, &camera, &inventory, &items, &block_entities, open_container, true);
                *snapshot_shared.lock().unwrap() = Some(snap);

                let (mut move_input, mut jump_input, mut sneak_input, mut sprint_input) = (Vec3::ZERO, false, false, false);
                let (mut aim_dir, mut mouse_ndc, mut current_hovered_block) = (None, (0.0f32, 0.0f32), None);
                let (mut profiler, mut piechart_state, mut show_chunk_borders) = (crate::engine::Profiler::new(), crate::engine::ProfilerPieChartState::new(), false);
                let (mut mining_state, mut last_loop, mut aspect) = (crate::engine::MiningState::new(), Instant::now(), 16.0 / 9.0);

                while running.load(Ordering::Relaxed) {
                    let now = Instant::now();
                    let dt = now.duration_since(last_loop).as_secs_f32().min(0.05);
                    last_loop = now;
                    profiler.start_tick();

                    profiler.push("commands");
                    while let Ok(cmd) = cmd_rx.try_recv() {
                        super::worker_commands::handle_worker_command(
                            cmd, &mut world, &mut player, &mut camera, &mut inventory, &mut items, &mut block_entities, &mut open_container, &mut move_input,
                            &mut jump_input, &mut sneak_input, &mut sprint_input, &mut aim_dir, &mut mouse_ndc, &mut aspect,
                            &mut current_hovered_block, &mut mining_state, &mut tick_system,
                            &mut show_chunk_borders, &mut piechart_state, &profiler, &block_tx,
                        );
                    }
                    profiler.pop();

                    profiler.push("tick");
                    let ticks = tick_system.advance(dt);
                    for _ in 0..ticks {
                        super::worker_tick::tick_world_and_player(&mut world, &mut player, &mut tick_system, &block_tx);
                    }
                    profiler.pop();

                    profiler.push("physics");
                    block_entities.tick(dt);
                    super::logic_step::step_physics_and_mining(
                        &mut player, &mut world, &mut items, &mut inventory,
                        &mut mining_state, &mut tick_system, move_input, jump_input, sneak_input, sprint_input, aim_dir, dt, &block_tx,
                    );
                    super::mining_helper::update_continuous_mining(
                        &mut mining_state, &world, &player, &camera, &inventory, aspect, mouse_ndc, &mut current_hovered_block,
                    );
                    profiler.pop();

                    profiler.push("snapshot");
                    super::worker_snapshot::update_snapshot(
                        &snapshot_shared, &mut world, &mut player, &camera, &inventory,
                        &items, &block_entities, open_container, current_hovered_block, mouse_ndc, show_chunk_borders,
                        &mining_state, &profiler, &piechart_state,
                    );
                    profiler.pop();
                    profiler.end_tick();

                    thread::sleep(Duration::from_millis(5));
                }
                autosaver.flush();
                drop(autosaver);
                save_world_to_disk(&world);
            })
            .expect("Failed to spawn logic worker")
    }
}