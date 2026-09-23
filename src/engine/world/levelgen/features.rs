// Minecraft Java Features: Trees (StraightTrunk + BlobFoliage), Ores (Ellipsoid sampling)
// Reference: net.minecraft.world.level.levelgen.feature.{TreeFeature, OreFeature}

use crate::world::block::BlockType;
use crate::world::chunk::Chunk;
use super::random::Xoroshiro128PlusPlus;

pub struct TreeFeature;

impl TreeFeature {
    pub fn place(
        chunk: &mut Chunk,
        rng: &mut Xoroshiro128PlusPlus,
        lx: usize,
        ground_y: usize,
        lz: usize,
    ) {
        let tree_height = 4 + (rng.next_i32(3) as usize); // 4..=6
        let trunk_top = ground_y + tree_height;

        // Trunk
        for y in ground_y..trunk_top {
            if y < 16 {
                chunk.set_block(lx, y, lz, BlockType::OakLog);
            }
        }

        // Leaves (BlobFoliagePlacer: 2 layers of radius 2, 1-2 layers of radius 1)
        let leaves_start = if ground_y + 3 > trunk_top { ground_y } else { trunk_top - 3 };
        let leaves_end = trunk_top + 1;

        for y in leaves_start..=leaves_end {
            if y >= 16 {
                continue;
            }
            let is_top = y >= trunk_top;
            let radius: i32 = if is_top { 1 } else { 2 };

            for dx in -radius..=radius {
                for dz in -radius..=radius {
                    if dx.abs() == radius && dz.abs() == radius {
                        // Skip corners randomly (BlobFoliagePlacer parity)
                        if is_top || rng.next_i32(2) == 0 {
                            continue;
                        }
                    }

                    let px = (lx as i32) + dx;
                    let pz = (lz as i32) + dz;

                    if px >= 0 && px < 16 && pz >= 0 && pz < 16 {
                        let ux = px as usize;
                        let uz = pz as usize;
                        let cur = chunk.get_block(ux, y, uz);
                        if cur == BlockType::Air {
                            chunk.set_block(ux, y, uz, BlockType::OakLeaves);
                        }
                    }
                }
            }
        }
    }
}

pub struct OreFeature;

impl OreFeature {
    pub fn place(
        chunk: &mut Chunk,
        rng: &mut Xoroshiro128PlusPlus,
        ore_type: BlockType,
        size: usize,
        lx: usize,
        ly: usize,
        lz: usize,
    ) {
        let dir = rng.next_f64() * std::f64::consts::PI;
        let spread_xz = (size as f64) / 8.0;
        let x0 = (lx as f64) + dir.sin() * spread_xz;
        let x1 = (lx as f64) - dir.sin() * spread_xz;
        let z0 = (lz as f64) + dir.cos() * spread_xz;
        let z1 = (lz as f64) - dir.cos() * spread_xz;
        let y0 = (ly as f64) + (rng.next_i32(3) - 1) as f64;
        let y1 = (ly as f64) + (rng.next_i32(3) - 1) as f64;

        for step_i in 0..size {
            let step = (step_i as f64) / (size as f64);
            let cx = x0 + (x1 - x0) * step;
            let cy = y0 + (y1 - y0) * step;
            let cz = z0 + (z1 - z0) * step;
            let radius = (std::f64::consts::PI * step).sin() * (spread_xz * 0.5) + 0.8;

            let min_x = (cx - radius).floor().max(0.0) as usize;
            let max_x = (cx + radius).ceil().min(15.0) as usize;
            let min_y = (cy - radius).floor().max(0.0) as usize;
            let max_y = (cy + radius).ceil().min(15.0) as usize;
            let min_z = (cz - radius).floor().max(0.0) as usize;
            let max_z = (cz + radius).ceil().min(15.0) as usize;

            for x in min_x..=max_x {
                let dx = (x as f64 + 0.5 - cx) / radius;
                for y in min_y..=max_y {
                    let dy = (y as f64 + 0.5 - cy) / radius;
                    for z in min_z..=max_z {
                        let dz = (z as f64 + 0.5 - cz) / radius;
                        if dx * dx + dy * dy + dz * dz <= 1.0 {
                            if chunk.get_block(x, y, z) == BlockType::Stone {
                                chunk.set_block(x, y, z, ore_type);
                            }
                        }
                    }
                }
            }
        }
    }
}
