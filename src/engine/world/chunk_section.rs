use super::block::BlockType;

pub const SECTION_SIZE: usize = 16;
pub const SECTION_BLOCK_COUNT: usize = SECTION_SIZE * SECTION_SIZE * SECTION_SIZE;

#[derive(Clone)]
pub struct LevelChunkSection {
    pub y_index: i32,
    pub blocks: Box<[BlockType; SECTION_BLOCK_COUNT]>,
    pub non_empty_block_count: i16,
    pub fluid_count: i16,
}

impl LevelChunkSection {
    pub fn new(y_index: i32) -> Self {
        let blocks = vec![BlockType::Air; SECTION_BLOCK_COUNT]
            .into_boxed_slice()
            .try_into()
            .unwrap();
        Self { y_index, blocks, non_empty_block_count: 0, fluid_count: 0 }
    }

    #[inline]
    fn index(x: usize, y: usize, z: usize) -> usize {
        x + y * SECTION_SIZE + z * SECTION_SIZE * SECTION_SIZE
    }

    pub fn get_block_state(&self, x: usize, y: usize, z: usize) -> BlockType {
        if x < SECTION_SIZE && y < SECTION_SIZE && z < SECTION_SIZE {
            self.blocks[Self::index(x, y, z)]
        } else {
            BlockType::Air
        }
    }

    pub fn set_block_state(&mut self, x: usize, y: usize, z: usize, state: BlockType) -> BlockType {
        if x >= SECTION_SIZE || y >= SECTION_SIZE || z >= SECTION_SIZE {
            return BlockType::Air;
        }
        let idx = Self::index(x, y, z);
        let old = self.blocks[idx];
        if old != state {
            if old != BlockType::Air { self.non_empty_block_count -= 1; }
            if old.is_fluid() { self.fluid_count -= 1; }
            if state != BlockType::Air { self.non_empty_block_count += 1; }
            if state.is_fluid() { self.fluid_count += 1; }
            self.blocks[idx] = state;
        }
        old
    }

    pub fn is_empty(&self) -> bool {
        self.non_empty_block_count == 0
    }
}