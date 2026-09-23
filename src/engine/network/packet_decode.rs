use super::byte_buf::FriendlyByteBufReader;
use super::packet_types::*;
use super::protocol::Packet;
use bevy::math::Vec3;

pub fn decode_packet(packet_id: i32, payload: &[u8]) -> Option<Packet> {
    let mut r = FriendlyByteBufReader::new(payload);
    match packet_id {
        CB_LOGIN => Some(Packet::ClientboundLogin { entity_id: r.read_varint().ok()? }),
        CB_ADD_ENTITY => Some(Packet::ClientboundAddEntity {
            entity_id: r.read_varint().ok()?,
            x: r.read_double().ok()?, y: r.read_double().ok()?, z: r.read_double().ok()?,
            yaw: r.read_float().ok()?, pitch: r.read_float().ok()?,
        }),
        CB_ENTITY_POSITION_SYNC => {
            let entity_id = r.read_varint().ok()?;
            let x = r.read_double().ok()? as f32; let y = r.read_double().ok()? as f32; let z = r.read_double().ok()? as f32;
            let yaw = r.read_float().ok()?; let pitch = r.read_float().ok()?;
            let is_sneaking = r.read_bool().ok()?; let is_sprinting = r.read_bool().ok()?;
            let mining_swing = r.read_float().ok()?;
            let code = r.read_varint().ok()?;
            Some(Packet::ClientboundEntityPositionSync {
                entity_id, pos: Vec3::new(x, y, z), yaw, pitch,
                held_item: if code >= 0 { Some(code as u8) } else { None },
                is_sneaking, is_sprinting, mining_swing,
            })
        }
        CB_BLOCK_UPDATE => {
            let (x, y, z) = r.read_block_pos().ok()?;
            let block_type = r.read_varint().ok()? as u8;
            Some(Packet::ClientboundBlockUpdate { x, y, z, block_type })
        }
        CB_CHUNK_DATA => {
            let x = r.read_varint().ok()?;
            let y = r.read_varint().ok()?;
            let z = r.read_varint().ok()?;
            let run_count = r.read_varint().ok()? as usize;
            let mut runs = Vec::with_capacity(run_count);
            for _ in 0..run_count {
                let count = r.read_varint().ok()? as u16;
                let bt = r.read_byte().ok()?;
                runs.push((count, bt));
            }
            Some(Packet::ClientboundChunkData { x, y, z, runs })
        }
        CB_REMOVE_ENTITIES => {
            let count = r.read_varint().ok()? as usize;
            let mut entity_ids = Vec::with_capacity(count);
            for _ in 0..count { entity_ids.push(r.read_varint().ok()?); }
            Some(Packet::ClientboundRemoveEntities { entity_ids })
        }
        CB_SPAWN_ITEM => {
            let entity_id = r.read_varint().ok()?;
            let x = r.read_double().ok()? as f32;
            let y = r.read_double().ok()? as f32;
            let z = r.read_double().ok()? as f32;
            let item_type = r.read_byte().ok()?;
            let count = r.read_varint().ok()? as u32;
            Some(Packet::ClientboundSpawnItem { entity_id, pos: Vec3::new(x, y, z), item_type, count })
        }
        SB_HELLO => Some(Packet::ServerboundHello { name: r.read_utf().ok()? }),
        SB_MOVE_PLAYER_POS_ROT => {
            let x = r.read_double().ok()? as f32; let y = r.read_double().ok()? as f32; let z = r.read_double().ok()? as f32;
            let yaw = r.read_float().ok()?; let pitch = r.read_float().ok()?;
            let is_sneaking = r.read_bool().ok()?; let is_sprinting = r.read_bool().ok()?;
            let mining_swing = r.read_float().ok()?;
            let code = r.read_varint().ok()?;
            Some(Packet::ServerboundMovePlayerPosRot {
                pos: Vec3::new(x, y, z), yaw, pitch,
                held_item: if code >= 0 { Some(code as u8) } else { None },
                is_sneaking, is_sprinting, mining_swing,
            })
        }
        SB_PLAYER_ACTION => {
            let action = r.read_varint().ok()?;
            let (x, y, z) = r.read_block_pos().ok()?;
            Some(Packet::ServerboundPlayerAction { action, x, y, z })
        }
        SB_USE_ITEM_ON => {
            let (x, y, z) = r.read_block_pos().ok()?;
            let block_type = r.read_byte().ok()?;
            Some(Packet::ServerboundUseItemOn { x, y, z, block_type })
        }
        _ => None,
    }
}
