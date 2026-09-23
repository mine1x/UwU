use super::block::BlockType;

pub fn block_color(b: &BlockType) -> [f32; 3] {
    match b {
        BlockType::Air => [0.0, 0.0, 0.0],
        BlockType::Grass => [0.28, 0.65, 0.22],
        BlockType::Dirt => [0.45, 0.28, 0.15],
        BlockType::Wood | BlockType::OakPlanks => [0.55, 0.38, 0.22],
        BlockType::Stone => [0.52, 0.52, 0.54],
        BlockType::Cobblestone => [0.48, 0.48, 0.48],
        BlockType::CraftingTable => [0.62, 0.45, 0.28],
        BlockType::Furnace => [0.42, 0.42, 0.42],
        BlockType::Chest => [0.58, 0.38, 0.18],
        BlockType::WaterSource => [0.12, 0.35, 0.88],
        BlockType::FlowingWater { .. } => [0.20, 0.48, 0.90],
        BlockType::OakLog => [0.40, 0.28, 0.16],
        BlockType::OakLeaves => [0.22, 0.48, 0.16],
        BlockType::Torch => [0.95, 0.80, 0.25],
        BlockType::IronBlock => [0.85, 0.85, 0.85],
        BlockType::CoalBlock => [0.10, 0.10, 0.10],
        BlockType::DiamondBlock => [0.38, 0.88, 0.85],
    }
}

pub fn block_name(b: &BlockType) -> &'static str {
    match b {
        BlockType::Air => "Air",
        BlockType::Grass => "Grass Block",
        BlockType::Dirt => "Dirt",
        BlockType::Wood => "Wood",
        BlockType::Stone => "Stone",
        BlockType::Cobblestone => "Cobblestone",
        BlockType::CraftingTable => "Crafting Table",
        BlockType::Furnace => "Furnace",
        BlockType::Chest => "Chest",
        BlockType::WaterSource => "Water Source (L8)",
        BlockType::FlowingWater { level } => match level {
            7 => "Flowing Water L7", 6 => "Flowing Water L6",
            5 => "Flowing Water L5", 4 => "Flowing Water L4",
            3 => "Flowing Water L3", 2 => "Flowing Water L2",
            _ => "Flowing Water L1",
        },
        BlockType::OakLog => "Oak Log",
        BlockType::OakPlanks => "Oak Planks",
        BlockType::OakLeaves => "Oak Leaves",
        BlockType::Torch => "Torch",
        BlockType::IronBlock => "Block of Iron",
        BlockType::CoalBlock => "Block of Coal",
        BlockType::DiamondBlock => "Block of Diamond",
    }
}
