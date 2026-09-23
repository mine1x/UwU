use super::block::BlockType;
use super::block_conv::{block_from_u8, block_to_u8};
use super::chunk::Chunk;

const CHUNK_CELLS: usize = 16 * 16 * 16;

pub fn encode_chunk_runs(chunk: &Chunk) -> Vec<(u16, u8)> {
    let mut runs: Vec<(u16, u8)> = Vec::with_capacity(64);
    let mut cur_type: u8 = block_to_u8(&BlockType::Air);
    let mut cur_count: u16 = 0;
    for y in 0..16usize {
        for x in 0..16usize {
            for z in 0..16usize {
                let bt = block_to_u8(&chunk.get_block(x, y, z));
                if bt == cur_type && cur_count < u16::MAX {
                    cur_count += 1;
                } else {
                    if cur_count > 0 {
                        runs.push((cur_count, cur_type));
                    }
                    cur_type = bt;
                    cur_count = 1;
                }
            }
        }
    }
    if cur_count > 0 {
        runs.push((cur_count, cur_type));
    }
    runs
}

pub fn apply_chunk_runs(chunk: &mut Chunk, runs: &[(u16, u8)]) {
    let mut idx = 0usize;
    for &(count, bt) in runs {
        let block = block_from_u8(bt);
        let mut remaining = count as usize;
        while remaining > 0 && idx < CHUNK_CELLS {
            let y = idx / 256;
            let rem = idx % 256;
            let x = rem / 16;
            let z = rem % 16;
            chunk.set_block(x, y, z, block);
            idx += 1;
            remaining -= 1;
        }
    }
}
