use std::collections::HashMap;
use std::sync::RwLock;
use super::chunk::Chunk;

#[derive(bevy::prelude::Resource)]
pub struct ConcurrentChunkStorage {
    shards: [RwLock<HashMap<(i32, i32, i32), Chunk>>; 16],
}

impl ConcurrentChunkStorage {
    pub fn new() -> Self {
        Self {
            shards: std::array::from_fn(|_| RwLock::new(HashMap::new())),
        }
    }

    #[inline]
    fn shard_idx(coords: &(i32, i32, i32)) -> usize {
        let (x, y, z) = *coords;
        let mut h = (x as u64).wrapping_mul(0x9E3779B97F4A7C15);
        h ^= (y as u64).wrapping_mul(0xC2B2AE3D27D4EB4F);
        h ^= (z as u64).wrapping_mul(0x165667B19E3779F9);
        (h as usize) % 16
    }

    pub fn get_chunk_read<F, R>(&self, coords: &(i32, i32, i32), f: F) -> Option<R>
    where
        F: FnOnce(&Chunk) -> R,
    {
        let shard = self.shards[Self::shard_idx(coords)].read().unwrap();
        shard.get(coords).map(f)
    }

    pub fn get_chunk_write<F, R>(&self, coords: &(i32, i32, i32), f: F) -> Option<R>
    where
        F: FnOnce(&mut Chunk) -> R,
    {
        let mut shard = self.shards[Self::shard_idx(coords)].write().unwrap();
        shard.get_mut(coords).map(f)
    }

    pub fn insert(&self, coords: (i32, i32, i32), chunk: Chunk) {
        let mut shard = self.shards[Self::shard_idx(&coords)].write().unwrap();
        shard.insert(coords, chunk);
    }

    pub fn get_all_keys(&self) -> Vec<(i32, i32, i32)> {
        let mut keys = Vec::new();
        for shard in &self.shards {
            let s = shard.read().unwrap();
            keys.extend(s.keys().copied());
        }
        keys
    }

    pub fn for_each_chunk<F>(&self, mut f: F)
    where
        F: FnMut(&(i32, i32, i32), &Chunk),
    {
        for shard in &self.shards {
            let s = shard.read().unwrap();
            for (k, v) in s.iter() {
                f(k, v);
            }
        }
    }

    /// Clones every chunk flagged as needing a disk flush and clears the flag.
    /// The clone is taken so the caller can write to disk without holding a lock;
    /// the original chunk stays in RAM with the flag cleared so later edits
    /// re-mark it for the next flush.
    pub fn drain_disk_dirty(&self) -> Vec<((i32, i32, i32), Chunk)> {
        let mut out = Vec::new();
        for shard in &self.shards {
            let mut s = shard.write().unwrap();
            for (k, chunk) in s.iter_mut() {
                if chunk.disk_dirty {
                    out.push((*k, chunk.clone()));
                    chunk.disk_dirty = false;
                }
            }
        }
        out
    }

    /// Clears the disk-dirty flag on every chunk (used after a full world load).
    pub fn clear_disk_dirty_all(&self) {
        for shard in &self.shards {
            let mut s = shard.write().unwrap();
            for v in s.values_mut() {
                v.disk_dirty = false;
            }
        }
    }
}