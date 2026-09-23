use super::block::BlockType;

pub fn block_to_u8(b: &BlockType) -> u8 {
    match b {
        BlockType::Air => 0,
        BlockType::Dirt => 1,
        BlockType::Grass => 2,
        BlockType::Wood => 3,
        BlockType::Stone => 4,
        BlockType::WaterSource => 5,
        BlockType::FlowingWater { level } => 6 + level.min(&7).saturating_sub(1),
        BlockType::OakLog => 14,
        BlockType::OakPlanks => 15,
        BlockType::OakLeaves => 16,
        BlockType::Cobblestone => 17,
        BlockType::CraftingTable => 18,
        BlockType::Furnace => 19,
        BlockType::Chest => 20,
        BlockType::Torch => 21,
        BlockType::IronBlock => 22,
        BlockType::CoalBlock => 23,
        BlockType::DiamondBlock => 24,
    }
}

pub fn block_from_u8(val: u8) -> BlockType {
    match val {
        0 => BlockType::Air,
        1 => BlockType::Dirt,
        2 => BlockType::Grass,
        3 => BlockType::Wood,
        4 => BlockType::Stone,
        5 => BlockType::WaterSource,
        6..=13 => BlockType::FlowingWater { level: (val - 5).min(7) },
        14 => BlockType::OakLog,
        15 => BlockType::OakPlanks,
        16 => BlockType::OakLeaves,
        17 => BlockType::Cobblestone,
        18 => BlockType::CraftingTable,
        19 => BlockType::Furnace,
        20 => BlockType::Chest,
        21 => BlockType::Torch,
        22 => BlockType::IronBlock,
        23 => BlockType::CoalBlock,
        24 => BlockType::DiamondBlock,
        _ => BlockType::Dirt,
    }
}
