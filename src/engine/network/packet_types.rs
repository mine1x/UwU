// Minecraft Java Protocol Packet Types
// Reference: net.minecraft.network.protocol.game.GamePacketTypes

pub const CB_LOGIN: i32 = 0x02;
pub const CB_ADD_ENTITY: i32 = 0x01;
pub const CB_ENTITY_POSITION_SYNC: i32 = 0x1D;
pub const CB_BLOCK_UPDATE: i32 = 0x09;
pub const CB_ANIMATE: i32 = 0x03;
pub const CB_REMOVE_ENTITIES: i32 = 0x42;
pub const CB_CHUNK_DATA: i32 = 0x21;
pub const CB_SPAWN_ITEM: i32 = 0x40;

pub const SB_HELLO: i32 = 0x00;
pub const SB_MOVE_PLAYER_POS_ROT: i32 = 0x1A;
pub const SB_PLAYER_ACTION: i32 = 0x24;
pub const SB_USE_ITEM_ON: i32 = 0x38;
pub const SB_SWING: i32 = 0x36;
