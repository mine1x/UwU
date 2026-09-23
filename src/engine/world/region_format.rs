pub const REGION_MAGIC: u32 = 0x5245474E; // 'REGN'
pub const REGION_CHUNKS_SIDE: usize = 16;
pub const REGION_CHUNKS: usize = REGION_CHUNKS_SIDE * REGION_CHUNKS_SIDE; // 256

#[inline]
pub fn chunk_to_region_coords(cx: i32, cz: i32) -> ((i32, i32), (usize, usize)) {
    let rx = cx.div_euclid(REGION_CHUNKS_SIDE as i32);
    let rz = cz.div_euclid(REGION_CHUNKS_SIDE as i32);
    let lx = cx.rem_euclid(REGION_CHUNKS_SIDE as i32) as usize;
    let lz = cz.rem_euclid(REGION_CHUNKS_SIDE as i32) as usize;
    ((rx, rz), (lx, lz))
}

#[inline]
pub fn region_local_index(lx: usize, lz: usize) -> usize {
    lz * REGION_CHUNKS_SIDE + lx
}
