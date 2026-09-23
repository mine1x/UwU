use super::app_state::App;
use crate::network::{Packet, RemotePlayer};
use crate::engine::items::{item_from_u8, item_to_u8};
use std::time::Instant;

pub fn update_lan_network(app: &mut App) {
    let local_id = app.local_player_id as i32;
    let local_sync = app.latest_snapshot.lock().unwrap().as_ref().map(|s| {
        let held = s.hotbar_items[s.selected_slot].map(|(it, _)| item_to_u8(&it));
        Packet::ClientboundEntityPositionSync {
            entity_id: local_id, pos: s.player_pos, yaw: s.player_yaw, pitch: 0.0,
            held_item: held, is_sneaking: s.is_sneaking,
            is_sprinting: s.is_sprinting, mining_swing: s.mining_swing,
        }
    });

    let mut local_blocks = Vec::new();
    if let Some(ref rx) = app.block_event_rx {
        while let Ok(pkt) = rx.try_recv() { local_blocks.push(pkt); }
    }

    let mut server_in = Vec::new();
    if let Some(srv) = &app.lan_server {
        if let Some(ref pkt) = local_sync { srv.broadcast(pkt); }
        for bpkt in &local_blocks {
            srv.broadcast(bpkt);
            record_server_event(srv, bpkt);
        }
        for pkt in srv.poll_packets() {
            srv.broadcast(&pkt);
            record_server_event(srv, &pkt);
            server_in.push(pkt);
        }
    }
    for pkt in server_in { handle_network_packet(app, pkt); }

    let mut client_in = Vec::new();
    if let Some(cli) = &app.lan_client {
        if let Some(ref pkt) = local_sync { cli.send(pkt); }
        for bpkt in &local_blocks { cli.send(bpkt); }
        client_in = cli.poll_packets();
    }
    for pkt in client_in { handle_network_packet(app, pkt); }

    let now = Instant::now();
    app.remote_players.retain(|p| now.duration_since(p.last_seen).as_secs() < 3);
}

fn record_server_event(srv: &crate::network::LanServer, pkt: &Packet) {
    match pkt {
        Packet::ClientboundBlockUpdate { x, y, z, block_type } => srv.record_block_change(*x, *y, *z, *block_type),
        Packet::ClientboundSpawnItem { entity_id, pos, item_type, count } => srv.record_spawn_item(*entity_id, *pos, *item_type, *count),
        Packet::ClientboundRemoveEntities { entity_ids } => srv.record_remove_entities(entity_ids),
        _ => {}
    }
}

fn handle_network_packet(app: &mut App, pkt: Packet) {
    match pkt {
        Packet::ClientboundLogin { entity_id } => {
            app.local_player_id = entity_id as u64;
        }
        Packet::ClientboundEntityPositionSync { entity_id, pos, yaw, pitch: _, held_item, is_sneaking, is_sprinting, mining_swing } => {
            if entity_id == app.local_player_id as i32 { return; }
            let id = entity_id as u64;
            let it = held_item.and_then(item_from_u8);
            if let Some(rp) = app.remote_players.iter_mut().find(|p| p.id == id) {
                rp.pos = pos; rp.yaw = yaw; rp.held_item = it;
                rp.is_sneaking = is_sneaking; rp.is_sprinting = is_sprinting;
                rp.mining_swing = mining_swing; rp.last_seen = Instant::now();
            } else {
                let mut rp = RemotePlayer::new(id, format!("Player_{}", id), pos);
                rp.yaw = yaw; rp.held_item = it; rp.is_sneaking = is_sneaking;
                rp.is_sprinting = is_sprinting; rp.mining_swing = mining_swing;
                app.remote_players.push(rp);
            }
        }
        Packet::ClientboundBlockUpdate { x, y, z, block_type } => {
            if let Some(ref tx) = app.command_tx { let _ = tx.send(crate::input::LogicCommand::RemoteBlockChange { x, y, z, block_type }); }
        }
        Packet::ClientboundChunkData { x, y, z, runs } => {
            if let Some(ref tx) = app.command_tx { let _ = tx.send(crate::input::LogicCommand::RemoteChunkData { x, y, z, runs }); }
        }
        Packet::ClientboundSpawnItem { entity_id, pos, item_type, count } => {
            if let Some(ref tx) = app.command_tx { let _ = tx.send(crate::input::LogicCommand::RemoteSpawnItem { entity_id, pos, item_type, count }); }
        }
        Packet::ClientboundRemoveEntities { entity_ids } => {
            app.remote_players.retain(|p| !entity_ids.contains(&(p.id as i32)));
            if let Some(ref tx) = app.command_tx { let _ = tx.send(crate::input::LogicCommand::RemoteRemoveEntities { entity_ids }); }
        }
        _ => {}
    }
}
