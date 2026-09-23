use bevy::math::Vec3;

#[derive(Clone, Debug, PartialEq)]
pub enum Packet {
    ClientboundLogin { entity_id: i32 },
    ClientboundAddEntity { entity_id: i32, x: f64, y: f64, z: f64, yaw: f32, pitch: f32 },
    ClientboundEntityPositionSync {
        entity_id: i32, pos: Vec3, yaw: f32, pitch: f32,
        held_item: Option<u8>, is_sneaking: bool, is_sprinting: bool, mining_swing: f32,
    },
    ClientboundBlockUpdate { x: i32, y: i32, z: i32, block_type: u8 },
    ClientboundChunkData { x: i32, y: i32, z: i32, runs: Vec<(u16, u8)> },
    ClientboundRemoveEntities { entity_ids: Vec<i32> },
    ClientboundSpawnItem { entity_id: i32, pos: Vec3, item_type: u8, count: u32 },

    ServerboundHello { name: String },
    ServerboundMovePlayerPosRot {
        pos: Vec3, yaw: f32, pitch: f32,
        held_item: Option<u8>, is_sneaking: bool, is_sprinting: bool, mining_swing: f32,
    },
    ServerboundPlayerAction { action: i32, x: i32, y: i32, z: i32 },
    ServerboundUseItemOn { x: i32, y: i32, z: i32, block_type: u8 },
}

impl Packet {
    pub fn encode(&self) -> (i32, Vec<u8>) {
        super::packet_codec::encode_packet(self)
    }

    pub fn decode(packet_id: i32, payload: &[u8]) -> Option<Self> {
        super::packet_codec::decode_packet(packet_id, payload)
    }
}
