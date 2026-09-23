use super::block::BlockType;

pub fn compute_face_texture(
    block: BlockType,
    offset: (i32, i32, i32),
    is_water: bool,
    water_top_tex: f32,
    base_tex: f32,
) -> f32 {
    if block == BlockType::Grass {
        if offset.1 == 1 { 0.0 } else if offset.1 == -1 { 1.0 } else { 7.0 }
    } else if block == BlockType::OakLog {
        if offset.1 != 0 { 15.0 } else { 14.0 }
    } else if block == BlockType::CraftingTable {
        if offset.1 == 1 { 29.0 }
        else if offset.1 == -1 { 16.0 }
        else { 30.0 }
    } else if block == BlockType::Furnace {
        if offset.1 == 1 { 33.0 }
        else if offset.1 == -1 { 3.0 }
        else if offset.2 == 1 { 31.0 }
        else { 32.0 }
    } else if is_water {
        if offset.1 == 1 { water_top_tex } else { 5.0 }
    } else {
        base_tex
    }
}
