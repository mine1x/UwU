use bevy::prelude::*;
use super::plugins::*;

pub fn build_game_app() -> App {
    let mut app = App::new();
    app.add_plugins(bevy::time::TimePlugin);
    app.add_plugins(VoxelWorldPlugin);
    app.add_plugins(ItemEntityPlugin);
    app.add_plugins(LanNetworkPlugin);
    app.add_plugins(WgpuRunnerPlugin);
    app
}

pub fn run_bevy_engine() {
    let mut app = build_game_app();
    app.run();
}
