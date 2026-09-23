use bevy::math::Vec3;
use winit::event::MouseButton;

pub enum LogicCommand {
    MoveInput(Vec3),
    Jump(bool),
    Sneak(bool),
    Sprint(bool),
    RotateCamera(f32),
    ZoomCamera(f32),
    ToggleInventory,
    CloseInventory,
    UpdateCursor {
        aspect: f32,
        mouse_pos: (f32, f32),
        screen_size: (f32, f32),
    },
    MouseAction {
        button: MouseButton,
        is_pressed: bool,
        aspect: f32,
        mouse_pos: (f32, f32),
        screen_size: (f32, f32),
        is_shift: bool,
    },
    SelectSlot(usize),
    NextSlot,
    PrevSlot,
    TogglePieChart,
    ToggleChunkBorders,
    ProfilerNavigate(usize),
    RemoteBlockChange { x: i32, y: i32, z: i32, block_type: u8 },
    RemoteChunkData { x: i32, y: i32, z: i32, runs: Vec<(u16, u8)> },
    RemoteSpawnItem { entity_id: i32, pos: Vec3, item_type: u8, count: u32 },
    RemoteRemoveEntities { entity_ids: Vec<i32> },
    ToggleGameMode,
    ToggleFlight,
}