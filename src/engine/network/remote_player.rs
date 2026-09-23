use bevy::math::Vec3;
use bevy::prelude::Component;
use std::time::Instant;
use crate::engine::player_ctrl::def::Player;
use crate::engine::items::ItemType;

#[derive(Component, Clone, Debug)]
pub struct RemotePlayer {
    pub id: u64,
    pub name: String,
    pub pos: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub held_item: Option<ItemType>,
    pub is_sneaking: bool,
    pub is_sprinting: bool,
    pub mining_swing: f32,
    pub last_seen: Instant,
}

impl RemotePlayer {
    pub fn new(id: u64, name: String, pos: Vec3) -> Self {
        Self {
            id, name, pos, yaw: 0.0, pitch: 0.0,
            held_item: None, is_sneaking: false, is_sprinting: false,
            mining_swing: 0.0, last_seen: Instant::now(),
        }
    }

    pub fn to_player(&self) -> Player {
        let mut p = Player::new(self.pos.x, self.pos.y, self.pos.z);
        p.yaw = self.yaw;
        p.held_item = self.held_item;
        p.is_sneaking = self.is_sneaking;
        p.is_sprinting = self.is_sprinting;
        p.mining_swing = self.mining_swing;
        p
    }
}
