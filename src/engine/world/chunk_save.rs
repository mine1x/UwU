use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Result, Write};
use std::path::Path;
use super::block_conv::block_from_u8;
use super::chunk::Chunk;
use super::chunk_codec::{apply_chunk_runs, encode_chunk_runs};

pub const CHUNK_MAGIC_OLD: u32 = 0x4D43484B; // 'MCHK' legacy sparse format
pub const CHUNK_MAGIC_RLE: u32 = 0x4D43484C; // 'MCHL' run-length encoded format

pub fn save_chunk_file(path: &Path, coords: (i32, i32, i32), chunk: &Chunk) -> Result<()> {
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let tmp_path = path.with_extension("dat.tmp");
    let file = File::create(&tmp_path)?;
    let mut w = BufWriter::new(file);

    w.write_all(&CHUNK_MAGIC_RLE.to_be_bytes())?;
    w.write_all(&coords.0.to_be_bytes())?;
    w.write_all(&coords.1.to_be_bytes())?;
    w.write_all(&coords.2.to_be_bytes())?;

    let runs = encode_chunk_runs(chunk);
    w.write_all(&(runs.len() as u32).to_be_bytes())?;
    for (count, bt) in runs {
        w.write_all(&count.to_be_bytes())?;
        w.write_all(&[bt])?;
    }
    w.flush()?;
    drop(w);
    std::fs::rename(&tmp_path, path)?;
    Ok(())
}

pub fn load_chunk_file(path: &Path) -> Result<((i32, i32, i32), Chunk)> {
    let file = File::open(path)?;
    let mut r = BufReader::new(file);

    let mut magic_buf = [0u8; 4];
    r.read_exact(&mut magic_buf)?;
    let magic = u32::from_be_bytes(magic_buf);
    let mut c_buf = [0u8; 12];
    r.read_exact(&mut c_buf)?;
    let cx = i32::from_be_bytes(c_buf[0..4].try_into().unwrap());
    let cy = i32::from_be_bytes(c_buf[4..8].try_into().unwrap());
    let cz = i32::from_be_bytes(c_buf[8..12].try_into().unwrap());

    let mut chunk = Chunk::new((cx, cy, cz));

    match magic {
        CHUNK_MAGIC_RLE => {
            let mut count_buf = [0u8; 4];
            r.read_exact(&mut count_buf)?;
            let run_count = u32::from_be_bytes(count_buf) as usize;
            let mut runs = Vec::with_capacity(run_count);
            let mut run_buf = [0u8; 3];
            for _ in 0..run_count {
                r.read_exact(&mut run_buf)?;
                let count = u16::from_be_bytes([run_buf[0], run_buf[1]]);
                runs.push((count, run_buf[2]));
            }
            apply_chunk_runs(&mut chunk, &runs);
        }
        CHUNK_MAGIC_OLD => {
            let mut count_buf = [0u8; 4];
            r.read_exact(&mut count_buf)?;
            let count = u32::from_be_bytes(count_buf) as usize;
            let mut entry = [0u8; 4];
            for _ in 0..count {
                r.read_exact(&mut entry)?;
                let (lx, ly, lz, bt) = (entry[0] as usize, entry[1] as usize, entry[2] as usize, entry[3]);
                chunk.set_block(lx, ly, lz, block_from_u8(bt));
            }
        }
        _ => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Bad chunk magic")),
    }

    chunk.disk_dirty = false;
    chunk.is_dirty = true;
    Ok(((cx, cy, cz), chunk))
}