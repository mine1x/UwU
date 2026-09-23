// Minecraft Java 1.18 - 1.21+ Worldgen Xoroshiro128++ & MD5 Hasher
// Reference: net.minecraft.world.level.levelgen.Xoroshiro128PlusPlus / RandomSupport

pub struct Xoroshiro128PlusPlus {
    pub seed_lo: u64,
    pub seed_hi: u64,
}

impl Xoroshiro128PlusPlus {
    pub fn new(seed_lo: u64, seed_hi: u64) -> Self {
        let (mut lo, mut hi) = (seed_lo, seed_hi);
        if (lo | hi) == 0 {
            lo = 0x9E3779B97F4A7C15; // Golden ratio 64
            hi = 0x6A09E667F3BCC909; // Silver ratio 64
        }
        Self { seed_lo: lo, seed_hi: hi }
    }

    pub fn from_seed(seed: i64) -> Self {
        let low = (seed as u64) ^ 0x6A09E667F3BCC909;
        let high = low.wrapping_add(0x9E3779B97F4A7C15);
        Self::new(mix_stafford13(low), mix_stafford13(high))
    }

    pub fn from_string(s: &str) -> Self {
        let hash = md5_hash(s.as_bytes());
        let lo = u64::from_le_bytes(hash[0..8].try_into().unwrap());
        let hi = u64::from_le_bytes(hash[8..16].try_into().unwrap());
        Self::new(lo, hi)
    }

    #[inline]
    pub fn next_u64(&mut self) -> u64 {
        let s0 = self.seed_lo;
        let mut s1 = self.seed_hi;
        let result = (s0.wrapping_add(s1)).rotate_left(17).wrapping_add(s0);

        s1 ^= s0;
        self.seed_lo = s0.rotate_left(49) ^ s1 ^ (s1 << 21);
        self.seed_hi = s1.rotate_left(28);
        result
    }

    #[inline]
    pub fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 * 1.1102230246251565e-16
    }

    #[inline]
    pub fn next_i32(&mut self, bound: i32) -> i32 {
        if bound <= 0 {
            return 0;
        }
        let r = (self.next_u64() & 0xFFFFFFFF) as u32;
        let mut m = (r as u64).wrapping_mul(bound as u64);
        let mut l = m as u32;
        if l < bound as u32 {
            let t = (!(bound as u32) + 1) % (bound as u32);
            while l < t {
                let r = (self.next_u64() & 0xFFFFFFFF) as u32;
                m = (r as u64).wrapping_mul(bound as u64);
                l = m as u32;
            }
        }
        (m >> 32) as i32
    }
}

pub fn mix_stafford13(mut z: u64) -> u64 {
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
    z ^ (z >> 31)
}

// Simple MD5 implementation for standard Minecraft string noise hashing
pub fn md5_hash(input: &[u8]) -> [u8; 16] {
    let mut h0: u32 = 0x67452301;
    let mut h1: u32 = 0xEFCDAB89;
    let mut h2: u32 = 0x98BADCFE;
    let mut h3: u32 = 0x10325476;

    let len = input.len();
    let mut padded = Vec::from(input);
    padded.push(0x80);
    while (padded.len() % 64) != 56 {
        padded.push(0);
    }
    let bit_len = (len as u64).wrapping_mul(8);
    padded.extend_from_slice(&bit_len.to_le_bytes());

    let s: [u32; 64] = [
        7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22,
        5,  9, 14, 20, 5,  9, 14, 20, 5,  9, 14, 20, 5,  9, 14, 20,
        4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23,
        6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
    ];

    let k: [u32; 64] = [
        0xD76AA478, 0xE8C7B756, 0x242070DB, 0xC1BDCEEE, 0xF57C0FAF, 0x4787C62A, 0xA8304613, 0xFD469501,
        0x698098D8, 0x8B44F7AF, 0xFFFF5BB1, 0x895CD7BE, 0x6B901122, 0xFD987193, 0xA679438E, 0x49B40821,
        0xF61E2562, 0xC040B340, 0x265E5A51, 0xE9B6C7AA, 0xD62F105D, 0x02441453, 0xD8A1E681, 0xE7D3FBC8,
        0x21E1CDE6, 0xC33707D6, 0xF4D50D87, 0x455A14ED, 0xA9E3E905, 0xFCEFA3F8, 0x676F02D9, 0x8D2A4C8A,
        0xFFFA3942, 0x8771F681, 0x6D9D6122, 0xFDE5380C, 0xA4BEEA44, 0x4BDECFA9, 0xF6BB4B60, 0xBEBFBC70,
        0x289B7EC6, 0xEAA127FA, 0xD4EF3085, 0x04881D05, 0xD9D4D039, 0xE6DB99E5, 0x1FA27CF8, 0xC4AC5665,
        0xF4292244, 0x432AFF97, 0xAB9423A7, 0xFC93A039, 0x655B59C3, 0x8F0CCC92, 0xFFEFF47D, 0x85845DD1,
        0x6FA87E4F, 0xFE2CE6E0, 0xA3014314, 0x4E0811A1, 0xF7537E82, 0xBD3AF235, 0x2AD7D2BB, 0xEB86D391,
    ];

    for chunk in padded.chunks_exact(64) {
        let mut m = [0u32; 16];
        for i in 0..16 {
            m[i] = u32::from_le_bytes(chunk[i * 4..(i + 1) * 4].try_into().unwrap());
        }

        let mut a = h0;
        let mut b = h1;
        let mut c = h2;
        let mut d = h3;

        for i in 0..64 {
            let (f, g) = match i {
                0..=15 => ((b & c) | (!b & d), i),
                16..=31 => ((d & b) | (!d & c), (5 * i + 1) % 16),
                32..=47 => (b ^ c ^ d, (3 * i + 5) % 16),
                _ => (c ^ (b | !d), (7 * i) % 16),
            };

            let temp = d;
            d = c;
            c = b;
            let sum = a.wrapping_add(f).wrapping_add(k[i]).wrapping_add(m[g]);
            b = b.wrapping_add(sum.rotate_left(s[i]));
            a = temp;
        }

        h0 = h0.wrapping_add(a);
        h1 = h1.wrapping_add(b);
        h2 = h2.wrapping_add(c);
        h3 = h3.wrapping_add(d);
    }

    let mut out = [0u8; 16];
    out[0..4].copy_from_slice(&h0.to_le_bytes());
    out[4..8].copy_from_slice(&h1.to_le_bytes());
    out[8..12].copy_from_slice(&h2.to_le_bytes());
    out[12..16].copy_from_slice(&h3.to_le_bytes());
    out
}
