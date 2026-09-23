use std::collections::HashMap;
use std::sync::atomic::AtomicBool;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Instant;
use winit::keyboard::KeyCode;
use winit::window::Window;

use crate::engine::RenderSnapshot;
use crate::input::LogicCommand;
use crate::render::Renderer;
use crate::world::ConcurrentChunkStorage;

pub struct App {
    pub window: Option<Arc<Window>>,
    pub renderer: Option<Renderer>,
    pub latest_snapshot: Arc<Mutex<Option<RenderSnapshot>>>,
    pub command_tx: Option<mpsc::Sender<LogicCommand>>,
    pub running: Arc<AtomicBool>,
    pub logic_thread: Option<thread::JoinHandle<()>>,
    pub keys_pressed: HashMap<KeyCode, bool>,
    pub mouse_pos: (f32, f32),
    pub is_middle_dragging: bool,
    pub last_drag_pos: (f32, f32),
    pub last_frame_time: Instant,
    pub game_state: super::game_state::GameState,
    pub lan_detector: Option<crate::network::LanServerDetector>,
    pub lan_server: Option<crate::network::LanServer>,
    pub lan_client: Option<crate::network::LanClient>,
    pub remote_players: Vec<crate::network::RemotePlayer>,
    pub direct_ip_input: String,
    pub local_player_id: u64,
    pub block_event_rx: Option<mpsc::Receiver<crate::network::Packet>>,
    pub world_storage: Arc<Mutex<Option<Arc<ConcurrentChunkStorage>>>>,
    pub last_space_time: Option<Instant>,
    pub inventory_open: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            window: None,
            renderer: None,
            latest_snapshot: Arc::new(Mutex::new(None)),
            command_tx: None,
            running: Arc::new(AtomicBool::new(true)),
            logic_thread: None,
            keys_pressed: HashMap::new(),
            mouse_pos: (0.0, 0.0),
            is_middle_dragging: false,
            last_drag_pos: (0.0, 0.0),
            last_frame_time: Instant::now(),
            game_state: super::game_state::GameState::TitleScreen,
            lan_detector: None,
            lan_server: None,
            lan_client: None,
            remote_players: Vec::new(),
            direct_ip_input: "127.0.0.1:25565".to_string(),
            local_player_id: 1,
            block_event_rx: None,
            world_storage: Arc::new(Mutex::new(None)),
            last_space_time: None,
            inventory_open: false,
        }
    }
}