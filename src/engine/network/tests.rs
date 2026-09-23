#[cfg(test)]
mod tests {
    use crate::network::block_pos_codec::{pack_block_pos, unpack_block_pos};
    use crate::network::lan_discovery::*;
    use crate::network::protocol::Packet;
    use crate::network::varint::{read_varint, write_varint};
    use bevy::math::Vec3;

    #[test]
    fn test_lan_discovery_java_parity() {
        let ping_str = create_ping_string("Steve's World", "25565");
        assert_eq!(ping_str, "[MOTD]Steve's World[/MOTD][AD]25565[/AD]");
        assert_eq!(parse_motd(&ping_str), "Steve's World");
        assert_eq!(parse_address(&ping_str), Some("25565".to_string()));
    }

    #[test]
    fn test_java_varint_and_blockpos_wire_format() {
        let mut buf = Vec::new();
        let _ = write_varint(&mut buf, 25565);
        let decoded = read_varint(&mut std::io::Cursor::new(&buf)).expect("decode");
        assert_eq!(decoded, 25565);

        let packed = pack_block_pos(-123, 64, 456);
        let (x, y, z) = unpack_block_pos(packed);
        assert_eq!((x, y, z), (-123, 64, 456));
    }

    #[test]
    fn test_java_binary_packet_roundtrip() {
        let p_sync = Packet::ClientboundEntityPositionSync {
            entity_id: 42, pos: Vec3::new(10.5, 64.0, -20.25),
            yaw: 1.57, pitch: -0.5, held_item: Some(6),
            is_sneaking: true, is_sprinting: false, mining_swing: 0.85,
        };
        let (pid, payload) = p_sync.encode();
        let decoded = Packet::decode(pid, &payload).expect("decode packet");
        assert_eq!(decoded, p_sync);

        let p_block = Packet::ClientboundBlockUpdate { x: 5, y: 12, z: -8, block_type: 2 };
        let (b_pid, b_payload) = p_block.encode();
        assert_eq!(Packet::decode(b_pid, &b_payload), Some(p_block));

        let p_item = Packet::ClientboundSpawnItem { entity_id: 101, pos: Vec3::new(1.0, 2.0, 3.0), item_type: 2, count: 5 };
        let (i_pid, i_payload) = p_item.encode();
        assert_eq!(Packet::decode(i_pid, &i_payload), Some(p_item));
    }

    #[test]
    fn test_server_client_java_handshake_and_world_sync() {
        use crate::network::{LanServer, LanClient};
        use std::sync::{Arc, Mutex};
        use std::thread;
        use std::time::Duration;

        let storage: Arc<Mutex<Option<Arc<crate::world::ConcurrentChunkStorage>>>> = Arc::new(Mutex::new(None));
        let server = LanServer::bind("SyncTestWorld".into(), 0, storage).expect("bind free port");
        server.record_block_change(10, 20, 30, 4);
        server.record_spawn_item(1001, Vec3::new(5.0, 6.0, 7.0), 3, 2);
        let port = server.port;

        let client = LanClient::connect(&format!("127.0.0.1:{}", port), "TestPlayer").expect("connect");
        thread::sleep(Duration::from_millis(100));

        let c_pkts = client.poll_packets();
        assert!(c_pkts.iter().any(|p| matches!(p, Packet::ClientboundLogin { entity_id: 2 })));
        assert!(c_pkts.iter().any(|p| matches!(p, Packet::ClientboundBlockUpdate { x: 10, y: 20, z: 30, block_type: 4 })));
        assert!(c_pkts.iter().any(|p| matches!(p, Packet::ClientboundSpawnItem { entity_id: 1001, item_type: 3, count: 2, .. })));
    }
}
