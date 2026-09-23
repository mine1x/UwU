#[derive(bevy::prelude::Component, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum BlockType {
    Air,
    Dirt,
    Grass,
    Wood,
    Stone,
    Cobblestone,
    CraftingTable,
    Furnace,
    Chest,
    WaterSource, // Level 8
    FlowingWater { level: u8 }, // Level 1..=7
    OakLog,
    OakPlanks,
    OakLeaves,
    Torch,
    IronBlock,
    CoalBlock,
    DiamondBlock,
}

impl BlockType {
    pub fn is_solid(&self) -> bool {
        !matches!(self, BlockType::Air | BlockType::WaterSource | BlockType::FlowingWater { .. })
    }

    pub fn is_fluid(&self) -> bool {
        matches!(self, BlockType::WaterSource | BlockType::FlowingWater { .. })
    }

    pub fn is_transparent(&self) -> bool {
        matches!(self, BlockType::Air | BlockType::WaterSource | BlockType::FlowingWater { .. } | BlockType::OakLeaves | BlockType::Torch)
    }

    pub fn tex_layer(&self) -> f32 {
        match self {
            BlockType::Grass => 0.0,
            BlockType::Dirt => 1.0,
            BlockType::Wood | BlockType::OakPlanks => 2.0,
            BlockType::Stone => 3.0,
            BlockType::WaterSource => 4.0,
            BlockType::FlowingWater { .. } => 5.0,
            BlockType::OakLog => 14.0,
            BlockType::OakLeaves => 17.0,
            BlockType::Cobblestone => 28.0,
            BlockType::CraftingTable => 29.0,
            BlockType::Furnace => 31.0,
            BlockType::Chest => 35.0,
            BlockType::Torch => 65.0,
            BlockType::IronBlock => 66.0,
            BlockType::CoalBlock => 67.0,
            BlockType::DiamondBlock => 68.0,
            BlockType::Air => -1.0,
        }
    }

    pub fn color(&self) -> [f32; 3] {
        super::block_props::block_color(self)
    }

    pub fn name(&self) -> &'static str {
        super::block_props::block_name(self)
    }
}