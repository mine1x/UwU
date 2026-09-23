pub mod key_events;
pub mod key_shortcuts;
pub mod menu_action_handler;
pub mod mouse_actions;
pub mod mouse_input_handler;
pub mod text_input;

pub use key_events::handle_keyboard_input;
pub use menu_action_handler::apply_menu_action;
pub use mouse_actions::handle_mouse_action;
pub use mouse_input_handler::handle_mouse_input;
