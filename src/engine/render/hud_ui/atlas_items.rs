use image::GenericImageView;
use super::atlas::HudAtlas;

pub fn blit_scaled(atlas: &mut [u8], img_bytes: &[u8], dst_x: u32, dst_y: u32, scale: u32) {
    if let Ok(img) = image::load_from_memory(img_bytes) {
        for y in 0..img.height() {
            for x in 0..img.width() {
                let px = img.get_pixel(x, y).0;
                if px[3] == 0 { continue; }
                for dy in 0..scale {
                    for dx in 0..scale {
                        let (ax, ay) = (dst_x + x * scale + dx, dst_y + y * scale + dy);
                        if ax < HudAtlas::WIDTH && ay < HudAtlas::HEIGHT {
                            let idx = ((ay * HudAtlas::WIDTH + ax) * 4) as usize;
                            atlas[idx..idx + 4].copy_from_slice(&px);
                        }
                    }
                }
            }
        }
    }
}

pub fn blit_items_and_player(p: &mut [u8]) {
    let cm = crate::world::ColorMap::new();
    let gt = cm.get_grass_color(0.8, 0.4);
    let foliage_tint = cm.get_foliage_color(0.8, 0.4);
    let iso = super::iso_block::render_isometric_block;

    // 64x64 High-Res Isometric Blocks at y=320
    iso(p, include_bytes!("../../../assets/textures/block/grass_top.png"), include_bytes!("../../../assets/textures/block/grass_side.png"), Some(gt), 0, 320, HudAtlas::WIDTH);
    iso(p, include_bytes!("../../../assets/textures/block/dirt.png"), include_bytes!("../../../assets/textures/block/dirt.png"), None, 64, 320, HudAtlas::WIDTH);
    iso(p, include_bytes!("../../../assets/textures/block/wood.png"), include_bytes!("../../../assets/textures/block/wood.png"), None, 128, 320, HudAtlas::WIDTH);
    iso(p, include_bytes!("../../../assets/textures/block/stone.png"), include_bytes!("../../../assets/textures/block/stone.png"), None, 192, 320, HudAtlas::WIDTH);
    iso(p, include_bytes!("../../../assets/textures/block/water_still.png"), include_bytes!("../../../assets/textures/block/water_still.png"), Some([0.247, 0.463, 0.894]), 256, 320, HudAtlas::WIDTH);
    iso(p, include_bytes!("../../../assets/textures/block/oak_log_top.png"), include_bytes!("../../../assets/textures/block/oak_log.png"), None, 320, 320, HudAtlas::WIDTH);
    iso(p, include_bytes!("../../../assets/textures/block/oak_planks.png"), include_bytes!("../../../assets/textures/block/oak_planks.png"), None, 384, 320, HudAtlas::WIDTH);
    iso(p, include_bytes!("../../../assets/textures/block/oak_leaves.png"), include_bytes!("../../../assets/textures/block/oak_leaves.png"), Some(foliage_tint), 448, 320, HudAtlas::WIDTH);

    // 64x64 High-Res 2D Items (4x scaled, crisp pixel-art) at y=320
    blit_scaled(p, include_bytes!("../../../assets/textures/item/apple.png"), 512, 320, 4);
    blit_scaled(p, include_bytes!("../../../assets/textures/item/bread.png"), 576, 320, 4);
    blit_scaled(p, include_bytes!("../../../assets/textures/item/bucket.png"), 640, 320, 4);
    blit_scaled(p, include_bytes!("../../../assets/textures/item/water_bucket.png"), 704, 320, 4);
    blit_scaled(p, include_bytes!("../../../assets/textures/item/diamond_sword.png"), 768, 320, 4);
    blit_scaled(p, include_bytes!("../../../assets/textures/item/diamond_pickaxe.png"), 832, 320, 4);

    // Steve 64x64 skin at (512, 0)
    blit_scaled(p, include_bytes!("../../../assets/textures/entity/player/steve.png"), 512, 0, 1);

    // Inventory GUI background (2x scaled = 352x332) at (0, 512)
    blit_scaled(p, include_bytes!("../../../assets/textures/gui/container/inventory.png"), 0, 512, 2);

    // Creative Inventory GUI background (2x scaled = 390x272) at (360, 512)
    blit_scaled(p, include_bytes!("../../../assets/textures/gui/container/creative_inventory/tab_items.png"), 360, 512, 2);
}
