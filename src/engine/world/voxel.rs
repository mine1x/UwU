use std::sync::Arc;
use super::block::BlockType;
use super::chunk::Chunk;
use super::chunk_mesh_par::{collect_world_mesh, rebuild_dirty_chunks_multithreaded};
use super::concurrent_storage::ConcurrentChunkStorage;
use super::flat_gen::generate_flat_chunk_data;
use super::island_gen::generate_island_chunk;
use crate::render::types::Vertex;

#[derive(bevy::prelude::Resource)]
pub struct VoxelWorld {
    pub storage: Arc<ConcurrentChunkStorage>,
    pub cached_world_mesh: (Vec<Vertex>, Vec<u32>),
    pub needs_mesh_rebuild: bool,
}

impl VoxelWorld {
    pub fn new() -> Self {
        Self {
            storage: Arc::new(ConcurrentChunkStorage::new()),
            cached_world_mesh: (Vec::new(), Vec::new()),
            needs_mesh_rebuild: true,
        }
    }

    pub fn new_flat(radius: i32) -> Self {
        let mut w = Self::new();
        for cx in -radius..=radius {
            for cz in -radius..=radius {
                let chunk = generate_flat_chunk_data(cx, 0, cz);
                w.storage.insert((cx, 0, cz), chunk);
            }
        }
        w.rebuild_all_dirty_chunks();
        w
    }

    pub fn new_island() -> Self {
        let mut w = Self::new();
        w.storage.insert((0, 0, 0), generate_island_chunk());
        w.rebuild_all_dirty_chunks();
        w
    }

    #[inline]
    pub fn world_to_chunk(gx: i32, gy: i32, gz: i32) -> ((i32, i32, i32), (usize, usize, usize)) {
        let (cx, cy, cz) = (gx.div_euclid(16), gy.div_euclid(16), gz.div_euclid(16));
        let (lx, ly, lz) = (gx.rem_euclid(16) as usize, gy.rem_euclid(16) as usize, gz.rem_euclid(16) as usize);
        ((cx, cy, cz), (lx, ly, lz))
    }

    pub fn get_block(&self, gx: i32, gy: i32, gz: i32) -> BlockType {
        let (c, l) = Self::world_to_chunk(gx, gy, gz);
        self.storage.get_chunk_read(&c, |chunk| chunk.get_block(l.0, l.1, l.2)).unwrap_or(BlockType::Air)
    }

    pub fn mark_chunk_dirty(&self, coords: (i32, i32, i32)) {
        self.storage.get_chunk_write(&coords, |c| c.is_dirty = true);
    }

    pub fn set_block(&mut self, gx: i32, gy: i32, gz: i32, block: BlockType) {
        let (c, l) = Self::world_to_chunk(gx, gy, gz);
        let exists = self.storage.get_chunk_write(&c, |chunk| {
            chunk.set_block(l.0, l.1, l.2, block);
        }).is_some();
        if !exists {
            let mut chunk = Chunk::new(c);
            chunk.set_block(l.0, l.1, l.2, block);
            self.storage.insert(c, chunk);
        }
        self.needs_mesh_rebuild = true;
        if l.0 == 0 { self.mark_chunk_dirty((c.0 - 1, c.1, c.2)); }
        if l.0 == 15 { self.mark_chunk_dirty((c.0 + 1, c.1, c.2)); }
        if l.1 == 0 { self.mark_chunk_dirty((c.0, c.1 - 1, c.2)); }
        if l.1 == 15 { self.mark_chunk_dirty((c.0, c.1 + 1, c.2)); }
        if l.2 == 0 { self.mark_chunk_dirty((c.0, c.1, c.2 - 1)); }
        if l.2 == 15 { self.mark_chunk_dirty((c.0, c.1, c.2 + 1)); }
    }

    pub fn remove_block(&mut self, gx: i32, gy: i32, gz: i32) {
        self.set_block(gx, gy, gz, BlockType::Air);
    }

    pub fn for_each_chunk<F: FnMut(&(i32, i32, i32), &Chunk)>(&self, f: F) {
        self.storage.for_each_chunk(f);
    }

    pub fn rebuild_all_dirty_chunks(&mut self) {
        let mut dirty_keys = Vec::new();
        self.storage.for_each_chunk(|&k, c| {
            if c.is_dirty { dirty_keys.push(k); }
        });
        if dirty_keys.is_empty() && !self.needs_mesh_rebuild { return; }

        rebuild_dirty_chunks_multithreaded(&self.storage, dirty_keys);
        self.cached_world_mesh = collect_world_mesh(&self.storage);
        self.needs_mesh_rebuild = false;
    }
}