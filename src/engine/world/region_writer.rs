use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Result, Seek, SeekFrom, Write};
use std::path::Path;
use super::chunk::Chunk;
use super::chunk_codec::encode_chunk_runs;
use super::region_format::{region_local_index, REGION_CHUNKS, REGION_MAGIC};

pub fn save_region_file(path: &Path, (rx, rz): (i32, i32), chunks: &HashMap<(usize, usize), Chunk>) -> Result<()> {
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let tmp_path = path.with_extension("mca.tmp");
    let file = File::create(&tmp_path)?;
    let mut w = BufWriter::new(file);

    w.write_all(&REGION_MAGIC.to_be_bytes())?;
    w.write_all(&rx.to_be_bytes())?;
    w.write_all(&rz.to_be_bytes())?;

    let header_offset = 12usize;
    let table_size = REGION_CHUNKS * 8; // 256 * (offset: u32, len: u32)
    let payload_start = header_offset + table_size;

    let mut table: [(u32, u32); REGION_CHUNKS] = [(0, 0); REGION_CHUNKS];
    // Reserve space for header table
    w.write_all(&vec![0u8; table_size])?;

    let mut cur_offset = payload_start as u32;
    for (&(lx, lz), chunk) in chunks {
        let idx = region_local_index(lx, lz);
        let runs = encode_chunk_runs(chunk);
        let mut chunk_bytes = Vec::with_capacity(8 + runs.len() * 3);
        chunk_bytes.extend_from_slice(&(chunk.coords.1).to_be_bytes()); // cy
        chunk_bytes.extend_from_slice(&(runs.len() as u32).to_be_bytes());
        for (count, bt) in runs {
            chunk_bytes.extend_from_slice(&count.to_be_bytes());
            chunk_bytes.push(bt);
        }
        let len = chunk_bytes.len() as u32;
        w.write_all(&chunk_bytes)?;
        table[idx] = (cur_offset, len);
        cur_offset += len;
    }

    w.flush()?;
    let mut inner = w.into_inner()?;
    inner.seek(SeekFrom::Start(header_offset as u64))?;
    for (offset, len) in table {
        inner.write_all(&offset.to_be_bytes())?;
        inner.write_all(&len.to_be_bytes())?;
    }
    inner.flush()?;
    drop(inner);

    std::fs::rename(&tmp_path, path)?;
    Ok(())
}
