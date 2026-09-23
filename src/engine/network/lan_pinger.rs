use std::net::UdpSocket;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use super::lan_discovery::create_ping_string;

pub struct LanServerPinger {
    running: Arc<AtomicBool>,
}

impl LanServerPinger {
    pub const MULTICAST_GROUP: &str = "224.0.2.60:4445";
    pub const PING_INTERVAL_MS: u64 = 1500;

    pub fn start(motd: String, address: String) -> Self {
        let running = Arc::new(AtomicBool::new(true));
        let flag = running.clone();

        thread::Builder::new()
            .name("LanServerPinger".into())
            .spawn(move || {
                let socket = match UdpSocket::bind("0.0.0.0:0") {
                    Ok(s) => s,
                    Err(e) => {
                        log::warn!("LanServerPinger bind failed: {}", e);
                        return;
                    }
                };
                let _ = socket.set_broadcast(true);
                let msg = create_ping_string(&motd, &address);
                let bytes = msg.as_bytes();

                let port_u16 = address.parse::<u16>().unwrap_or(25565);
                while flag.load(Ordering::Relaxed) {
                    let _ = socket.send_to(bytes, Self::MULTICAST_GROUP);
                    let _ = socket.send_to(bytes, "127.0.0.1:4445");
                    let _ = socket.send_to(bytes, "255.255.255.255:4445");
                    super::local_ipc::register_local_host(&motd, port_u16);
                    thread::sleep(Duration::from_millis(Self::PING_INTERVAL_MS));
                }
            })
            .ok();

        Self { running }
    }

    pub fn stop(&self) {
        self.running.store(false, Ordering::Relaxed);
    }
}

impl Drop for LanServerPinger {
    fn drop(&mut self) {
        self.stop();
    }
}
