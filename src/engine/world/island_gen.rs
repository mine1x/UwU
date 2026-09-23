use super::block::BlockType;
use super::chunk::Chunk;

pub const ISLAND_X: i32 = 5;
pub const ISLAND_Y: i32 = 4;
pub const ISLAND_Z: i32 = 5;
pub const ISLAND_OX: i32 = 5;
pub const ISLAND_OY: i32 = 0;
pub const ISLAND_OZ: i32 = 5;

pub fn generate_island_chunk() -> Chunk {
    let mut chunk = Chunk::new((0, 0, 0));
    for x in 0..ISLAND_X {
        for z in 0..ISLAND_Z {
            for y in 0..ISLAND_Y {
                let block = if y < ISLAND_Y - 1 {
                    BlockType::Stone
                } else {
                    BlockType::Grass
                };
                chunk.set_block(
                    (ISLAND_OX + x) as usize,
                    (ISLAND_OY + y) as usize,
                    (ISLAND_OZ + z) as usize,
                    block,
                );
            }
        }
    }
    chunk
}