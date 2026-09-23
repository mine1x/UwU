use std::collections::HashMap;
use std::path::PathBuf;
use crate::world::chunk::Chunk;
use crate::world::concurrent_storage::ConcurrentChunkStorage;
use crate::world::region_format::chunk_to_region_coords;
use crate::world::region_reader::load_region_file;
use crate::world::region_writer::save_region_file;
use crate::world::voxel::VoxelWorld;

pub fn get_world_save_dir() -> PathBuf {
    PathBuf::from("saves").join("world").join("regions")
}

pub fn save_world_to_disk(world: &VoxelWorld) -> usize {
    let dir = get_world_save_dir();
    let _ = std::fs::create_dir_all(&dir);
    let mut regions: HashMap<(i32, i32), HashMap<(usize, usize), Chunk>> = HashMap::new();
    world.for_each_chunk(|&coords, chunk| {
        let (r_coord, l_coord) = chunk_to_region_coords(coords.0, coords.2);
        regions.entry(r_coord).or_default().insert(l_coord, chunk.clone());
    });
    let mut count = 0;
    for (r_coord, chunks) in &regions {
        let path = dir.join(format!("r.{}.{}.mca", r_coord.0, r_coord.1));
        if save_region_file(&path, *r_coord, chunks).is_ok() {
            count += chunks.len();
        }
    }
    world.storage.clear_disk_dirty_all();
    count
}

pub fn save_dirty_chunks_to_disk(storage: &ConcurrentChunkStorage) -> usize {
    let dirty = storage.drain_disk_dirty();
    if dirty.is_empty() {
        return 0;
    }
    let dir = get_world_save_dir();
    let _ = std::fs::create_dir_all(&dir);
    let mut dirty_regions: HashMap<(i32, i32), HashMap<(usize, usize), Chunk>> = HashMap::new();
    for (coords, chunk) in dirty {
        let (r_coord, l_coord) = chunk_to_region_coords(coords.0, coords.2);
        dirty_regions.entry(r_coord).or_default().insert(l_coord, chunk);
    }
    let mut count = 0;
    for (r_coord, dirty_chunks) in dirty_regions {
        let path = dir.join(format!("r.{}.{}.mca", r_coord.0, r_coord.1));
        let mut full_chunks: HashMap<(usize, usize), Chunk> = HashMap::new();
        if path.exists() {
            if let Ok(loaded) = load_region_file(&path) {
                for (coords, c) in loaded {
                    let (_, l_coord) = chunk_to_region_coords(coords.0, coords.2);
                    full_chunks.insert(l_coord, c);
                }
            }
        }
        for (l_coord, c) in dirty_chunks {
            full_chunks.insert(l_coord, c);
            count += 1;
        }
        let _ = save_region_file(&path, r_coord, &full_chunks);
    }
    count
}

pub fn load_world_from_disk(world: &mut VoxelWorld) -> bool {
    let dir = get_world_save_dir();
    if !dir.exists() {
        return false;
    }
    let mut loaded_any = false;
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                if ext == "mca" {
                    if let Ok(chunks) = load_region_file(&path) {
                        for (coords, chunk) in chunks {
                            world.storage.insert(coords, chunk);
                            loaded_any = true;
                        }
                    }
                }
            }
        }
    }
    if loaded_any {
        world.needs_mesh_rebuild = true;
    }
    loaded_any
}
