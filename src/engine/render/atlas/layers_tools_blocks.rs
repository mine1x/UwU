use super::loader::load_atlas_layer;

pub fn append_tools_and_blocks(pixels: &mut Vec<u8>) {
    // 52: Diamond
    pixels.extend(load_atlas_layer(include_bytes!("../../../../assets/textures/item/diamond.png"), None, [70, 220, 220, 255]));

    // 53..=56: WoodenAxe, StoneAxe, IronAxe, DiamondAxe
    pixels.extend(load_atlas_layer(include_bytes!("../../../../assets/textures/item/wooden_axe.png"), None, [140, 100, 60, 255]));
    pixels.extend(load_atlas_layer(include_bytes!("../../../../assets/textures/item/stone_axe.png"), None, [130, 130, 130, 255]));
    pixels.extend(load_atlas_layer(include_bytes!("../../../../assets/textures/item/iron_axe.png"), None, [200, 200, 200, 255]));
    pixels.extend(load_atlas_layer(include_bytes!("../../../../assets/textures/item/diamond_axe.png"), None, [70, 220, 220, 255]));

    // 57..=60: WoodenShovel, StoneShovel, IronShovel, DiamondShovel
    pixels.extend(load_atlas_layer(include_bytes!("../../../../assets/textures/item/wooden_shovel.png"), None, [140, 100, 60, 255]));
    pixels.extend(load_atlas_layer(include_bytes!("../../../../assets/textures/item/stone_shovel.png"), None, [130, 130, 130, 255]));
    pixels.extend(load_atlas_layer(include_bytes!("../../../../assets/textures/item/iron_shovel.png"), None, [200, 200, 200, 255]));
    pixels.extend(load_atlas_layer(include_bytes!("../../../../assets/textures/item/diamond_shovel.png"), None, [70, 220, 220, 255]));

    // 61..=64: WoodenHoe, StoneHoe, IronHoe, DiamondHoe
    pixels.extend(load_atlas_layer(include_bytes!("../../../../assets/textures/item/wooden_hoe.png"), None, [140, 100, 60, 255]));
    pixels.extend(load_atlas_layer(include_bytes!("../../../../assets/textures/item/stone_hoe.png"), None, [130, 130, 130, 255]));
    pixels.extend(load_atlas_layer(include_bytes!("../../../../assets/textures/item/iron_hoe.png"), None, [200, 200, 200, 255]));
    pixels.extend(load_atlas_layer(include_bytes!("../../../../assets/textures/item/diamond_hoe.png"), None, [70, 220, 220, 255]));

    // 65: Torch
    pixels.extend(load_atlas_layer(include_bytes!("../../../../assets/textures/block/torch.png"), None, [240, 200, 60, 255]));

    // 66..=68: IronBlock, CoalBlock, DiamondBlock
    pixels.extend(load_atlas_layer(include_bytes!("../../../../assets/textures/block/iron_block.png"), None, [220, 220, 220, 255]));
    pixels.extend(load_atlas_layer(include_bytes!("../../../../assets/textures/block/coal_block.png"), None, [25, 25, 25, 255]));
    pixels.extend(load_atlas_layer(include_bytes!("../../../../assets/textures/block/diamond_block.png"), None, [100, 230, 230, 255]));
}
