use winit::event::MouseButton;
use crate::engine::{Camera, ContainerRef, Inventory, MiningState, Player, TickPriority, TickSystem};
use crate::engine::container; 
use crate::core::inventory::{get_clicked_creative_slot, handle_creative_slot_click, HudZone};
use crate::world::{BlockType, FluidSimulator, VoxelWorld};

pub fn handle_mouse_action(
    button: MouseButton,
    is_pressed: bool,
    aspect: f32,
    mouse_pos: (f32, f32),
    screen_size: (f32, f32),
    is_shift: bool,
    inventory: &mut Inventory,
    mining_state: &mut MiningState,
    world: &mut VoxelWorld,
    camera: &Camera,
    player: &Player,
    tick_system: &mut TickSystem<BlockType>,
    block_tx: &std::sync::mpsc::Sender<crate::network::Packet>,
    block_entities: &mut crate::engine::BlockEntityManager,
    open_container: &mut Option<ContainerRef>,
) {
    let in_hud = HudZone::is_point_in_hud(mouse_pos.0, mouse_pos.1, screen_size.0, screen_size.1, inventory.is_open);
    if in_hud {
        mining_state.is_holding_left = false;
        mining_state.set_mining(None, false);
        if !is_pressed { return; }
        if inventory.is_open {
            if let Some(container_ref) = *open_container {
                // Container slot click
                if let Some(target) = crate::engine::container::get_clicked_container_slot(
                    mouse_pos.0, mouse_pos.1, screen_size.0, screen_size.1, container_ref.kind,
                ) {
                    container::apply_container_click(inventory, block_entities, &container_ref, target, button == MouseButton::Right, is_shift);
                }
            } else if player.game_mode.is_creative() {
                if let Some(act) = get_clicked_creative_slot(mouse_pos.0, mouse_pos.1, screen_size.0, screen_size.1) {
                    handle_creative_slot_click(inventory, act, button == MouseButton::Right, is_shift);
                }
            } else if let Some(slot) = HudZone::get_clicked_inventory_slot(mouse_pos.0, mouse_pos.1, screen_size.0, screen_size.1) {
                inventory.click_slot(slot, button == MouseButton::Right, is_shift);
            }
        } else if button == MouseButton::Left {
            if let Some(slot) = HudZone::get_clicked_hotbar_slot(mouse_pos.0, mouse_pos.1, screen_size.0, screen_size.1) {
                inventory.select_slot(slot);
            }
        }
    } else {
        let (ro, rd) = camera.screen_to_ray(player.position, aspect, mouse_pos.0, mouse_pos.1, screen_size.0, screen_size.1);
        let hit = crate::world::raycast_world_precise(world, ro, rd, 150.0);
        match button {
            MouseButton::Left => {
                mining_state.is_holding_left = is_pressed;
                let target = hit.map(|(hit_b, _, _)| hit_b);
                mining_state.set_mining(target, is_pressed);
            }
            MouseButton::Right => {
                if is_pressed {
                    if let Some((hit_b, place_p, _)) = hit {
                        let block_type = world.get_block(hit_b.0, hit_b.1, hit_b.2);
                        // Priority: interact with container block > place block
                        if let Some(kind) = container::container_kind_from_block(block_type) {
                            *open_container = Some(ContainerRef { kind, pos: hit_b });
                            inventory.is_open = true;
                            inventory.carried_item = None;
                            mining_state.is_holding_left = false;
                            mining_state.set_mining(None, false);
                        } else {
                            let block_to_place = if player.game_mode.is_creative() {
                                inventory.hotbar[inventory.selected_slot].as_ref().and_then(|s| match s.item {
                                    crate::engine::items::ItemType::Block(b) => Some(b),
                                    crate::engine::items::ItemType::WaterBucket => Some(BlockType::WaterSource),
                                    _ => None,
                                })
                            } else {
                                inventory.consume_selected()
                            };
                            if let Some(selected) = block_to_place {
                                world.set_block(place_p.0, place_p.1, place_p.2, selected);
                                let _ = block_tx.send(crate::network::Packet::ClientboundBlockUpdate {
                                    x: place_p.0, y: place_p.1, z: place_p.2,
                                    block_type: crate::world::block_to_u8(&selected),
                                });
                                if selected.is_fluid() {
                                    tick_system.schedule_tick(selected, place_p, FluidSimulator::WATER_TICK_DELAY, TickPriority::Normal);
                                }
                                FluidSimulator::schedule_neighbors(world, tick_system, place_p.0, place_p.1, place_p.2);
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
}
