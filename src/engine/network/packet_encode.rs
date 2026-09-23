use super::byte_buf::FriendlyByteBuf;
use super::packet_types::*;
use super::protocol::Packet;

pub fn encode_packet(packet: &Packet) -> (i32, Vec<u8>) {
    let mut buf = FriendlyByteBuf::new();
    match packet {
        Packet::ClientboundLogin { entity_id } => {
            buf.write_varint(*entity_id);
            (CB_LOGIN, buf.data)
        }
        Packet::ClientboundAddEntity { entity_id, x, y, z, yaw, pitch } => {
            buf.write_varint(*entity_id);
            buf.write_double(*x); buf.write_double(*y); buf.write_double(*z);
            buf.write_float(*yaw); buf.write_float(*pitch);
            (CB_ADD_ENTITY, buf.data)
        }
        Packet::ClientboundEntityPositionSync { entity_id, pos, yaw, pitch, held_item, is_sneaking, is_sprinting, mining_swing } => {
            buf.write_varint(*entity_id);
            buf.write_double(pos.x as f64); buf.write_double(pos.y as f64); buf.write_double(pos.z as f64);
            buf.write_float(*yaw); buf.write_float(*pitch);
            buf.write_bool(*is_sneaking); buf.write_bool(*is_sprinting);
            buf.write_float(*mining_swing);
            buf.write_varint(held_item.map(|v| v as i32).unwrap_or(-1));
            (CB_ENTITY_POSITION_SYNC, buf.data)
        }
        Packet::ClientboundBlockUpdate { x, y, z, block_type } => {
            buf.write_block_pos(*x, *y, *z);
            buf.write_varint(*block_type as i32);
            (CB_BLOCK_UPDATE, buf.data)
        }
        Packet::ClientboundChunkData { x, y, z, runs } => {
            buf.write_varint(*x); buf.write_varint(*y); buf.write_varint(*z);
            buf.write_varint(runs.len() as i32);
            for &(count, bt) in runs {
                buf.write_varint(count as i32);
                buf.write_byte(bt);
            }
            (CB_CHUNK_DATA, buf.data)
        }
        Packet::ClientboundRemoveEntities { entity_ids } => {
            buf.write_varint(entity_ids.len() as i32);
            for &id in entity_ids { buf.write_varint(id); }
            (CB_REMOVE_ENTITIES, buf.data)
        }
        Packet::ClientboundSpawnItem { entity_id, pos, item_type, count } => {
            buf.write_varint(*entity_id);
            buf.write_double(pos.x as f64); buf.write_double(pos.y as f64); buf.write_double(pos.z as f64);
            buf.write_byte(*item_type);
            buf.write_varint(*count as i32);
            (CB_SPAWN_ITEM, buf.data)
        }
        Packet::ServerboundHello { name } => {
            buf.write_utf(name);
            (SB_HELLO, buf.data)
        }
        Packet::ServerboundMovePlayerPosRot { pos, yaw, pitch, held_item, is_sneaking, is_sprinting, mining_swing } => {
            buf.write_double(pos.x as f64); buf.write_double(pos.y as f64); buf.write_double(pos.z as f64);
            buf.write_float(*yaw); buf.write_float(*pitch);
            buf.write_bool(*is_sneaking); buf.write_bool(*is_sprinting);
            buf.write_float(*mining_swing);
            buf.write_varint(held_item.map(|v| v as i32).unwrap_or(-1));
            (SB_MOVE_PLAYER_POS_ROT, buf.data)
        }
        Packet::ServerboundPlayerAction { action, x, y, z } => {
            buf.write_varint(*action);
            buf.write_block_pos(*x, *y, *z);
            (SB_PLAYER_ACTION, buf.data)
        }
        Packet::ServerboundUseItemOn { x, y, z, block_type } => {
            buf.write_block_pos(*x, *y, *z);
            buf.write_byte(*block_type);
            (SB_USE_ITEM_ON, buf.data)
        }
    }
}
