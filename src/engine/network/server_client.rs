use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use super::async_frame::{read_packet_frame_async, write_packet_frame_async};
use super::protocol::Packet;
use crate::world::{Chunk, ConcurrentChunkStorage, encode_chunk_runs};
use tokio::net::TcpStream;
use tokio::sync::mpsc;

pub fn handle_client_async(
    stream: TcpStream,
    assigned_id: i32,
    running: Arc<AtomicBool>,
    incoming_packets: Arc<Mutex<Vec<Packet>>>,
    modified_blocks: Arc<Mutex<Vec<(i32, i32, i32, u8)>>>,
    dropped_items: Arc<Mutex<Vec<(i32, bevy::math::Vec3, u8, u32)>>>,
    storage: Arc<Mutex<Option<Arc<ConcurrentChunkStorage>>>>,
    mut client_out_rx: mpsc::UnboundedReceiver<Packet>,
) {
    let (mut reader, mut writer) = stream.into_split();

    // Outgoing packets writer task
    let r_w = running.clone();
    let (direct_tx, mut direct_rx) = mpsc::unbounded_channel::<Packet>();

    let write_task = tokio::spawn(async move {
        while r_w.load(Ordering::Relaxed) {
            tokio::select! {
                pkt = client_out_rx.recv() => {
                    match pkt {
                        Some(p) => {
                            let (pid, pld) = p.encode();
                            if write_packet_frame_async(&mut writer, pid, &pld).await.is_err() {
                                break;
                            }
                        }
                        None => break,
                    }
                }
                pkt = direct_rx.recv() => {
                    match pkt {
                        Some(p) => {
                            let (pid, pld) = p.encode();
                            if write_packet_frame_async(&mut writer, pid, &pld).await.is_err() {
                                break;
                            }
                        }
                        None => break,
                    }
                }
            }
        }
    });

    // Incoming packets reader task
    let r_r = running.clone();
    let in_p = incoming_packets.clone();
    let read_task = tokio::spawn(async move {
        while r_r.load(Ordering::Relaxed) {
            match read_packet_frame_async(&mut reader).await {
                Ok((packet_id, payload)) => {
                    if let Some(packet) = Packet::decode(packet_id, &payload) {
                        if matches!(packet, Packet::ServerboundHello { .. }) {
                            let login_pkt = Packet::ClientboundLogin { entity_id: assigned_id };
                            let _ = direct_tx.send(login_pkt);

                            let storage_arc = storage.lock().unwrap().clone();
                            if let Some(st) = &storage_arc {
                                let mut chunks_to_send = Vec::new();
                                st.for_each_chunk(|&coords, chunk| {
                                    chunks_to_send.push(chunk_to_packet(coords, chunk));
                                });
                                for c_pkt in chunks_to_send {
                                    let _ = direct_tx.send(c_pkt);
                                }
                            }

                            let mods = modified_blocks.lock().unwrap().clone();
                            for (x, y, z, bt) in mods {
                                let b_pkt = Packet::ClientboundBlockUpdate { x, y, z, block_type: bt };
                                let _ = direct_tx.send(b_pkt);
                            }

                            let items = dropped_items.lock().unwrap().clone();
                            for (eid, pos, it, count) in items {
                                let i_pkt = Packet::ClientboundSpawnItem { entity_id: eid, pos, item_type: it, count };
                                let _ = direct_tx.send(i_pkt);
                            }
                        }
                        in_p.lock().unwrap().push(packet);
                    }
                }
                Err(_) => break,
            }
        }
    });

    tokio::spawn(async move {
        let _ = tokio::join!(write_task, read_task);
    });
}

fn chunk_to_packet(coords: (i32, i32, i32), chunk: &Chunk) -> Packet {
    let runs = encode_chunk_runs(chunk);
    Packet::ClientboundChunkData {
        x: coords.0,
        y: coords.1,
        z: coords.2,
        runs,
    }
}