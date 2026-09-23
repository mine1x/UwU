#[cfg(test)]
mod tests {
    use crate::world::levelgen::{MinecraftWorldGenerator, BiomeType};
    use crate::world::block::BlockType;

    #[test]
    fn test_minecraft_worldgen_surface_and_biomes() {
        let generator = MinecraftWorldGenerator::new(12345);
        let (h0, b0) = generator.compute_surface_height(0, 0);
        assert!(h0 >= 50 && h0 <= 150, "Surface height should be reasonable, got: {}", h0);
        assert!(matches!(b0, BiomeType::Plains | BiomeType::Forest | BiomeType::BirchForest | BiomeType::Taiga | BiomeType::Desert | BiomeType::Ocean | BiomeType::Beach | BiomeType::Mountains | BiomeType::StonyPeaks | BiomeType::Swamp | BiomeType::Jungle | BiomeType::Savanna | BiomeType::SnowyPlains | BiomeType::DeepOcean));
    }

    #[test]
    fn test_minecraft_chunk_generation_and_caves() {
        let generator = MinecraftWorldGenerator::new(12345);
        let (h, _) = generator.compute_surface_height(0, 0);
        let surf_cy = h.div_euclid(16);
        // Generate chunk section containing the surface
        let chunk = generator.generate_chunk(0, surf_cy, 0);
        // Also check underground chunk section
        let chunk_under = generator.generate_chunk(0, surf_cy - 1, 0);

        let mut has_solid = false;
        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    let block = chunk.get_block(x, y, z);
                    let block_u = chunk_under.get_block(x, y, z);
                    if block.is_solid() || block_u.is_solid() {
                        has_solid = true;
                    }
                }
            }
        }
        assert!(has_solid, "Chunk must contain terrain solid blocks at or below surface");
    }

    #[test]
    fn test_underground_mineshafts_and_ores() {
        let generator = MinecraftWorldGenerator::new(12345);
        // Generate deep underground chunk sections (cy = -2..=2)
        let mut found_ores = false;
        for cy in -2..=2 {
            let chunk = generator.generate_chunk(0, cy, 0);
            for x in 0..16 {
                for y in 0..16 {
                    for z in 0..16 {
                        let block = chunk.get_block(x, y, z);
                        if block == BlockType::IronBlock || block == BlockType::CoalBlock || block == BlockType::DiamondBlock {
                            found_ores = true;
                        }
                    }
                }
            }
        }
        assert!(found_ores, "Must find generated ore veins underground");
    }
}
