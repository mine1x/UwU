use super::*;
use std::sync::Arc;
use std::thread;

#[test]
fn test_chunk_pos_pack_unpack() {
    let pos = ChunkPos::new(-12, 34);
    let packed = pos.pack();
    let unpacked = ChunkPos::unpack(packed);
    assert_eq!(pos, unpacked);
    assert_eq!(pos.distance_squared(ChunkPos::new(-12, 35)), 1);
}

#[test]
fn test_chunk_section_counts() {
    let mut sec = LevelChunkSection::new(0);
    assert!(sec.is_empty());
    sec.set_block_state(0, 0, 0, BlockType::Stone);
    assert_eq!(sec.non_empty_block_count, 1);
    assert_eq!(sec.fluid_count, 0);

    sec.set_block_state(1, 1, 1, BlockType::WaterSource);
    assert_eq!(sec.non_empty_block_count, 2);
    assert_eq!(sec.fluid_count, 1);

    sec.set_block_state(0, 0, 0, BlockType::Air);
    assert_eq!(sec.non_empty_block_count, 1);
}

#[test]
fn test_colormap_lookup() {
    let cm = ColorMap::new();
    let grass_color = cm.get_grass_color(0.8, 0.4);
    assert!(grass_color[0] > 0.0 && grass_color[1] > 0.0);
    let foliage_color = cm.get_foliage_color(0.8, 0.4);
    assert!(foliage_color[0] > 0.0 && foliage_color[1] > 0.0);
}

#[test]
fn test_concurrent_storage_multithreaded() {
    let storage = Arc::new(ConcurrentChunkStorage::new());
    let mut handles = Vec::new();
    for i in 0..8 {
        let st = Arc::clone(&storage);
        handles.push(thread::spawn(move || {
            for j in 0..10 {
                let mut chunk = Chunk::new((i, 0, j));
                chunk.set_block(0, 0, 0, BlockType::Stone);
                st.insert((i, 0, j), chunk);
            }
        }));
    }
    for h in handles { h.join().unwrap(); }
    assert_eq!(storage.get_all_keys().len(), 80);
}

#[test]
fn test_voxel_world_parallel_mesh() {
    let mut world = VoxelWorld::new_flat(1);
    assert!(!world.cached_world_mesh.0.is_empty());
    assert!(!world.cached_world_mesh.1.is_empty());
    world.set_block(0, 5, 0, BlockType::Wood);
    assert_eq!(world.get_block(0, 5, 0), BlockType::Wood);
    world.rebuild_all_dirty_chunks();
    assert!(!world.needs_mesh_rebuild);
}

#[test]
fn test_water_flow_vector_and_uvs() {
    let (fx, fz) = compute_water_flow_vector(0, 5, 0, |x, y, z| {
        if x == 0 && y == 5 && z == 0 { BlockType::FlowingWater { level: 4 } }
        else if x == 1 && y == 5 && z == 0 { BlockType::FlowingWater { level: 3 } }
        else { BlockType::Air }
    });
    assert!(fx > 0.0);

    let (uvs, tex) = compute_water_top_uvs(fx, fz);
    assert_eq!(tex, 5.0);
    assert_ne!(uvs[0], [0.0, 1.0]);
}
