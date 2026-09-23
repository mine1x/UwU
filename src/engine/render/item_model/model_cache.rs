use std::collections::HashMap;
use std::sync::Mutex;
use crate::engine::items::ItemType;
use super::extruder::{generate_extruded_item_model, ModelQuad};
use super::registry::get_item_texture_bytes;

pub fn get_item_model(item: ItemType) -> &'static [ModelQuad] {
    static CACHE: Mutex<Option<HashMap<ItemType, &'static [ModelQuad]>>> = Mutex::new(None);
    let mut lock = CACHE.lock().unwrap();
    let map = lock.get_or_insert_with(HashMap::new);
    if let Some(&quads) = map.get(&item) {
        return quads;
    }
    let quads: &'static [ModelQuad] = if let Some(bytes) = get_item_texture_bytes(item) {
        let baked = generate_extruded_item_model(bytes);
        Box::leak(baked.into_boxed_slice())
    } else {
        &[]
    };
    map.insert(item, quads);
    quads
}
