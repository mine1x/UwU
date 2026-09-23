use bevy::prelude::*;
use std::sync::{Arc, Mutex};
use crate::network::RemotePlayer;

#[derive(Resource, Default, Clone)]
pub struct LanNetworkResource {
    pub remote_players: Arc<Mutex<Vec<RemotePlayer>>>,
    pub local_player_id: u64,
}

pub struct LanNetworkPlugin;

impl Plugin for LanNetworkPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LanNetworkResource>();
        app.add_systems(Update, lan_network_system);
    }
}

fn lan_network_system(net_res: Res<LanNetworkResource>) {
    let now = std::time::Instant::now();
    if let Ok(mut players) = net_res.remote_players.try_lock() {
        players.retain(|p| now.duration_since(p.last_seen).as_secs() < 3);
    }
}
