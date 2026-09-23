// Minecraft Java PerlinNoise, SmearedPerlinNoise, and NormalNoise
// Reference: net.minecraft.world.level.levelgen.synth.{GradientNoise, PerlinNoise, NormalNoise, BlendedNoise}

use super::random::Xoroshiro128PlusPlus;

const GRADIENTS: [(f32, f32, f32); 16] = [
    (1.0, 1.0, 0.0), (-1.0, 1.0, 0.0), (1.0, -1.0, 0.0), (-1.0, -1.0, 0.0),
    (1.0, 0.0, 1.0), (-1.0, 0.0, 1.0), (1.0, 0.0, -1.0), (-1.0, 0.0, -1.0),
    (0.0, 1.0, 1.0), (0.0, -1.0, 1.0), (0.0, 1.0, -1.0), (0.0, -1.0, -1.0),
    (1.0, 1.0, 0.0), (0.0, -1.0, 1.0), (-1.0, 1.0, 0.0), (0.0, -1.0, -1.0),
];

#[inline]
fn smoothstep(x: f32) -> f32 {
    x * x * x * (x * (x * 6.0 - 15.0) + 10.0)
}

#[inline]
fn lerp(t: f32, a: f32, b: f32) -> f32 {
    a + t * (b - a)
}

#[inline]
fn lerp3(x: f32, y: f32, z: f32,
         d000: f32, d100: f32, d010: f32, d110: f32,
         d001: f32, d101: f32, d011: f32, d111: f32) -> f32 {
    lerp(z,
        lerp(y, lerp(x, d000, d100), lerp(x, d010, d110)),
        lerp(y, lerp(x, d001, d101), lerp(x, d011, d111)),
    )
}

#[inline]
fn grad_dot(hash: usize, x: f32, y: f32, z: f32) -> f32 {
    let (gx, gy, gz) = GRADIENTS[hash & 15];
    gx * x + gy * y + gz * z
}

#[derive(Clone)]
pub struct PerlinNoise {
    pub perms: [u8; 256],
    pub offset_x: f64,
    pub offset_y: f64,
    pub offset_z: f64,
}

impl PerlinNoise {
    pub fn new(rng: &mut Xoroshiro128PlusPlus) -> Self {
        let mut perms = [0u8; 256];
        for i in 0..256 {
            perms[i] = i as u8;
        }
        for i in 0..256 {
            let offset = rng.next_i32(256 - i as i32) as usize;
            perms.swap(i, offset + i);
        }
        let offset_x = rng.next_f64() * 256.0;
        let offset_y = rng.next_f64() * 256.0;
        let offset_z = rng.next_f64() * 256.0;
        Self { perms, offset_x, offset_y, offset_z }
    }

    #[inline]
    fn permute(&self, x: i32) -> usize {
        self.perms[(x & 255) as usize] as usize
    }

    pub fn sample(&self, _x: f64, _y: f64, _z: f64) -> f32 {
        let x = _x + self.offset_x;
        let y = _y + self.offset_y;
        let z = _z + self.offset_z;
        let floor_x = x.floor() as i32;
        let floor_y = y.floor() as i32;
        let floor_z = z.floor() as i32;
        let rx = (x - floor_x as f64) as f32;
        let ry = (y - floor_y as f64) as f32;
        let rz = (z - floor_z as f64) as f32;

        let x0 = self.permute(floor_x);
        let x1 = self.permute(floor_x + 1);
        let xy00 = self.permute((x0 as i32) + floor_y);
        let xy01 = self.permute((x0 as i32) + floor_y + 1);
        let xy10 = self.permute((x1 as i32) + floor_y);
        let xy11 = self.permute((x1 as i32) + floor_y + 1);

        let d000 = grad_dot(self.permute((xy00 as i32) + floor_z), rx, ry, rz);
        let d100 = grad_dot(self.permute((xy10 as i32) + floor_z), rx - 1.0, ry, rz);
        let d010 = grad_dot(self.permute((xy01 as i32) + floor_z), rx, ry - 1.0, rz);
        let d110 = grad_dot(self.permute((xy11 as i32) + floor_z), rx - 1.0, ry - 1.0, rz);
        let d001 = grad_dot(self.permute((xy00 as i32) + floor_z + 1), rx, ry, rz - 1.0);
        let d101 = grad_dot(self.permute((xy10 as i32) + floor_z + 1), rx - 1.0, ry, rz - 1.0);
        let d011 = grad_dot(self.permute((xy01 as i32) + floor_z + 1), rx, ry - 1.0, rz - 1.0);
        let d111 = grad_dot(self.permute((xy11 as i32) + floor_z + 1), rx - 1.0, ry - 1.0, rz - 1.0);

        let xa = smoothstep(rx);
        let ya = smoothstep(ry);
        let za = smoothstep(rz);

        lerp3(xa, ya, za, d000, d100, d010, d110, d001, d101, d011, d111)
    }
}

#[derive(Clone)]
pub struct NormalNoise {
    first: Vec<(PerlinNoise, f64, f32)>,
    second: Vec<(PerlinNoise, f64, f32)>,
}

impl NormalNoise {
    pub fn create_parity(
        seed: i64,
        key: &str,
        first_octave: i32,
        amplitudes: &[f64],
    ) -> Self {
        let octaves = amplitudes.len();
        let mut first = Vec::with_capacity(octaves);
        let mut second = Vec::with_capacity(octaves);

        let mut sum_amp = 0.0f64;
        for &a in amplitudes {
            sum_amp += a.abs();
        }
        let normalization = 0.3333333333333333 / (sum_amp * 0.2702247831245211 * 1.4142135623730951).max(0.000001);

        for (i, &amp) in amplitudes.iter().enumerate() {
            if amp == 0.0 {
                continue;
            }
            let octave_idx = first_octave + (i as i32);
            let freq = 2.0f64.powi(octave_idx);
            let val_factor = (normalization * amp) as f32;

            let seed1_str = format!("{}:first:{}", key, i);
            let seed2_str = format!("{}:second:{}", key, i);
            let mut rng1 = Xoroshiro128PlusPlus::from_seed(seed ^ (octave_idx as i64 * 341873128712));
            let mut rng2 = Xoroshiro128PlusPlus::from_string(&format!("{}:{}", seed, seed2_str));
            let _ = seed1_str; // avoid warning

            first.push((PerlinNoise::new(&mut rng1), freq, val_factor));
            second.push((PerlinNoise::new(&mut rng2), freq * 1.0181268882175227, val_factor));
        }

        Self { first, second }
    }

    pub fn sample(&self, x: f64, y: f64, z: f64) -> f32 {
        let mut total = 0.0f32;
        for (noise, freq, amp) in &self.first {
            total += amp * noise.sample(x * freq, y * freq, z * freq);
        }
        for (noise, freq, amp) in &self.second {
            total += amp * noise.sample(x * freq, y * freq, z * freq);
        }
        total
    }

    #[inline]
    pub fn sample_2d(&self, x: f64, z: f64) -> f32 {
        self.sample(x, 0.0, z)
    }
}
