use super::loader::load_atlas_layer;

pub fn append_legacy_items(pixels: &mut Vec<u8>) {
    // 8..=13: Apple, Bread, Bucket, WaterBucket, DiamondSword, DiamondPickaxe
    pixels.extend(load_atlas_layer(include_bytes!("../../../assets/textures/item/apple.png"), None, [220, 40, 40, 255]));
    pixels.extend(load_atlas_layer(include_bytes!("../../../assets/textures/item/bread.png"), None, [180, 130, 70, 255]));
    pixels.extend(load_atlas_layer(include_bytes!("../../../assets/textures/item/bucket.png"), None, [180, 180, 180, 255]));
    pixels.extend(load_atlas_layer(include_bytes!("../../../assets/textures/item/water_bucket.png"), None, [80, 140, 240, 255]));
    pixels.extend(load_atlas_layer(include_bytes!("../../../assets/textures/item/diamond_sword.png"), None, [70, 220, 220, 255]));
    pixels.extend(load_atlas_layer(include_bytes!("../../../assets/textures/item/diamond_pickaxe.png"), None, [70, 220, 220, 255]));
}

pub fn append_survival_items(pixels: &mut Vec<u8>) {
    // 36..=45: Coal, Stick, IronIngot, Porkchop, CookedPorkchop, Beef, CookedBeef, Mutton, CookedMutton, Leather
    pixels.extend(load_atlas_layer(include_bytes!("../../../assets/textures/item/coal.png"), None, [30, 30, 30, 255]));
    pixels.extend(load_atlas_layer(include_bytes!("../../../assets/textures/item/stick.png"), None, [140, 100, 60, 255]));
    pixels.extend(load_atlas_layer(include_bytes!("../../../assets/textures/item/iron_ingot.png"), None, [200, 200, 200, 255]));
    pixels.extend(load_atlas_layer(include_bytes!("../../../assets/textures/item/porkchop.png"), None, [230, 150, 150, 255]));
    pixels.extend(load_atlas_layer(include_bytes!("../../../assets/textures/item/cooked_porkchop.png"), None, [190, 100, 70, 255]));
    pixels.extend(load_atlas_layer(include_bytes!("../../../assets/textures/item/beef.png"), None, [180, 50, 50, 255]));
    pixels.extend(load_atlas_layer(include_bytes!("../../../assets/textures/item/cooked_beef.png"), None, [140, 65, 45, 255]));
    pixels.extend(load_atlas_layer(include_bytes!("../../../assets/textures/item/mutton.png"), None, [210, 110, 110, 255]));
    pixels.extend(load_atlas_layer(include_bytes!("../../../assets/textures/item/cooked_mutton.png"), None, [160, 85, 55, 255]));
    pixels.extend(load_atlas_layer(include_bytes!("../../../assets/textures/item/leather.png"), None, [160, 90, 50, 255]));

    // 46..=51: WoodenPickaxe, StonePickaxe, IronPickaxe, WoodenSword, StoneSword, IronSword
    pixels.extend(load_atlas_layer(include_bytes!("../../../assets/textures/item/wooden_pickaxe.png"), None, [140, 100, 60, 255]));
    pixels.extend(load_atlas_layer(include_bytes!("../../../assets/textures/item/stone_pickaxe.png"), None, [130, 130, 130, 255]));
    pixels.extend(load_atlas_layer(include_bytes!("../../../assets/textures/item/iron_pickaxe.png"), None, [200, 200, 200, 255]));
    pixels.extend(load_atlas_layer(include_bytes!("../../../assets/textures/item/wooden_sword.png"), None, [140, 100, 60, 255]));
    pixels.extend(load_atlas_layer(include_bytes!("../../../assets/textures/item/stone_sword.png"), None, [130, 130, 130, 255]));
    pixels.extend(load_atlas_layer(include_bytes!("../../../assets/textures/item/iron_sword.png"), None, [200, 200, 200, 255]));
}
