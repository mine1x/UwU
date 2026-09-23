#[cfg(test)]
mod tests {
    use crate::engine::TickSystem;
    use crate::world::chunk_codec::{apply_chunk_runs, encode_chunk_runs};
    use crate::world::chunk_save::{load_chunk_file, save_chunk_file, CHUNK_MAGIC_OLD};
    use crate::world::flat_gen::generate_flat_chunk_data;
    use crate::world::{schedule_camera_chunk_fluids, BlockType, Chunk, VoxelWorld};
    use std::io::Write;

    #[test]
    fn test_chunk_save_and_load_roundtrip() {
        let temp_dir = std::env::temp_dir().join("zomboid_test_save");
        let _ = std::fs::create_dir_all(&temp_dir);
        let path = temp_dir.join("c.1.0.2.dat");

        let mut chunk = Chunk::new((1, 0, 2));
        chunk.set_block(0, 1, 0, BlockType::Stone);
        chunk.set_block(5, 2, 7, BlockType::Grass);
        chunk.set_block(15, 3, 15, BlockType::WaterSource);

        assert!(save_chunk_file(&path, (1, 0, 2), &chunk).is_ok());
        let (coords, loaded) = load_chunk_file(&path).expect("Failed to load chunk");
        assert_eq!(coords, (1, 0, 2));
        assert_eq!(loaded.get_block(0, 1, 0), BlockType::Stone);
        assert_eq!(loaded.get_block(5, 2, 7), BlockType::Grass);
        assert_eq!(loaded.get_block(15, 3, 15), BlockType::WaterSource);
        assert_eq!(loaded.get_block(0, 0, 0), BlockType::Air);

        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn test_rle_compresses_terrain_layers() {
        let flat = generate_flat_chunk_data(3, 0, -2);
        let runs = encode_chunk_runs(&flat);
        // Flat terrain: 5 solid layers on top of air -> at most 6 runs.
        assert!(runs.len() <= 6, "expected <=6 runs, got {}", runs.len());
        // Total cells decoded from runs must always equal a full chunk.
        let total: usize = runs.iter().map(|&(c, _)| c as usize).sum();
        assert_eq!(total, 16 * 16 * 16);

        let mut decoded = Chunk::new((3, 0, -2));
        apply_chunk_runs(&mut decoded, &runs);
        for y in 0..16usize {
            for x in 0..16usize {
                for z in 0..16usize {
                    assert_eq!(flat.get_block(x, y, z), decoded.get_block(x, y, z), "mismatch at {x},{y},{z}");
                }
            }
        }
    }

    #[test]
    fn test_loads_legacy_sparse_format() {
        let temp_dir = std::env::temp_dir().join("zomboid_test_save");
        let _ = std::fs::create_dir_all(&temp_dir);
        let path = temp_dir.join("c.2.0.1.dat");

        let mut f = std::fs::File::create(&path).unwrap();
        f.write_all(&CHUNK_MAGIC_OLD.to_be_bytes()).unwrap();
        f.write_all(&2i32.to_be_bytes()).unwrap();
        f.write_all(&0i32.to_be_bytes()).unwrap();
        f.write_all(&1i32.to_be_bytes()).unwrap();
        f.write_all(&2u32.to_be_bytes()).unwrap(); // 2 non-air blocks
        f.write_all(&[3, 4, 5, 4]).unwrap(); // Stone at (3,4,5)
        f.write_all(&[7, 2, 9, 2]).unwrap(); // Grass at (7,2,9)

        let (coords, loaded) = load_chunk_file(&path).expect("Failed to load legacy chunk");
        assert_eq!(coords, (2, 0, 1));
        assert_eq!(loaded.get_block(3, 4, 5), BlockType::Stone);
        assert_eq!(loaded.get_block(7, 2, 9), BlockType::Grass);
        assert_eq!(loaded.get_block(0, 0, 0), BlockType::Air);

        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn test_fluid_camera_scheduling_and_border_dirty() {
        let mut world = VoxelWorld::new_flat(1);
        world.set_block(0, 5, 0, BlockType::WaterSource);

        let mut tick_system: TickSystem<BlockType> = TickSystem::new(20.0);
        schedule_camera_chunk_fluids(&world, &mut tick_system, (0, 0), 1);
        tick_system.advance(0.26);

        let ticks = tick_system.drain_current_ticks();
        assert!(ticks.iter().any(|t| t.pos == (0, 5, 0) && t.target == BlockType::WaterSource));

        // Border dirty test
        world.rebuild_all_dirty_chunks();
        world.set_block(0, 4, 0, BlockType::Air);
        let is_neighbor_dirty = world.storage.get_chunk_read(&(-1, 0, 0), |c| c.is_dirty).unwrap_or(false);
        assert!(is_neighbor_dirty);
    }
}
