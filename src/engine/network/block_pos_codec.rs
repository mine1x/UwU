// Minecraft Java BlockPos packed 64-bit coordinate codec
// Reference: net.minecraft.core.BlockPos.java

pub fn pack_block_pos(x: i32, y: i32, z: i32) -> u64 {
    let mut val = 0u64;
    val |= ((x as i64 & 0x3FFFFFF) as u64) << 38;
    val |= ((z as i64 & 0x3FFFFFF) as u64) << 12;
    val |= (y as i64 & 0xFFF) as u64;
    val
}

pub fn unpack_block_pos(val: u64) -> (i32, i32, i32) {
    let signed = val as i64;
    let x = (signed >> 38) as i32;
    let y = ((signed << 52) >> 52) as i32;
    let z = ((signed << 26) >> 38) as i32;
    (x, y, z)
}
