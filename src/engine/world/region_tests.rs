#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::world::flat_gen::generate_flat_chunk_data;
    use crate::world::region_format::chunk_to_region_coords;
    use crate::world::region_reader::load_region_file;
    use crate::world::region_writer::save_region_file;
    use crate::world::{BlockType, Chunk, VoxelWorld};

    #[test]
    fn test_region_save_and_load_roundtrip() {
        let temp_dir = std::env::temp_dir().join("zomboid_test_regions");
        let _ = std::fs::create_dir_all(&temp_dir);
        let path = temp_dir.join("r.0.0.mca");

        let mut chunks = HashMap::new();
        let mut c1 = Chunk::new((0, 0, 0));
        c1.set_block(0, 0, 0, BlockType::Stone);
        c1.set_block(5, 4, 3, BlockType::Grass);
        chunks.insert((0, 0), c1);

        let c2 = generate_flat_chunk_data(1, 0, 2);
        chunks.insert((1, 2), c2);

        assert!(save_region_file(&path, (0, 0), &chunks).is_ok());
        let loaded = load_region_file(&path).expect("Failed to load region");
        assert_eq!(loaded.len(), 2);

        let map: HashMap<(i32, i32, i32), Chunk> = loaded.into_iter().collect();
        let loaded_c1 = map.get(&(0, 0, 0)).expect("chunk 0,0,0 not found");
        assert_eq!(loaded_c1.get_block(0, 0, 0), BlockType::Stone);
        assert_eq!(loaded_c1.get_block(5, 4, 3), BlockType::Grass);

        let loaded_c2 = map.get(&(1, 0, 2)).expect("chunk 1,0,2 not found");
        assert_eq!(loaded_c2.get_block(0, 0, 0), BlockType::Stone);
        assert_eq!(loaded_c2.get_block(0, 4, 0), BlockType::Grass);

        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn test_chunk_to_region_coords_calc() {
        let ((rx, rz), (lx, lz)) = chunk_to_region_coords(0, 0);
        assert_eq!((rx, rz), (0, 0));
        assert_eq!((lx, lz), (0, 0));

        let ((rx, rz), (lx, lz)) = chunk_to_region_coords(15, 15);
        assert_eq!((rx, rz), (0, 0));
        assert_eq!((lx, lz), (15, 15));

        let ((rx, rz), (lx, lz)) = chunk_to_region_coords(16, 32);
        assert_eq!((rx, rz), (1, 2));
        assert_eq!((lx, lz), (0, 0));

        let ((rx, rz), (lx, lz)) = chunk_to_region_coords(-1, -1);
        assert_eq!((rx, rz), (-1, -1));
        assert_eq!((lx, lz), (15, 15));
    }

    #[test]
    fn test_save_and_load_world_regions() {
        let world = VoxelWorld::new_flat(1);
        let count = crate::core::world::save_world_to_disk(&world);
        assert!(count > 0);

        let mut loaded_world = VoxelWorld::new();
        assert!(crate::core::world::load_world_from_disk(&mut loaded_world));
        assert_eq!(loaded_world.get_block(0, 0, 0), BlockType::Stone);
        assert_eq!(loaded_world.get_block(0, 4, 0), BlockType::Grass);
    }
}
