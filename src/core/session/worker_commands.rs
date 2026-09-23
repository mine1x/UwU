use bevy::math::Vec3;
use crate::engine::{Camera, ContainerRef, Inventory, Player, TickSystem};
use crate::input::LogicCommand;
use crate::world::{BlockType, VoxelWorld};

pub fn handle_worker_command(
    cmd: LogicCommand,
    world: &mut VoxelWorld,
    player: &mut Player,
    camera: &mut Camera,
    inventory: &mut Inventory,
    items: &mut crate::engine::ItemEntityManager,
    block_entities: &mut crate::engine::BlockEntityManager,
    open_container: &mut Option<ContainerRef>,
    move_input: &mut Vec3,
    jump_input: &mut bool,
    sneak_input: &mut bool,
    sprint_input: &mut bool,
    aim_dir: &mut Option<Vec3>,
    mouse_ndc: &mut (f32, f32),
    aspect_out: &mut f32,
    current_hovered: &mut Option<(i32, i32, i32)>,
    mining_state: &mut crate::engine::MiningState,
    tick_system: &mut TickSystem<BlockType>,
    show_chunk_borders: &mut bool,
    piechart_state: &mut crate::engine::ProfilerPieChartState,
    profiler: &crate::engine::Profiler,
    block_tx: &std::sync::mpsc::Sender<crate::network::Packet>,
) {
    match cmd {
        LogicCommand::TogglePieChart => piechart_state.toggle(),
        LogicCommand::ToggleChunkBorders => *show_chunk_borders = !*show_chunk_borders,
        LogicCommand::ProfilerNavigate(idx) => {
            let times = profiler.get_times(&piechart_state.current_path);
            piechart_state.handle_key_press(idx, &times);
        }
        LogicCommand::MoveInput(dir) => *move_input = dir,
        LogicCommand::Jump(j) => *jump_input = j,
        LogicCommand::Sneak(s) => *sneak_input = s,
        LogicCommand::Sprint(s) => *sprint_input = s,
        LogicCommand::RotateCamera(amt) => camera.rotation_angle += amt,
        LogicCommand::ZoomCamera(amt) => camera.distance = (camera.distance + amt).clamp(6.0, 80.0),
        LogicCommand::ToggleInventory => {
            if open_container.is_some() {
                *open_container = None;
                inventory.is_open = false;
                inventory.carried_item = None;
            } else {
                inventory.toggle_open();
            }
        }
        LogicCommand::CloseInventory => {
            if open_container.is_some() {
                *open_container = None;
                inventory.carried_item = None;
            }
            inventory.is_open = false;
        }
        LogicCommand::SelectSlot(slot) => inventory.select_slot(slot),
        LogicCommand::NextSlot => inventory.next_slot(),
        LogicCommand::PrevSlot => inventory.prev_slot(),
        LogicCommand::UpdateCursor { aspect, mouse_pos, screen_size } => {
            super::worker_cmd_cursor::handle_update_cursor(
                aspect, mouse_pos, screen_size, aspect_out, mouse_ndc,
                inventory.is_open, current_hovered, mining_state, camera, player, world, aim_dir,
            );
        }
        LogicCommand::MouseAction { button, is_pressed, aspect, mouse_pos, screen_size, is_shift } => {
            super::mouse_actions::handle_mouse_action(
                button, is_pressed, aspect, mouse_pos, screen_size, is_shift,
                inventory, mining_state, world, camera, player, tick_system, block_tx,
                block_entities, open_container,
            );
        }
        LogicCommand::ToggleGameMode => {
            player.game_mode = match player.game_mode {
                crate::engine::GameMode::Survival => crate::engine::GameMode::Creative,
                crate::engine::GameMode::Creative => crate::engine::GameMode::Survival,
                other => other,
            };
            if !player.game_mode.can_fly() { player.is_flying = false; }
        }
        LogicCommand::ToggleFlight => {
            if player.game_mode.can_fly() {
                player.is_flying = !player.is_flying;
                if player.is_flying { player.velocity.y = 0.0; }
            }
        }
        other => {
            super::worker_remote_cmds::handle_remote_command(&other, world, items);
        }
    }
}