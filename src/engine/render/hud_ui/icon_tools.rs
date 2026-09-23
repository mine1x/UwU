use super::atlas::{HudAtlas, SpriteRect};
use crate::engine::items::ItemType;

pub fn get_tool_sprite(it: &ItemType) -> Option<SpriteRect> {
    match it {
        ItemType::WoodenSword => Some(HudAtlas::rect(256, 448, 64, 64)),
        ItemType::StoneSword => Some(HudAtlas::rect(320, 448, 64, 64)),
        ItemType::IronSword => Some(HudAtlas::rect(384, 448, 64, 64)),
        ItemType::DiamondSword => Some(HudAtlas::rect(768, 320, 64, 64)),

        ItemType::WoodenPickaxe => Some(HudAtlas::rect(448, 448, 64, 64)),
        ItemType::StonePickaxe => Some(HudAtlas::rect(512, 448, 64, 64)),
        ItemType::IronPickaxe => Some(HudAtlas::rect(576, 448, 64, 64)),
        ItemType::DiamondPickaxe => Some(HudAtlas::rect(832, 320, 64, 64)),

        ItemType::WoodenAxe => Some(HudAtlas::rect(640, 448, 64, 64)),
        ItemType::StoneAxe => Some(HudAtlas::rect(704, 448, 64, 64)),
        ItemType::IronAxe => Some(HudAtlas::rect(768, 448, 64, 64)),
        ItemType::DiamondAxe => Some(HudAtlas::rect(832, 448, 64, 64)),

        ItemType::WoodenShovel => Some(HudAtlas::rect(896, 448, 64, 64)),
        ItemType::StoneShovel => Some(HudAtlas::rect(960, 448, 64, 64)),
        ItemType::IronShovel => Some(HudAtlas::rect(0, 256, 64, 64)),
        ItemType::DiamondShovel => Some(HudAtlas::rect(64, 256, 64, 64)),

        ItemType::WoodenHoe => Some(HudAtlas::rect(128, 256, 64, 64)),
        ItemType::StoneHoe => Some(HudAtlas::rect(192, 256, 64, 64)),
        ItemType::IronHoe => Some(HudAtlas::rect(256, 256, 64, 64)),
        ItemType::DiamondHoe => Some(HudAtlas::rect(320, 256, 64, 64)),
        _ => None,
    }
}
