use super::atlas_items::blit_scaled;

pub fn blit_all_tool_sprites(p: &mut [u8]) {
    // Row C: Swords, Pickaxes, Axes, Shovels at y=448
    blit_scaled(p, include_bytes!("../../../assets/textures/item/wooden_sword.png"), 256, 448, 4);
    blit_scaled(p, include_bytes!("../../../assets/textures/item/stone_sword.png"), 320, 448, 4);
    blit_scaled(p, include_bytes!("../../../assets/textures/item/iron_sword.png"), 384, 448, 4);

    blit_scaled(p, include_bytes!("../../../assets/textures/item/wooden_pickaxe.png"), 448, 448, 4);
    blit_scaled(p, include_bytes!("../../../assets/textures/item/stone_pickaxe.png"), 512, 448, 4);
    blit_scaled(p, include_bytes!("../../../assets/textures/item/iron_pickaxe.png"), 576, 448, 4);

    blit_scaled(p, include_bytes!("../../../assets/textures/item/wooden_axe.png"), 640, 448, 4);
    blit_scaled(p, include_bytes!("../../../assets/textures/item/stone_axe.png"), 704, 448, 4);
    blit_scaled(p, include_bytes!("../../../assets/textures/item/iron_axe.png"), 768, 448, 4);
    blit_scaled(p, include_bytes!("../../../assets/textures/item/diamond_axe.png"), 832, 448, 4);

    blit_scaled(p, include_bytes!("../../../assets/textures/item/wooden_shovel.png"), 896, 448, 4);
    blit_scaled(p, include_bytes!("../../../assets/textures/item/stone_shovel.png"), 960, 448, 4);

    // Row D: Iron/Diamond Shovels & All Hoes at y=256
    blit_scaled(p, include_bytes!("../../../assets/textures/item/iron_shovel.png"), 0, 256, 4);
    blit_scaled(p, include_bytes!("../../../assets/textures/item/diamond_shovel.png"), 64, 256, 4);

    blit_scaled(p, include_bytes!("../../../assets/textures/item/wooden_hoe.png"), 128, 256, 4);
    blit_scaled(p, include_bytes!("../../../assets/textures/item/stone_hoe.png"), 192, 256, 4);
    blit_scaled(p, include_bytes!("../../../assets/textures/item/iron_hoe.png"), 256, 256, 4);
    blit_scaled(p, include_bytes!("../../../assets/textures/item/diamond_hoe.png"), 320, 256, 4);
}
