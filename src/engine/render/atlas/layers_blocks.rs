use super::loader::load_atlas_layer;

pub fn append_initial_blocks(pixels: &mut Vec<u8>, grass_tint: [f32; 3]) {
    let water_tint = [0.247, 0.463, 0.894];
    pixels.extend(load_atlas_layer(include_bytes!("../../../../assets/textures/block/grass_top.png"), Some(grass_tint), [121, 192, 90, 255]));
    pixels.extend(load_atlas_layer(include_bytes!("../../../../assets/textures/block/dirt.png"), None, [120, 80, 45, 255]));
    pixels.extend(load_atlas_layer(include_bytes!("../../../../assets/textures/block/wood.png"), None, [160, 115, 65, 255]));
    pixels.extend(load_atlas_layer(include_bytes!("../../../../assets/textures/block/stone.png"), None, [130, 130, 130, 255]));
    pixels.extend(load_atlas_layer(include_bytes!("../../../../assets/textures/block/water_still.png"), Some(water_tint), [40, 100, 220, 210]));
    pixels.extend(load_atlas_layer(include_bytes!("../../../../assets/textures/block/water_flow.png"), Some(water_tint), [55, 120, 230, 210]));
    pixels.extend(load_atlas_layer(include_bytes!("../../../../assets/textures/entity/player/steve.png"), None, [180, 100, 80, 255]));
    pixels.extend(load_atlas_layer(include_bytes!("../../../../assets/textures/block/grass_side.png"), None, [130, 95, 60, 255]));
}

pub fn append_wood_leaves_and_functional_blocks(pixels: &mut Vec<u8>, foliage_tint: [f32; 3]) {
    // 14..=17: OakLog, OakLogTop, OakPlanks, OakLeaves
    pixels.extend(load_atlas_layer(include_bytes!("../../../../assets/textures/block/oak_log.png"), None, [140, 100, 60, 255]));
    pixels.extend(load_atlas_layer(include_bytes!("../../../../assets/textures/block/oak_log_top.png"), None, [160, 130, 80, 255]));
    pixels.extend(load_atlas_layer(include_bytes!("../../../../assets/textures/block/oak_planks.png"), None, [170, 135, 85, 255]));
    pixels.extend(load_atlas_layer(include_bytes!("../../../../assets/textures/block/oak_leaves.png"), Some(foliage_tint), [60, 140, 40, 255]));
}

pub fn append_survival_blocks_and_entities(pixels: &mut Vec<u8>) {
    // 28..=35: Cobblestone, CraftingTableTop, CraftingTableSide, FurnaceFront, FurnaceSide, FurnaceTop, WhiteWool, Chest
    pixels.extend(load_atlas_layer(include_bytes!("../../../../assets/textures/block/cobblestone.png"), None, [128, 128, 128, 255]));
    pixels.extend(load_atlas_layer(include_bytes!("../../../../assets/textures/block/crafting_table_top.png"), None, [160, 115, 65, 255]));
    pixels.extend(load_atlas_layer(include_bytes!("../../../../assets/textures/block/crafting_table_side.png"), None, [140, 100, 60, 255]));
    pixels.extend(load_atlas_layer(include_bytes!("../../../../assets/textures/block/furnace_front.png"), None, [100, 100, 100, 255]));
    pixels.extend(load_atlas_layer(include_bytes!("../../../../assets/textures/block/furnace_side.png"), None, [110, 110, 110, 255]));
    pixels.extend(load_atlas_layer(include_bytes!("../../../../assets/textures/block/furnace_top.png"), None, [120, 120, 120, 255]));
    pixels.extend(load_atlas_layer(include_bytes!("../../../../assets/textures/block/white_wool.png"), None, [230, 230, 230, 255]));
    pixels.extend(load_atlas_layer(include_bytes!("../../../../assets/textures/entity/chest.png"), None, [160, 115, 65, 255]));
}
