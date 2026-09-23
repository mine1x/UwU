use std::sync::Arc;
use std::thread;
use super::block::BlockType;
use super::chunk_mesh::compute_chunk_mesh;
use super::concurrent_storage::ConcurrentChunkStorage;
use crate::render::types::Vertex;

pub fn rebuild_dirty_chunks_multithreaded(
    storage: &Arc<ConcurrentChunkStorage>,
    dirty_coords: Vec<(i32, i32, i32)>,
) {
    if dirty_coords.is_empty() { return; }

    let chunk_count = dirty_coords.len();
    let num_threads = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4)
        .min(chunk_count)
        .max(1);

    let chunk_size = (chunk_count + num_threads - 1) / num_threads;
    let mut handles = Vec::with_capacity(num_threads);

    for chunk_slice in dirty_coords.chunks(chunk_size) {
        let storage_clone = Arc::clone(storage);
        let coords_vec = chunk_slice.to_vec();

        let handle = thread::spawn(move || {
            let mut results = Vec::with_capacity(coords_vec.len());
            for c_pos in coords_vec {
                let chunk_copy = storage_clone.get_chunk_read(&c_pos, |c| c.clone());
                if let Some(chunk) = chunk_copy {
                    let mesh = compute_chunk_mesh(&chunk, |nx, ny, nz| {
                        let (cx, cy, cz) = (nx.div_euclid(16), ny.div_euclid(16), nz.div_euclid(16));
                        let (lx, ly, lz) = (nx.rem_euclid(16) as usize, ny.rem_euclid(16) as usize, nz.rem_euclid(16) as usize);
                        storage_clone.get_chunk_read(&(cx, cy, cz), |nc| nc.get_block(lx, ly, lz)).unwrap_or(BlockType::Air)
                    });
                    results.push((c_pos, mesh));
                }
            }
            results
        });
        handles.push(handle);
    }

    for handle in handles {
        if let Ok(results) = handle.join() {
            for (pos, mesh) in results {
                storage.get_chunk_write(&pos, |c| {
                    c.mesh = mesh;
                    c.is_dirty = false;
                });
            }
        }
    }
}

pub fn collect_world_mesh(storage: &ConcurrentChunkStorage) -> (Vec<Vertex>, Vec<u32>) {
    let mut all_v = Vec::new();
    let mut all_i = Vec::new();
    storage.for_each_chunk(|_, chunk| {
        let offset = all_v.len() as u32;
        all_v.extend_from_slice(&chunk.mesh.0);
        for &idx in &chunk.mesh.1 {
            all_i.push(offset + idx);
        }
    });
    (all_v, all_i)
}