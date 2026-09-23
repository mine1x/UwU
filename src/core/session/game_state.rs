use bevy::prelude::Resource;

#[derive(Resource, Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    TitleScreen,
    LanLobby,
    DirectConnect,
    Playing,
    Paused,
}
