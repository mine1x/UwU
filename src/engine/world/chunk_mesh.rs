use super::block::BlockType;
use super::chunk::Chunk;
use super::water_slope::compute_water_corner_heights;
use crate::render::types::Vertex;
use bevy::math::Vec3;

pub fn compute_chunk_mesh<F>(chunk: &Chunk, mut get_world_block: F) -> (Vec<Vertex>, Vec<u32>)
where
    F: FnMut(i32, i32, i32) -> BlockType,
{
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let (bx, by, bz) = (chunk.coords.0 * 16, chunk.coords.1 * 16, chunk.coords.2 * 16);

    // Grid of chunk + 1-block boundary padding: 18 x 18 x 18
    let mut grid = [BlockType::Air; 18 * 18 * 18];
    #[inline(always)]
    fn idx(x: usize, y: usize, z: usize) -> usize {
        x + y * 18 + z * 18 * 18
    }

    for z in 0..18 {
        for y in 0..18 {
            for x in 0..18 {
                let gx = bx + x as i32 - 1;
                let gy = by + y as i32 - 1;
                let gz = bz + z as i32 - 1;
                let b = if x >= 1 && x <= 16 && y >= 1 && y <= 16 && z >= 1 && z <= 16 {
                    chunk.get_block(x - 1, y - 1, z - 1)
                } else {
                    get_world_block(gx, gy, gz)
                };
                grid[idx(x, y, z)] = b;
            }
        }
    }

    // 1. GREEDY MESHING FOR 6 FACES
    // Faces definition:
    // face 0: +Y (Top)
    // face 1: -Y (Bottom)
    // face 2: +Z (South)
    // face 3: -Z (North)
    // face 4: +X (East)
    // face 5: -X (West)

    for face in 0..6 {
        let (normal, offset) = match face {
            0 => (Vec3::Y, (0, 1, 0)),
            1 => (-Vec3::Y, (0, -1, 0)),
            2 => (Vec3::Z, (0, 0, 1)),
            3 => (-Vec3::Z, (0, 0, -1)),
            4 => (Vec3::X, (1, 0, 0)),
            5 => (-Vec3::X, (-1, 0, 0)),
            _ => unreachable!(),
        };

        // Determine slice orientation:
        // For +Y / -Y: slice through Y (1..=16), 2D grid is (X: 1..=16, Z: 1..=16)
        // For +Z / -Z: slice through Z (1..=16), 2D grid is (X: 1..=16, Y: 1..=16)
        // For +X / -X: slice through X (1..=16), 2D grid is (Z: 1..=16, Y: 1..=16)

        for d in 1..=16 {
            // Mask for the 16x16 slice: None or Some(tex_layer)
            let mut mask: [Option<f32>; 16 * 16] = [None; 16 * 16];

            for v in 0..16 {
                for u in 0..16 {
                    let (x, y, z) = match face {
                        0 | 1 => (u + 1, d, v + 1),
                        2 | 3 => (u + 1, v + 1, d),
                        4 | 5 => (d, v + 1, u + 1),
                        _ => unreachable!(),
                    };

                    let block = grid[idx(x, y, z)];
                    // Only standard solid blocks participate in greedy meshing
                    if block == BlockType::Air || block.is_fluid() || block == BlockType::Torch {
                        continue;
                    }

                    let nx = (x as i32 + offset.0) as usize;
                    let ny = (y as i32 + offset.1) as usize;
                    let nz = (z as i32 + offset.2) as usize;
                    let neighbor = grid[idx(nx, ny, nz)];

                    if neighbor.is_transparent() {
                        let tex = super::face_texture::compute_face_texture(block, offset, false, 0.0, block.tex_layer());
                        mask[u + v * 16] = Some(tex);
                    }
                }
            }

            // Greedy merge 2D mask
            let mut visited = [false; 16 * 16];
            for v in 0..16 {
                let mut u = 0;
                while u < 16 {
                    let m_idx = u + v * 16;
                    if let Some(tex) = mask[m_idx] {
                        if !visited[m_idx] {
                            // Find width along u
                            let mut w = 1;
                            while u + w < 16 && !visited[u + w + v * 16] && mask[u + w + v * 16] == Some(tex) {
                                w += 1;
                            }

                            // Find height along v
                            let mut h = 1;
                            'outer_h: while v + h < 16 {
                                for k in 0..w {
                                    let check_idx = u + k + (v + h) * 16;
                                    if visited[check_idx] || mask[check_idx] != Some(tex) {
                                        break 'outer_h;
                                    }
                                }
                                h += 1;
                            }

                            // Mark as visited
                            for dv in 0..h {
                                for du in 0..w {
                                    visited[u + du + (v + dv) * 16] = true;
                                }
                            }

                            // Generate Quad vertices & UVs
                            let wf = w as f32;
                            let hf = h as f32;

                            // Corner coords in chunk-local space
                            let (corners, uvs) = match face {
                                // +Y (Top): u is X (width), v is Z (height)
                                // Original quad: [0, 1, 1], [1, 1, 1], [1, 1, 0], [0, 1, 0] with UVs: [0,1], [1,1], [1,0], [0,0]
                                0 => {
                                    let x0 = u as f32;
                                    let x1 = (u + w) as f32;
                                    let z0 = v as f32;
                                    let z1 = (v + h) as f32;
                                    let y = d as f32;
                                    (
                                        [Vec3::new(x0, y, z1), Vec3::new(x1, y, z1), Vec3::new(x1, y, z0), Vec3::new(x0, y, z0)],
                                        [[0.0, hf], [wf, hf], [wf, 0.0], [0.0, 0.0]],
                                    )
                                }
                                // -Y (Bottom): u is X (width), v is Z (height)
                                // Original quad: [0, 0, 0], [1, 0, 0], [1, 0, 1], [0, 0, 1] with UVs: [0,1], [1,1], [1,0], [0,0]
                                1 => {
                                    let x0 = u as f32;
                                    let x1 = (u + w) as f32;
                                    let z0 = v as f32;
                                    let z1 = (v + h) as f32;
                                    let y = (d - 1) as f32;
                                    (
                                        [Vec3::new(x0, y, z0), Vec3::new(x1, y, z0), Vec3::new(x1, y, z1), Vec3::new(x0, y, z1)],
                                        [[0.0, 0.0], [wf, 0.0], [wf, hf], [0.0, hf]],
                                    )
                                }
                                // +Z (South): u is X (width), v is Y (height)
                                // Original quad: [0, 0, 1], [1, 0, 1], [1, 1, 1], [0, 1, 1] with UVs: [0,1], [1,1], [1,0], [0,0]
                                2 => {
                                    let x0 = u as f32;
                                    let x1 = (u + w) as f32;
                                    let y0 = v as f32;
                                    let y1 = (v + h) as f32;
                                    let z = d as f32;
                                    (
                                        [Vec3::new(x0, y0, z), Vec3::new(x1, y0, z), Vec3::new(x1, y1, z), Vec3::new(x0, y1, z)],
                                        [[0.0, hf], [wf, hf], [wf, 0.0], [0.0, 0.0]],
                                    )
                                }
                                // -Z (North): u is X (width), v is Y (height)
                                // Original quad: [1, 0, 0], [0, 0, 0], [0, 1, 0], [1, 1, 0] with UVs: [0,1], [1,1], [1,0], [0,0]
                                3 => {
                                    let x0 = u as f32;
                                    let x1 = (u + w) as f32;
                                    let y0 = v as f32;
                                    let y1 = (v + h) as f32;
                                    let z = (d - 1) as f32;
                                    (
                                        [Vec3::new(x1, y0, z), Vec3::new(x0, y0, z), Vec3::new(x0, y1, z), Vec3::new(x1, y1, z)],
                                        [[wf, hf], [0.0, hf], [0.0, 0.0], [wf, 0.0]],
                                    )
                                }
                                // +X (East): u is Z (width), v is Y (height)
                                // Original quad: [1, 0, 1], [1, 0, 0], [1, 1, 0], [1, 1, 1] with UVs: [0,1], [1,1], [1,0], [0,0]
                                4 => {
                                    let z0 = u as f32;
                                    let z1 = (u + w) as f32;
                                    let y0 = v as f32;
                                    let y1 = (v + h) as f32;
                                    let x = d as f32;
                                    (
                                        [Vec3::new(x, y0, z1), Vec3::new(x, y0, z0), Vec3::new(x, y1, z0), Vec3::new(x, y1, z1)],
                                        [[wf, hf], [0.0, hf], [0.0, 0.0], [wf, 0.0]],
                                    )
                                }
                                // -X (West): u is Z (width), v is Y (height)
                                // Original quad: [0, 0, 0], [0, 0, 1], [0, 1, 1], [0, 1, 0] with UVs: [0,1], [1,1], [1,0], [0,0]
                                5 => {
                                    let z0 = u as f32;
                                    let z1 = (u + w) as f32;
                                    let y0 = v as f32;
                                    let y1 = (v + h) as f32;
                                    let x = (d - 1) as f32;
                                    (
                                        [Vec3::new(x, y0, z0), Vec3::new(x, y0, z1), Vec3::new(x, y1, z1), Vec3::new(x, y1, z0)],
                                        [[0.0, hf], [wf, hf], [wf, 0.0], [0.0, 0.0]],
                                    )
                                }
                                _ => unreachable!(),
                            };

                            let s = vertices.len() as u32;
                            let base_c = Vec3::new(bx as f32, by as f32, bz as f32);
                            for i in 0..4 {
                                vertices.push(Vertex {
                                    position: (base_c + corners[i]).to_array(),
                                    normal: normal.to_array(),
                                    uv: uvs[i],
                                    tex_layer: tex,
                                });
                            }
                            indices.extend_from_slice(&[s, s + 1, s + 2, s, s + 2, s + 3]);

                            u += w;
                            continue;
                        }
                    }
                    u += 1;
                }
            }
        }
    }

    // 2. WATER & CUSTOM MESHES (Torches, fluid slope heights, etc.)
    let water_faces: [(Vec3, [Vec3; 4], (i32, i32, i32), [usize; 4]); 6] = [
        (Vec3::Y, [Vec3::new(0.0, 1.0, 1.0), Vec3::new(1.0, 1.0, 1.0), Vec3::new(1.0, 1.0, 0.0), Vec3::new(0.0, 1.0, 0.0)], (0, 1, 0), [3, 2, 1, 0]),
        (-Vec3::Y, [Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 1.0), Vec3::new(0.0, 0.0, 1.0)], (0, -1, 0), [0, 1, 2, 3]),
        (Vec3::Z, [Vec3::new(0.0, 0.0, 1.0), Vec3::new(1.0, 0.0, 1.0), Vec3::new(1.0, 1.0, 1.0), Vec3::new(0.0, 1.0, 1.0)], (0, 0, 1), [3, 2, 2, 3]),
        (-Vec3::Z, [Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), Vec3::new(1.0, 1.0, 0.0)], (0, 0, -1), [1, 0, 0, 1]),
        (Vec3::X, [Vec3::new(1.0, 0.0, 1.0), Vec3::new(1.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 0.0), Vec3::new(1.0, 1.0, 1.0)], (1, 0, 0), [2, 1, 1, 2]),
        (-Vec3::X, [Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0), Vec3::new(0.0, 1.0, 1.0), Vec3::new(0.0, 1.0, 0.0)], (-1, 0, 0), [0, 3, 3, 0]),
    ];
    let default_uvs = [[0.0, 1.0], [1.0, 1.0], [1.0, 0.0], [0.0, 0.0]];

    for lx in 0..16 {
        for ly in 0..16 {
            for lz in 0..16 {
                let block = chunk.get_block(lx, ly, lz);
                if block == BlockType::Air { continue; }
                let (gx, gy, gz) = (bx + lx as i32, by + ly as i32, bz + lz as i32);
                let base = Vec3::new(gx as f32, gy as f32, gz as f32);

                if super::custom_block_mesh::append_custom_block_mesh(block, base, &mut vertices, &mut indices) {
                    continue;
                }

                if block.is_fluid() {
                    let ch = compute_water_corner_heights(gx, gy, gz, &mut get_world_block);
                    let (fx, fz) = super::water_flow_dir::compute_water_flow_vector(gx, gy, gz, &mut get_world_block);
                    let (water_uvs, water_top_tex) = super::water_flow_dir::compute_water_top_uvs(fx, fz);

                    for (normal, corners, offset, c_map) in &water_faces {
                        let neighbor = get_world_block(gx + offset.0, gy + offset.1, gz + offset.2);
                        if !neighbor.is_fluid() {
                            let face_tex = super::face_texture::compute_face_texture(block, *offset, true, water_top_tex, block.tex_layer());
                            let face_uvs = if offset.1 == 1 { water_uvs } else { default_uvs };
                            let s = vertices.len() as u32;
                            for (i, corner) in corners.iter().enumerate() {
                                let mut p = *corner;
                                if p.y > 0.0 { p.y = ch[c_map[i]]; }
                                vertices.push(Vertex {
                                    position: (base + p).to_array(),
                                    normal: normal.to_array(),
                                    uv: face_uvs[i],
                                    tex_layer: face_tex,
                                });
                            }
                            indices.extend_from_slice(&[s, s + 1, s + 2, s, s + 2, s + 3]);
                        }
                    }
                }
            }
        }
    }

    (vertices, indices)
}