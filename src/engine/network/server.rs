use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use super::lan_pinger::LanServerPinger;
use super::packet_frame::write_packet_frame;
use super::protocol::Packet;
use super::server_client::handle_client;
use crate::world::ConcurrentChunkStorage;

pub struct LanServer {
    pub port: u16,
    running: Arc<AtomicBool>,
    _pinger: LanServerPinger,
    incoming: Arc<Mutex<Vec<Packet>>>,
    clients: Arc<Mutex<Vec<TcpStream>>>,
    pub modified_blocks: Arc<Mutex<Vec<(i32, i32, i32, u8)>>>,
    pub dropped_items: Arc<Mutex<Vec<(i32, bevy::math::Vec3, u8, u32)>>>,
}

impl LanServer {
    pub fn bind(
        motd: String,
        port: u16,
        storage: Arc<Mutex<Option<Arc<ConcurrentChunkStorage>>>>,
    ) -> Result<Self, std::io::Error> {
        let listener = TcpListener::bind(("0.0.0.0", port))
            .or_else(|_| TcpListener::bind(("0.0.0.0", 0)))?;
        let _ = listener.set_nonblocking(true);
        let actual_port = listener.local_addr()?.port();
        let pinger = LanServerPinger::start(motd, actual_port.to_string());

        let running = Arc::new(AtomicBool::new(true));
        let incoming = Arc::new(Mutex::new(Vec::new()));
        let clients = Arc::new(Mutex::new(Vec::<TcpStream>::new()));
        let modified_blocks = Arc::new(Mutex::new(Vec::new()));
        let dropped_items = Arc::new(Mutex::new(Vec::new()));

        let (r_c, in_c, cl_c, mb_c, di_c) = (running.clone(), incoming.clone(), clients.clone(), modified_blocks.clone(), dropped_items.clone());
        let storage_c = storage.clone();

        thread::Builder::new().name("LanServerAcceptor".into()).spawn(move || {
            let mut next_id = 2;
            while r_c.load(Ordering::Relaxed) {
                if let Ok((stream, _)) = listener.accept() {
                    let _ = stream.set_nonblocking(false);
                    let _ = stream.set_read_timeout(Some(Duration::from_millis(150)));
                    let _ = stream.set_write_timeout(Some(Duration::from_millis(500)));
                    if let Ok(writer_stream) = stream.try_clone() {
                        cl_c.lock().unwrap().push(writer_stream);
                    }
                    handle_client(stream, next_id, r_c.clone(), in_c.clone(), mb_c.clone(), di_c.clone(), storage_c.clone());
                    next_id += 1;
                }
                thread::sleep(Duration::from_millis(10));
            }
        }).ok();

        Ok(Self { port: actual_port, running, _pinger: pinger, incoming, clients, modified_blocks, dropped_items })
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
        let (pid, pld) = packet.encode();
        let mut list = self.clients.lock().unwrap();
        list.retain_mut(|stream| write_packet_frame(stream, pid, &pld).is_ok());
    }
}

impl Drop for LanServer {
    fn drop(&mut self) {
        self.running.store(false, Ordering::Relaxed);
    }
}
