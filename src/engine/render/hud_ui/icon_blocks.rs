use super::atlas::{HudAtlas, SpriteRect};
use crate::world::block::BlockType;

pub fn get_block_sprite(b: &BlockType) -> SpriteRect {
    match b {
        BlockType::Grass => HudAtlas::rect(0, 320, 64, 64),
        BlockType::Dirt => HudAtlas::rect(64, 320, 64, 64),
        BlockType::Wood => HudAtlas::rect(128, 320, 64, 64),
        BlockType::Stone => HudAtlas::rect(192, 320, 64, 64),
        BlockType::WaterSource | BlockType::FlowingWater { .. } => HudAtlas::rect(256, 320, 64, 64),
        BlockType::OakLog => HudAtlas::rect(320, 320, 64, 64),
        BlockType::OakPlanks => HudAtlas::rect(384, 320, 64, 64),
        BlockType::OakLeaves => HudAtlas::rect(448, 320, 64, 64),
        BlockType::Cobblestone => HudAtlas::rect(0, 384, 64, 64),
        BlockType::CraftingTable => HudAtlas::rect(64, 384, 64, 64),
        BlockType::Furnace => HudAtlas::rect(128, 384, 64, 64),
        BlockType::Chest => HudAtlas::rect(192, 384, 64, 64),
        BlockType::Torch => HudAtlas::rect(256, 384, 64, 64),
        BlockType::IronBlock => HudAtlas::rect(320, 384, 64, 64),
        BlockType::CoalBlock => HudAtlas::rect(384, 384, 64, 64),
        BlockType::DiamondBlock => HudAtlas::rect(448, 384, 64, 64),
        _ => HudAtlas::rect(0, 320, 64, 64),
    }
}
