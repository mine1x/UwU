use super::block::BlockType;

pub fn get_fluid_own_height(block: BlockType, has_fluid_above: bool) -> f32 {
    if has_fluid_above { return 1.0; }
    match block {
        BlockType::WaterSource => 8.0 / 9.0, // Minecraft MAX_FLUID_HEIGHT = 0.8888889
        BlockType::FlowingWater { level } => (level.clamp(1, 7) as f32) / 9.0,
        _ => 0.0,
    }
}

// 4 corners matching Minecraft FluidRenderer:
// 0: NorthWest (X=0, Z=0) -> average of North(0,-1), West(-1,0), NorthWest(-1,-1)
// 1: NorthEast (X=1, Z=0) -> average of North(0,-1), East(1,0), NorthEast(1,-1)
// 2: SouthEast (X=1, Z=1) -> average of South(0,1), East(1,0), SouthEast(1,1)
// 3: SouthWest (X=0, Z=1) -> average of South(0,1), West(-1,0), SouthWest(-1,1)
pub fn compute_water_corner_heights<F>(gx: i32, gy: i32, gz: i32, mut get_block: F) -> [f32; 4]
where
    F: FnMut(i32, i32, i32) -> BlockType,
{
    let above_center = get_block(gx, gy + 1, gz);
    if above_center.is_fluid() {
        return [1.0; 4];
    }

    let h_self = get_fluid_own_height(get_block(gx, gy, gz), false);
    if h_self >= 1.0 {
        return [1.0; 4];
    }

    let calc_corner = |h_self: f32, h1: f32, h2: f32, h_corner: f32| -> f32 {
        if h1 >= 1.0 || h2 >= 1.0 || h_corner >= 1.0 {
            return 1.0;
        }
        let mut sum = 0.0f32;
        let mut count = 0.0f32;
        let mut add = |h: f32| {
            if h >= 0.8 {
                sum += h * 10.0;
                count += 10.0;
            } else if h >= 0.0 {
                sum += h;
                count += 1.0;
            }
        };
        if h1 > 0.0 || h2 > 0.0 {
            add(h_corner);
        }
        add(h_self);
        add(h1);
        add(h2);
        if count > 0.0 { sum / count } else { h_self }
    };

    let gh = |ox: i32, oz: i32, get_block: &mut F| -> f32 {
        let b = get_block(gx + ox, gy, gz + oz);
        let ab = get_block(gx + ox, gy + 1, gz + oz);
        if ab.is_fluid() {
            1.0
        } else if b.is_fluid() {
            get_fluid_own_height(b, false)
        } else if b.is_solid() {
            -1.0
        } else {
            0.0
        }
    };

    let (hn, hs, he, hw) = (
        gh(0, -1, &mut get_block),
        gh(0, 1, &mut get_block),
        gh(1, 0, &mut get_block),
        gh(-1, 0, &mut get_block),
    );

    let h_nw = calc_corner(h_self, hn, hw, gh(-1, -1, &mut get_block));
    let h_ne = calc_corner(h_self, hn, he, gh(1, -1, &mut get_block));
    let h_se = calc_corner(h_self, hs, he, gh(1, 1, &mut get_block));
    let h_sw = calc_corner(h_self, hs, hw, gh(-1, 1, &mut get_block));

    [h_nw, h_ne, h_se, h_sw]
}