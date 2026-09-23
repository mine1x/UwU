use std::fs::File;
use std::io::{BufReader, Read, Result, Seek, SeekFrom};
use std::path::Path;
use super::chunk::Chunk;
use super::chunk_codec::apply_chunk_runs;
use super::region_format::{REGION_CHUNKS, REGION_CHUNKS_SIDE, REGION_MAGIC};

pub fn load_region_file(path: &Path) -> Result<Vec<((i32, i32, i32), Chunk)>> {
    let file = File::open(path)?;
    let mut r = BufReader::new(file);

    let mut head = [0u8; 12];
    r.read_exact(&mut head)?;
    let magic = u32::from_be_bytes(head[0..4].try_into().unwrap());
    if magic != REGION_MAGIC {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Bad region magic"));
    }
    let rx = i32::from_be_bytes(head[4..8].try_into().unwrap());
    let rz = i32::from_be_bytes(head[8..12].try_into().unwrap());

    let mut table = Vec::with_capacity(REGION_CHUNKS);
    let mut entry_buf = [0u8; 8];
    for _ in 0..REGION_CHUNKS {
        r.read_exact(&mut entry_buf)?;
        let off = u32::from_be_bytes(entry_buf[0..4].try_into().unwrap());
        let len = u32::from_be_bytes(entry_buf[4..8].try_into().unwrap());
        table.push((off, len));
    }

    let mut result = Vec::new();
    for (idx, &(off, len)) in table.iter().enumerate() {
        if off == 0 || len == 0 { continue; }
        r.seek(SeekFrom::Start(off as u64))?;
        let mut data = vec![0u8; len as usize];
        r.read_exact(&mut data)?;
        if data.len() < 8 { continue; }
        let cy = i32::from_be_bytes(data[0..4].try_into().unwrap());
        let run_count = u32::from_be_bytes(data[4..8].try_into().unwrap()) as usize;
        let mut runs = Vec::with_capacity(run_count);
        let mut cursor = 8usize;
        for _ in 0..run_count {
            if cursor + 3 > data.len() { break; }
            let count = u16::from_be_bytes([data[cursor], data[cursor + 1]]);
            let bt = data[cursor + 2];
            runs.push((count, bt));
            cursor += 3;
        }
        let lx = (idx % REGION_CHUNKS_SIDE) as i32;
        let lz = (idx / REGION_CHUNKS_SIDE) as i32;
        let cx = rx * REGION_CHUNKS_SIDE as i32 + lx;
        let cz = rz * REGION_CHUNKS_SIDE as i32 + lz;
        let mut chunk = Chunk::new((cx, cy, cz));
        apply_chunk_runs(&mut chunk, &runs);
        chunk.disk_dirty = false;
        chunk.is_dirty = true;
        result.push(((cx, cy, cz), chunk));
    }
    Ok(result)
}
