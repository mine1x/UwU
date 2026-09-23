// Minecraft Java Mineshaft structure generation
// Reference: net.minecraft.world.level.levelgen.structure.structures.{MineshaftStructure, MineshaftPieces}

use crate::world::block::BlockType;
use crate::world::chunk::Chunk;
use super::random::Xoroshiro128PlusPlus;

pub struct MineshaftGenerator;

impl MineshaftGenerator {
    pub fn is_mineshaft_chunk(cx: i32, cz: i32, seed: i64) -> bool {
        let mut rng = Xoroshiro128PlusPlus::from_seed(seed ^ ((cx as i64) * 341873128712 ^ (cz as i64) * 132897987541));
        rng.next_i32(120) == 0 // ~1 in 120 chunks has a mineshaft room
    }

    pub fn carve_corridors(
        chunk: &mut Chunk,
        cx: i32,
        cy: i32,
        cz: i32,
        seed: i64,
    ) {
        // Only generate in underground chunk sections (-4 <= cy <= 3, i.e., y = -64 to 64)
        if cy < -3 || cy > 2 {
            return;
        }

        // Check neighbouring 3x3 chunks for intersecting shafts
        for ox in -1..=1 {
            for oz in -1..=1 {
                let ncx = cx + ox;
                let ncz = cz + oz;
                if Self::is_mineshaft_chunk(ncx, ncz, seed) {
                    let mut rng = Xoroshiro128PlusPlus::from_seed(seed ^ ((ncx as i64) * 341873128712 ^ (ncz as i64) * 132897987541));
                    let room_x = ncx * 16 + 2 + rng.next_i32(12);
                    let room_y = (cy * 16) + 4 + rng.next_i32(6);
                    let room_z = ncz * 16 + 2 + rng.next_i32(12);

                    // Carve 4 corridors out of room
                    let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
                    for (dx, dz) in dirs {
                        let length = 12 + rng.next_i32(16);
                        for l in 0..length {
                            let wx = room_x + dx * l;
                            let wz = room_z + dz * l;
                            let wy = room_y;

                            let (target_cx, target_cy, target_cz) = (wx.div_euclid(16), wy.div_euclid(16), wz.div_euclid(16));
                            if target_cx == cx && target_cy == cy && target_cz == cz {
                                let lx = wx.rem_euclid(16) as usize;
                                let ly = wy.rem_euclid(16) as usize;
                                let lz = wz.rem_euclid(16) as usize;

                                for cy_off in 0..3 {
                                    for cx_off in -1..=1 {
                                        for cz_off in -1..=1 {
                                            let px = (lx as i32 + cx_off) as usize;
                                            let py = ly + cy_off;
                                            let pz = (lz as i32 + cz_off) as usize;

                                            if px < 16 && py < 16 && pz < 16 {
                                                if cy_off == 0 {
                                                    // Floor planks occasionally
                                                    if l % 5 == 0 && (cx_off != 0 || cz_off != 0) {
                                                        chunk.set_block(px, py, pz, BlockType::OakPlanks);
                                                    }
                                                } else {
                                                    chunk.set_block(px, py, pz, BlockType::Air);
                                                }
                                            }
                                        }
                                    }
                                }

                                // Wooden arches every 5 blocks
                                if l % 5 == 0 && ly + 2 < 16 {
                                    chunk.set_block(lx, ly + 2, lz, BlockType::OakPlanks);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
