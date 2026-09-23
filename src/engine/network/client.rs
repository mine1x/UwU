use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use super::async_frame::{read_packet_frame_async, write_packet_frame_async};
use super::protocol::Packet;
use tokio::sync::mpsc;

pub struct LanClient {
    tx: mpsc::UnboundedSender<Packet>,
    incoming: Arc<Mutex<Vec<Packet>>>,
    running: Arc<AtomicBool>,
}

impl LanClient {
    pub fn connect(addr: &str, player_name: &str) -> Result<Self, std::io::Error> {
        let addr_owned = addr.to_string();
        let name_owned = player_name.to_string();
        let (tx, mut rx) = mpsc::unbounded_channel::<Packet>();
        let incoming = Arc::new(Mutex::new(Vec::new()));
        let running = Arc::new(AtomicBool::new(true));

        let (in_c, r_c) = (incoming.clone(), running.clone());
        let rt = super::runtime::get_network_runtime();

        let (ready_tx, ready_rx) = std::sync::mpsc::channel();

        rt.spawn(async move {
            let stream = match tokio::net::TcpStream::connect(&addr_owned).await {
                Ok(s) => {
                    let _ = ready_tx.send(Ok(()));
                    s
                }
                Err(e) => {
                    let _ = ready_tx.send(Err(e));
                    return;
                }
            };

            let (mut reader, mut writer) = stream.into_split();

            // Spawn write task
            let r_w = r_c.clone();
            let write_task = tokio::spawn(async move {
                while r_w.load(Ordering::Relaxed) {
                    tokio::select! {
                        pkt = rx.recv() => {
                            match pkt {
                                Some(packet) => {
                                    let (pid, pld) = packet.encode();
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

            // Spawn read task
            let r_r = r_c.clone();
            let in_r = in_c.clone();
            let read_task = tokio::spawn(async move {
                while r_r.load(Ordering::Relaxed) {
                    match read_packet_frame_async(&mut reader).await {
                        Ok((packet_id, payload)) => {
                            if let Some(packet) = Packet::decode(packet_id, &payload) {
                                in_r.lock().unwrap().push(packet);
                            }
                        }
                        Err(_) => break,
                    }
                }
            });

            let _ = tokio::join!(write_task, read_task);
        });

        match ready_rx.recv_timeout(std::time::Duration::from_secs(3)) {
            Ok(Ok(())) => {
                let client = Self { tx, incoming, running };
                client.send(&Packet::ServerboundHello { name: name_owned });
                Ok(client)
            }
            Ok(Err(e)) => Err(e),
            Err(_) => Err(std::io::Error::new(
                std::io::ErrorKind::TimedOut,
                "Timed out connecting to LAN server",
            )),
        }
    }

    pub fn send(&self, packet: &Packet) {
        let _ = self.tx.send(packet.clone());
    }

    pub fn poll_packets(&self) -> Vec<Packet> {
        let mut q = self.incoming.lock().unwrap();
        std::mem::take(&mut *q)
    }
}

impl Drop for LanClient {
    fn drop(&mut self) {
        self.running.store(false, Ordering::Relaxed);
    }
}
