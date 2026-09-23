use image::GenericImageView;

pub struct ColorMap {
    grass_pixels: Box<[u32; 65536]>,
    foliage_pixels: Box<[u32; 65536]>,
}

impl ColorMap {
    pub fn new() -> Self {
        let grass_bytes = include_bytes!("../../assets/textures/colormap/grass.png");
        let foliage_bytes = include_bytes!("../../assets/textures/colormap/foliage.png");

        let grass_pixels = Self::load_colormap(grass_bytes, 0x79C05A);
        let foliage_pixels = Self::load_colormap(foliage_bytes, 0x59C93C);

        Self { grass_pixels, foliage_pixels }
    }

    fn load_colormap(bytes: &[u8], fallback: u32) -> Box<[u32; 65536]> {
        let mut pixels: Box<[u32; 65536]> = vec![fallback; 65536].into_boxed_slice().try_into().unwrap();
        if let Ok(img) = image::load_from_memory(bytes) {
            for y in 0..256.min(img.height()) {
                for x in 0..256.min(img.width()) {
                    let p = img.get_pixel(x, y);
                    let rgb = ((p.0[0] as u32) << 16) | ((p.0[1] as u32) << 8) | (p.0[2] as u32);
                    pixels[(y * 256 + x) as usize] = rgb;
                }
            }
        }
        pixels
    }

    /// ColorMapColorUtil.get(temp, rain, pixels, default)
    fn get_color(temp: f64, rain: f64, pixels: &[u32; 65536], default_color: u32) -> [f32; 3] {
        let t = temp.clamp(0.0, 1.0);
        let r = (rain.clamp(0.0, 1.0)) * t;
        let x = ((1.0 - t) * 255.0) as usize;
        let y = ((1.0 - r) * 255.0) as usize;
        let index = (y << 8) | x;
        let rgb = if index < pixels.len() { pixels[index] } else { default_color };
        [
            ((rgb >> 16) & 0xFF) as f32 / 255.0,
            ((rgb >> 8) & 0xFF) as f32 / 255.0,
            (rgb & 0xFF) as f32 / 255.0,
        ]
    }

    pub fn get_grass_color(&self, temp: f64, rain: f64) -> [f32; 3] {
        Self::get_color(temp, rain, &self.grass_pixels, 0x79C05A)
    }

    pub fn get_foliage_color(&self, temp: f64, rain: f64) -> [f32; 3] {
        Self::get_color(temp, rain, &self.foliage_pixels, 0x59C93C)
    }
}