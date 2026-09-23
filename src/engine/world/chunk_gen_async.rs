// Background Asynchronous World Generator
// Minecraft Java parity: Uses background worker thread pool to generate chunk columns
// without blocking the main logic tick or render thread.

use std::collections::HashSet;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::Duration;

use super::concurrent_storage::ConcurrentChunkStorage;
use super::levelgen::MinecraftWorldGenerator;

pub struct ChunkGeneratorWorker {
    request_tx: Sender<ChunkGenRequest>,
    completed_rx: Receiver<(i32, i32)>,
    pending_coords: HashSet<(i32, i32)>,
    running: Arc<AtomicBool>,
    _worker_threads: Vec<JoinHandle<()>>,
}

enum ChunkGenRequest {
    GenerateColumn { x: i32, z: i32 },
    UpdateGenerator(MinecraftWorldGenerator),
    ClearPending,
}

impl ChunkGeneratorWorker {
    pub fn new(
        storage: Arc<ConcurrentChunkStorage>,
        generator: MinecraftWorldGenerator,
    ) -> Self {
        let (request_tx, request_rx) = mpsc::channel::<ChunkGenRequest>();
        let (completed_tx, completed_rx) = mpsc::channel::<(i32, i32)>();
        let request_rx = Arc::new(std::sync::Mutex::new(request_rx));
        let running = Arc::new(AtomicBool::new(true));

        let num_threads = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4)
            .min(4)
            .max(2);

        let mut _worker_threads = Vec::with_capacity(num_threads);

        for _ in 0..num_threads {
            let storage = Arc::clone(&storage);
            let mut generator_instance = generator.clone();
            let rx = Arc::clone(&request_rx);
            let tx = completed_tx.clone();
            let run = Arc::clone(&running);

            let handle = thread::Builder::new()
                .name("chunk-gen-worker".into())
                .spawn(move || {
                    while run.load(Ordering::Relaxed) {
                        let req = {
                            let lock = rx.lock().unwrap();
                            lock.recv_timeout(Duration::from_millis(50))
                        };

                        match req {
                            Ok(ChunkGenRequest::GenerateColumn { x, z }) => {
                                // Double check if already generated
                                if !storage.contains_chunk(&(x, 0, z)) {
                                    for y in -4..=8 {
                                        let chunk = generator_instance.generate_chunk(x, y, z);
                                        storage.insert((x, y, z), chunk);
                                    }
                                    let _ = tx.send((x, z));
                                }
                            }
                            Ok(ChunkGenRequest::UpdateGenerator(new_gen)) => {
                                generator_instance = new_gen;
                            }
                            Ok(ChunkGenRequest::ClearPending) => {}
                            Err(mpsc::RecvTimeoutError::Timeout) => {}
                            Err(mpsc::RecvTimeoutError::Disconnected) => break,
                        }
                    }
                })
                .expect("Failed to spawn chunk gen worker thread");

            _worker_threads.push(handle);
        }

        Self {
            request_tx,
            completed_rx,
            pending_coords: HashSet::new(),
            running,
            _worker_threads,
        }
    }

    /// Requests chunk generation for a column (x, z)
    pub fn request_column(&mut self, x: i32, z: i32) {
        if self.pending_coords.insert((x, z)) {
            let _ = self.request_tx.send(ChunkGenRequest::GenerateColumn { x, z });
        }
    }

    /// Polls completed columns from background threads
    /// Returns true if at least one column finished generation
    pub fn poll_completed(&mut self) -> bool {
        let mut completed_any = false;
        loop {
            match self.completed_rx.try_recv() {
                Ok((x, z)) => {
                    self.pending_coords.remove(&(x, z));
                    completed_any = true;
                }
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => break,
            }
        }
        completed_any
    }

    /// Resets/updates generator (e.g. on new seed)
    pub fn update_generator(&mut self, generator: MinecraftWorldGenerator) {
        self.pending_coords.clear();
        let _ = self.request_tx.send(ChunkGenRequest::ClearPending);
        let _ = self.request_tx.send(ChunkGenRequest::UpdateGenerator(generator));
    }
}

impl Drop for ChunkGeneratorWorker {
    fn drop(&mut self) {
        self.running.store(false, Ordering::Relaxed);
    }
}
