use image::GenericImageView;

pub struct HudAtlas;

#[derive(Clone, Copy)]
pub struct SpriteRect {
    pub u0: f32,
    pub v0: f32,
    pub u1: f32,
    pub v1: f32,
}

impl HudAtlas {
    pub const WIDTH: u32 = 1024;
    pub const HEIGHT: u32 = 1024;

    pub const HOTBAR: SpriteRect = Self::rect(0, 0, 182, 22);
    pub const SELECTION: SpriteRect = Self::rect(0, 24, 24, 24);
    pub const HEART_BG: SpriteRect = Self::rect(30, 24, 9, 9);
    pub const HEART_FULL: SpriteRect = Self::rect(40, 24, 9, 9);
    pub const HEART_HALF: SpriteRect = Self::rect(50, 24, 9, 9);
    pub const FOOD_BG: SpriteRect = Self::rect(60, 24, 9, 9);
    pub const FOOD_FULL: SpriteRect = Self::rect(70, 24, 9, 9);
    pub const CROSSHAIR: SpriteRect = Self::rect(80, 24, 15, 15);
    pub const XP_BG: SpriteRect = Self::rect(0, 50, 182, 5);
    pub const XP_FG: SpriteRect = Self::rect(0, 56, 182, 5);
    pub const INVENTORY_BG: SpriteRect = Self::rect(0, 512, 352, 332);
    pub const SLOT: SpriteRect = Self::rect(16, 680, 36, 36); // storage row0 col0 slot from INVENTORY_BG
    pub const CREATIVE_INVENTORY_BG: SpriteRect = Self::rect(360, 512, 390, 272);
    pub const STEVE_SKIN: SpriteRect = Self::rect(512, 0, 64, 64);
    pub const BUTTON: SpriteRect = Self::rect(0, 200, 200, 20);
    pub const BUTTON_HL: SpriteRect = Self::rect(0, 225, 200, 20);
    pub const MENU_BG: SpriteRect = Self::rect(210, 200, 16, 16);
    pub const LOGO: SpriteRect = Self::rect(0, 850, 256, 44);

    pub const fn ascii_char(c: u8) -> SpriteRect {
        let col = (c % 16) as u32;
        let row = (c / 16) as u32;
        Self::rect(col * 8, 64 + row * 8, 8, 8)
    }

    pub const fn rect(x: u32, y: u32, w: u32, h: u32) -> SpriteRect {
        SpriteRect { u0: x as f32 / Self::WIDTH as f32, v0: y as f32 / Self::HEIGHT as f32, u1: (x + w) as f32 / Self::WIDTH as f32, v1: (y + h) as f32 / Self::HEIGHT as f32 }
    }

    pub fn blit(atlas: &mut [u8], img_bytes: &[u8], dst_x: u32, dst_y: u32) {
        if let Ok(img) = image::load_from_memory(img_bytes) {
            for y in 0..img.height() {
                for x in 0..img.width() {
                    let (ax, ay) = (dst_x + x, dst_y + y);
                    if ax < Self::WIDTH && ay < Self::HEIGHT {
                        let idx = ((ay * Self::WIDTH + ax) * 4) as usize;
                        atlas[idx..idx + 4].copy_from_slice(&img.get_pixel(x, y).0);
                    }
                }
            }
        }
    }

    pub fn generate_atlas_pixels() -> Vec<u8> {
        let mut p = vec![0u8; (Self::WIDTH * Self::HEIGHT * 4) as usize];
        Self::blit(&mut p, include_bytes!("../../../assets/textures/hotbar.png"), 0, 0);
        Self::blit(&mut p, include_bytes!("../../../assets/textures/hotbar_selection.png"), 0, 24);
        Self::blit(&mut p, include_bytes!("../../../assets/textures/heart_bg.png"), 30, 24);
        Self::blit(&mut p, include_bytes!("../../../assets/textures/heart_full.png"), 40, 24);
        Self::blit(&mut p, include_bytes!("../../../assets/textures/heart_half.png"), 50, 24);
        Self::blit(&mut p, include_bytes!("../../../assets/textures/food_bg.png"), 60, 24);
        Self::blit(&mut p, include_bytes!("../../../assets/textures/food_full.png"), 70, 24);
        Self::blit(&mut p, include_bytes!("../../../assets/textures/crosshair.png"), 80, 24);
        Self::blit(&mut p, include_bytes!("../../../assets/textures/xp_bg.png"), 0, 50);
        Self::blit(&mut p, include_bytes!("../../../assets/textures/xp_fg.png"), 0, 56);
        Self::blit(&mut p, include_bytes!("../../../assets/textures/font/ascii.png"), 0, 64);
        super::atlas_items::blit_items_and_player(&mut p);
        super::atlas_more_items::blit_survival_and_food_items(&mut p);
        super::atlas_tools::blit_all_tool_sprites(&mut p);
        super::atlas_menu::blit_menu_textures(&mut p);
        p
    }
}