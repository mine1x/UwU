use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use crate::world::concurrent_storage::ConcurrentChunkStorage;
use super::persistence::save_dirty_chunks_to_disk;

pub const AUTOSAVE_INTERVAL: Duration = Duration::from_secs(300); // 5 minutes

pub struct WorldAutosaver {
    storage: Arc<ConcurrentChunkStorage>,
    stop: Arc<AtomicBool>,
    handle: Option<thread::JoinHandle<()>>,
}

impl WorldAutosaver {
    pub fn start(storage: Arc<ConcurrentChunkStorage>) -> Self {
        let stop = Arc::new(AtomicBool::new(false));
        let s = stop.clone();
        let st = storage.clone();
        let handle = thread::Builder::new()
            .name("world-autosaver".into())
            .spawn(move || {
                while !s.load(Ordering::Relaxed) {
                    thread::sleep(AUTOSAVE_INTERVAL);
                    if s.load(Ordering::Relaxed) {
                        break;
                    }
                    let written = save_dirty_chunks_to_disk(&st);
                    log::info!("Autosave: flushed {written} chunk(s) to disk");
                }
            })
            .ok();
        Self { storage, stop, handle }
    }

    pub fn flush(&self) -> usize {
        save_dirty_chunks_to_disk(&self.storage)
    }
}

impl Drop for WorldAutosaver {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(h) = self.handle.take() {
            let _ = h.join();
        }
    }
}
