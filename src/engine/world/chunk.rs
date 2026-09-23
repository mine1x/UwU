use super::block::BlockType;
use super::chunk_section::{LevelChunkSection, SECTION_SIZE};
use super::chunk_status::ChunkStatus;
use crate::render::types::Vertex;

pub const CHUNK_SIZE_X: usize = SECTION_SIZE;
pub const CHUNK_SIZE_Y: usize = SECTION_SIZE;
pub const CHUNK_SIZE_Z: usize = SECTION_SIZE;

#[derive(bevy::prelude::Component, Clone)]
pub struct Chunk {
    pub coords: (i32, i32, i32),
    pub section: LevelChunkSection,
    pub status: ChunkStatus,
    pub is_dirty: bool,
    /// Set when block data was modified in RAM but not yet flushed to disk.
    pub disk_dirty: bool,
    pub mesh: (Vec<Vertex>, Vec<u32>),
}

impl Chunk {
    pub fn new(coords: (i32, i32, i32)) -> Self {
        Self {
            coords,
            section: LevelChunkSection::new(coords.1),
            status: ChunkStatus::Full,
            is_dirty: true,
            disk_dirty: false,
            mesh: (Vec::new(), Vec::new()),
        }
    }

    #[inline]
    pub fn get_block(&self, x: usize, y: usize, z: usize) -> BlockType {
        self.section.get_block_state(x, y, z)
    }

    #[inline]
    pub fn set_block(&mut self, x: usize, y: usize, z: usize, block: BlockType) {
        let old = self.section.set_block_state(x, y, z, block);
        if old != block {
            self.is_dirty = true;
            self.disk_dirty = true;
        }
    }
}