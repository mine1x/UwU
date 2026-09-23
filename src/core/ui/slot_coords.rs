pub fn get_slot_pos(idx: usize) -> Option<(f32, f32)> {
    if idx < 27 { // 0..27: Storage (3 rows of 9)
        let row = (idx / 9) as f32;
        let col = (idx % 9) as f32;
        Some((8.0 + col * 18.0, 84.0 + row * 18.0))
    } else if idx < 36 { // 27..36: Hotbar (1 row of 9)
        let col = (idx - 27) as f32;
        Some((8.0 + col * 18.0, 142.0))
    } else if idx < 40 { // 36..40: Armor (4 slots)
        let i = (idx - 36) as f32;
        Some((8.0, 8.0 + i * 18.0))
    } else if idx == 40 { // 40: Offhand
        Some((77.0, 62.0))
    } else if idx < 45 { // 41..45: Crafting 2x2
        let c = idx - 41;
        let col = (c % 2) as f32;
        let row = (c / 2) as f32;
        Some((98.0 + col * 18.0, 18.0 + row * 18.0))
    } else if idx == 45 { // 45: Result
        Some((154.0, 28.0))
    } else {
        None
    }
}
