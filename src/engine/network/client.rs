use std::io::BufReader;
use std::net::TcpStream;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use super::packet_frame::{read_packet_frame, write_packet_frame};
use super::protocol::Packet;

pub struct LanClient {
    stream_writer: Arc<Mutex<TcpStream>>,
    incoming: Arc<Mutex<Vec<Packet>>>,
    running: Arc<AtomicBool>,
}

impl LanClient {
    pub fn connect(addr: &str, player_name: &str) -> Result<Self, std::io::Error> {
        let stream = TcpStream::connect(addr)?;
        let _ = stream.set_read_timeout(Some(std::time::Duration::from_millis(150)));
        let _ = stream.set_write_timeout(Some(std::time::Duration::from_millis(500)));
        let reader_stream = stream.try_clone()?;
        let stream_writer = Arc::new(Mutex::new(stream));
        let incoming = Arc::new(Mutex::new(Vec::new()));
        let running = Arc::new(AtomicBool::new(true));

        let (r_c, in_c) = (running.clone(), incoming.clone());

        thread::Builder::new().name("LanClientReceiver".into()).spawn(move || {
            let mut reader = BufReader::new(reader_stream);
            while r_c.load(Ordering::Relaxed) {
                match read_packet_frame(&mut reader) {
                    Ok((packet_id, payload)) => {
                        if let Some(packet) = Packet::decode(packet_id, &payload) {
                            in_c.lock().unwrap().push(packet);
                        }
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut || e.kind() == std::io::ErrorKind::WouldBlock => {
                        thread::sleep(std::time::Duration::from_millis(5));
                        continue;
                    }
                    Err(_) => break,
                }
            }
        }).ok();

        let client = Self { stream_writer, incoming, running };
        client.send(&Packet::ServerboundHello { name: player_name.to_string() });
        Ok(client)
    }

    pub fn send(&self, packet: &Packet) {
        let (pid, pld) = packet.encode();
        let mut w = self.stream_writer.lock().unwrap();
        let _ = write_packet_frame(&mut *w, pid, &pld);
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
