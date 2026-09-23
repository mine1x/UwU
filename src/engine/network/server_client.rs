use std::io::BufReader;
use std::net::TcpStream;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use bevy::math::Vec3;
use super::packet_frame::{read_packet_frame, write_packet_frame};
use super::protocol::Packet;
use crate::world::{Chunk, ConcurrentChunkStorage, encode_chunk_runs};

pub fn handle_client(
    stream: TcpStream,
    assigned_id: i32,
    running: Arc<AtomicBool>,
    incoming_packets: Arc<Mutex<Vec<Packet>>>,
    modified_blocks: Arc<Mutex<Vec<(i32, i32, i32, u8)>>>,
    dropped_items: Arc<Mutex<Vec<(i32, Vec3, u8, u32)>>>,
    storage: Arc<Mutex<Option<Arc<ConcurrentChunkStorage>>>>,
) {
    let mut writer = match stream.try_clone() {
        Ok(w) => w,
        Err(_) => return,
    };
    let mut reader = BufReader::new(stream);

    thread::Builder::new()
        .name("LanServerClientHandler".into())
        .spawn(move || {
            while running.load(Ordering::Relaxed) {
                match read_packet_frame(&mut reader) {
                    Ok((packet_id, payload)) => {
                        if let Some(packet) = Packet::decode(packet_id, &payload) {
                            if matches!(packet, Packet::ServerboundHello { .. }) {
                                let login_pkt = Packet::ClientboundLogin { entity_id: assigned_id };
                                let (pid, pld) = login_pkt.encode();
                                let _ = write_packet_frame(&mut writer, pid, &pld);

                                let storage_arc = storage.lock().unwrap().clone();
                                if let Some(st) = &storage_arc {
                                    st.for_each_chunk(|&coords, chunk| {
                                        let c_pkt = chunk_to_packet(coords, chunk);
                                        let (c_pid, c_pld) = c_pkt.encode();
                                        let _ = write_packet_frame(&mut writer, c_pid, &c_pld);
                                    });
                                }

                                let mods = modified_blocks.lock().unwrap().clone();
                                for (x, y, z, bt) in mods {
                                    let b_pkt = Packet::ClientboundBlockUpdate { x, y, z, block_type: bt };
                                    let (b_pid, b_pld) = b_pkt.encode();
                                    let _ = write_packet_frame(&mut writer, b_pid, &b_pld);
                                }

                                let items = dropped_items.lock().unwrap().clone();
                                for (eid, pos, it, count) in items {
                                    let i_pkt = Packet::ClientboundSpawnItem { entity_id: eid, pos, item_type: it, count };
                                    let (i_pid, i_pld) = i_pkt.encode();
                                    let _ = write_packet_frame(&mut writer, i_pid, &i_pld);
                                }
                            }
                            incoming_packets.lock().unwrap().push(packet);
                        }
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut || e.kind() == std::io::ErrorKind::WouldBlock => {
                        thread::sleep(std::time::Duration::from_millis(5));
                        continue;
                    }
                    Err(_) => break,
                }
            }
        })
        .ok();
}

fn chunk_to_packet(coords: (i32, i32, i32), chunk: &Chunk) -> Packet {
    Packet::ClientboundChunkData {
        x: coords.0,
        y: coords.1,
        z: coords.2,
        runs: encode_chunk_runs(chunk),
    }
}