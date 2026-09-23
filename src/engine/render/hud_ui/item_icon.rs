use super::atlas::{HudAtlas, SpriteRect};
use crate::engine::items::ItemType;

pub fn get_item_sprite(item: &ItemType) -> SpriteRect {
    if let Some(tool_sprite) = super::icon_tools::get_tool_sprite(item) {
        return tool_sprite;
    }
    match item {
        ItemType::Block(b) => super::icon_blocks::get_block_sprite(b),
        ItemType::Apple => HudAtlas::rect(512, 320, 64, 64),
        ItemType::Bread => HudAtlas::rect(576, 320, 64, 64),
        ItemType::Bucket => HudAtlas::rect(640, 320, 64, 64),
        ItemType::WaterBucket => HudAtlas::rect(704, 320, 64, 64),
        ItemType::Coal => HudAtlas::rect(512, 384, 64, 64),
        ItemType::Stick => HudAtlas::rect(576, 384, 64, 64),
        ItemType::IronIngot => HudAtlas::rect(640, 384, 64, 64),
        ItemType::Diamond => HudAtlas::rect(704, 384, 64, 64),
        ItemType::Porkchop => HudAtlas::rect(768, 384, 64, 64),
        ItemType::CookedPorkchop => HudAtlas::rect(832, 384, 64, 64),
        ItemType::Beef => HudAtlas::rect(896, 384, 64, 64),
        ItemType::CookedBeef => HudAtlas::rect(960, 384, 64, 64),
        ItemType::Mutton => HudAtlas::rect(0, 448, 64, 64),
        ItemType::CookedMutton => HudAtlas::rect(64, 448, 64, 64),
        ItemType::WhiteWool => HudAtlas::rect(128, 448, 64, 64),
        ItemType::Leather => HudAtlas::rect(192, 448, 64, 64),
        _ => HudAtlas::rect(0, 320, 64, 64),
    }
}
