#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ChunkPos {
    pub x: i32,
    pub z: i32,
}

impl ChunkPos {
    pub const fn new(x: i32, z: i32) -> Self {
        Self { x, z }
    }

    #[inline]
    pub const fn pack(self) -> i64 {
        ((self.x as i64) & 0xFFFFFFFF) | (((self.z as i64) & 0xFFFFFFFF) << 32)
    }

    #[inline]
    pub const fn unpack(packed: i64) -> Self {
        Self { x: packed as i32, z: (packed >> 32) as i32 }
    }

    #[inline]
    pub const fn block_to_section_coord(block_coord: i32) -> i32 {
        block_coord >> 4
    }

    #[inline]
    pub fn containing(bx: i32, bz: i32) -> Self {
        Self {
            x: Self::block_to_section_coord(bx),
            z: Self::block_to_section_coord(bz),
        }
    }

    #[inline]
    pub const fn min_block_x(&self) -> i32 {
        self.x << 4
    }

    #[inline]
    pub const fn min_block_z(&self) -> i32 {
        self.z << 4
    }

    #[inline]
    pub fn distance_squared(&self, other: ChunkPos) -> i32 {
        let dx = self.x - other.x;
        let dz = self.z - other.z;
        dx * dx + dz * dz
    }
}

impl From<(i32, i32)> for ChunkPos {
    #[inline]
    fn from((x, z): (i32, i32)) -> Self {
        Self::new(x, z)
    }
}

impl From<(i32, i32, i32)> for ChunkPos {
    #[inline]
    fn from((x, _, z): (i32, i32, i32)) -> Self {
        Self::new(x, z)
    }
}