use super::atlas::HudAtlas;
use super::atlas_items::blit_scaled;

pub fn blit_survival_and_food_items(p: &mut [u8]) {
    let iso = super::iso_block::render_isometric_block;

    // Row B: Blocks at y=384
    iso(p, include_bytes!("../../../assets/textures/block/cobblestone.png"), include_bytes!("../../../assets/textures/block/cobblestone.png"), None, 0, 384, HudAtlas::WIDTH);
    iso(p, include_bytes!("../../../assets/textures/block/crafting_table_top.png"), include_bytes!("../../../assets/textures/block/crafting_table_side.png"), None, 64, 384, HudAtlas::WIDTH);
    iso(p, include_bytes!("../../../assets/textures/block/furnace_top.png"), include_bytes!("../../../assets/textures/block/furnace_front.png"), None, 128, 384, HudAtlas::WIDTH);
    iso(p, include_bytes!("../../../assets/textures/block/oak_planks.png"), include_bytes!("../../../assets/textures/block/oak_planks.png"), None, 192, 384, HudAtlas::WIDTH);
    blit_scaled(p, include_bytes!("../../../assets/textures/block/torch.png"), 256, 384, 4);
    iso(p, include_bytes!("../../../assets/textures/block/iron_block.png"), include_bytes!("../../../assets/textures/block/iron_block.png"), None, 320, 384, HudAtlas::WIDTH);
    iso(p, include_bytes!("../../../assets/textures/block/coal_block.png"), include_bytes!("../../../assets/textures/block/coal_block.png"), None, 384, 384, HudAtlas::WIDTH);
    iso(p, include_bytes!("../../../assets/textures/block/diamond_block.png"), include_bytes!("../../../assets/textures/block/diamond_block.png"), None, 448, 384, HudAtlas::WIDTH);

    // Row B: Items & Foods at y=384
    blit_scaled(p, include_bytes!("../../../assets/textures/item/coal.png"), 512, 384, 4);
    blit_scaled(p, include_bytes!("../../../assets/textures/item/stick.png"), 576, 384, 4);
    blit_scaled(p, include_bytes!("../../../assets/textures/item/iron_ingot.png"), 640, 384, 4);
    blit_scaled(p, include_bytes!("../../../assets/textures/item/diamond.png"), 704, 384, 4);
    blit_scaled(p, include_bytes!("../../../assets/textures/item/porkchop.png"), 768, 384, 4);
    blit_scaled(p, include_bytes!("../../../assets/textures/item/cooked_porkchop.png"), 832, 384, 4);
    blit_scaled(p, include_bytes!("../../../assets/textures/item/beef.png"), 896, 384, 4);
    blit_scaled(p, include_bytes!("../../../assets/textures/item/cooked_beef.png"), 960, 384, 4);

    // Row C: Foods & Materials at y=448
    blit_scaled(p, include_bytes!("../../../assets/textures/item/mutton.png"), 0, 448, 4);
    blit_scaled(p, include_bytes!("../../../assets/textures/item/cooked_mutton.png"), 64, 448, 4);
    iso(p, include_bytes!("../../../assets/textures/block/white_wool.png"), include_bytes!("../../../assets/textures/block/white_wool.png"), None, 128, 448, HudAtlas::WIDTH);
    blit_scaled(p, include_bytes!("../../../assets/textures/item/leather.png"), 192, 448, 4);
}
