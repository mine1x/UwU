use super::block::BlockType;
use super::chunk::{Chunk, CHUNK_SIZE_X, CHUNK_SIZE_Z};

pub fn generate_flat_chunk_data(cx: i32, cy: i32, cz: i32) -> Chunk {
    let mut chunk = Chunk::new((cx, cy, cz));
    if cy == 0 {
        for x in 0..CHUNK_SIZE_X {
            for z in 0..CHUNK_SIZE_Z {
                chunk.set_block(x, 0, z, BlockType::Stone);
                chunk.set_block(x, 1, z, BlockType::Stone);
                chunk.set_block(x, 2, z, BlockType::Dirt);
                chunk.set_block(x, 3, z, BlockType::Dirt);
                chunk.set_block(x, 4, z, BlockType::Grass);
            }
        }
    }
    chunk
}
