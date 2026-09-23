use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use super::lan_pinger::LanServerPinger;
use super::protocol::Packet;
use super::server_client::handle_client_async;
use crate::world::ConcurrentChunkStorage;
use tokio::net::TcpListener;
use tokio::sync::mpsc;

pub struct LanServer {
    pub port: u16,
    running: Arc<AtomicBool>,
    _pinger: LanServerPinger,
    incoming: Arc<Mutex<Vec<Packet>>>,
    client_senders: Arc<Mutex<Vec<mpsc::UnboundedSender<Packet>>>>,
    pub modified_blocks: Arc<Mutex<Vec<(i32, i32, i32, u8)>>>,
    pub dropped_items: Arc<Mutex<Vec<(i32, bevy::math::Vec3, u8, u32)>>>,
}

impl LanServer {
    pub fn bind(
        motd: String,
        port: u16,
        storage: Arc<Mutex<Option<Arc<ConcurrentChunkStorage>>>>,
    ) -> Result<Self, std::io::Error> {
        let rt = super::runtime::get_network_runtime();

        let (actual_port, listener) = rt.block_on(async {
            let res: Result<(u16, TcpListener), std::io::Error> = match TcpListener::bind(("0.0.0.0", port)).await {
                Ok(l) => {
                    let p = l.local_addr()?.port();
                    Ok((p, l))
                }
                Err(_) => {
                    let l = TcpListener::bind(("0.0.0.0", 0)).await?;
                    let p = l.local_addr()?.port();
                    Ok((p, l))
                }
            };
            res
        })?;

        let pinger = LanServerPinger::start(motd, actual_port.to_string());
        let running = Arc::new(AtomicBool::new(true));
        let incoming = Arc::new(Mutex::new(Vec::new()));
        let client_senders = Arc::new(Mutex::new(Vec::<mpsc::UnboundedSender<Packet>>::new()));
        let modified_blocks = Arc::new(Mutex::new(Vec::new()));
        let dropped_items = Arc::new(Mutex::new(Vec::new()));

        let (r_c, in_c, cl_c, mb_c, di_c) = (
            running.clone(),
            incoming.clone(),
            client_senders.clone(),
            modified_blocks.clone(),
            dropped_items.clone(),
        );
        let storage_c = storage.clone();

        rt.spawn(async move {
            let mut next_id = 2;
            while r_c.load(Ordering::Relaxed) {
                match listener.accept().await {
                    Ok((stream, _)) => {
                        let (tx, rx) = mpsc::unbounded_channel::<Packet>();
                        cl_c.lock().unwrap().push(tx);
                        handle_client_async(
                            stream,
                            next_id,
                            r_c.clone(),
                            in_c.clone(),
                            mb_c.clone(),
                            di_c.clone(),
                            storage_c.clone(),
                            rx,
                        );
                        next_id += 1;
                    }
                    Err(_) => {
                        tokio::time::sleep(std::time::Duration::from_millis(20)).await;
                    }
                }
            }
        });

        Ok(Self {
            port: actual_port,
            running,
            _pinger: pinger,
            incoming,
            client_senders,
            modified_blocks,
            dropped_items,
        })
    }

    pub fn poll_packets(&self) -> Vec<Packet> {
        let mut q = self.incoming.lock().unwrap();
        std::mem::take(&mut *q)
    }

    pub fn record_block_change(&self, x: i32, y: i32, z: i32, bt: u8) {
        let mut list = self.modified_blocks.lock().unwrap();
        if let Some(entry) = list.iter_mut().find(|(bx, by, bz, _)| *bx == x && *by == y && *bz == z) {
            entry.3 = bt;
        } else {
            list.push((x, y, z, bt));
        }
    }

    pub fn record_spawn_item(&self, id: i32, pos: bevy::math::Vec3, it: u8, count: u32) {
        let mut list = self.dropped_items.lock().unwrap();
        list.retain(|(eid, _, _, _)| *eid != id);
        list.push((id, pos, it, count));
    }

    pub fn record_remove_entities(&self, ids: &[i32]) {
        let mut list = self.dropped_items.lock().unwrap();
        list.retain(|(eid, _, _, _)| !ids.contains(eid));
    }

    pub fn broadcast(&self, packet: &Packet) {
        let mut list = self.client_senders.lock().unwrap();
        list.retain(|tx| tx.send(packet.clone()).is_ok());
    }
}

impl Drop for LanServer {
    fn drop(&mut self) {
        self.running.store(false, Ordering::Relaxed);
    }
}
