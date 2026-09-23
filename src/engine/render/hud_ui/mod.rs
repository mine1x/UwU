pub mod atlas;
pub mod atlas_items;
pub mod atlas_menu;
pub mod atlas_more_items;
pub mod atlas_tools;
pub mod bars;
pub mod font;
pub mod icon_blocks;
pub mod icon_tools;
pub mod iso_block;
pub mod item_icon;
pub mod piechart;
pub mod renderer;
pub mod resources;
pub mod slot_render;

pub use atlas::{HudAtlas, SpriteRect};
pub use bars::{add_hud_quad, add_textured_quad, draw_bars_and_crosshair};
pub use font::draw_text;
pub use icon_blocks::get_block_sprite;
pub use icon_tools::get_tool_sprite;
pub use item_icon::get_item_sprite;
pub use piechart::draw_profiler_piechart;
pub use renderer::{HudRenderer, HudVertex};
pub use slot_render::draw_slot_item_and_count;

#[cfg(test)]
mod tests;