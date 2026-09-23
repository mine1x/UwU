#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CreativeSlotAction {
    Palette(usize),
    Hotbar(usize),
    Destroy,
}

pub fn get_clicked_creative_slot(mx: f32, my: f32, sw: f32, sh: f32) -> Option<CreativeSlotAction> {
    let aspect = sw / sh;
    let ndc_x = (mx / sw) * 2.0 - 1.0;
    let ndc_y = 1.0 - (my / sh) * 2.0;

    let s = 0.0065f32;
    let win_w = 195.0 * s / aspect;
    let win_x = -win_w * 0.5;
    let win_y = -136.0 * s * 0.5;

    // 1. 9x5 Creative item palette
    for row in 0..5 {
        for col in 0..9 {
            let gx = 9.0 + (col as f32) * 18.0;
            let gy = 18.0 + (row as f32) * 18.0;
            let sx = win_x + gx * s / aspect;
            let sy = win_y + (136.0 - gy - 18.0) * s;
            if ndc_x >= sx && ndc_x <= sx + 18.0 * s / aspect && ndc_y >= sy && ndc_y <= sy + 18.0 * s {
                return Some(CreativeSlotAction::Palette(row * 9 + col));
            }
        }
    }

    // 2. Hotbar row (col: 0..9, gy = 112)
    for col in 0..9 {
        let gx = 9.0 + (col as f32) * 18.0;
        let gy = 112.0;
        let sx = win_x + gx * s / aspect;
        let sy = win_y + (136.0 - gy - 18.0) * s;
        if ndc_x >= sx && ndc_x <= sx + 18.0 * s / aspect && ndc_y >= sy && ndc_y <= sy + 18.0 * s {
            return Some(CreativeSlotAction::Hotbar(col));
        }
    }

    // 3. Destroy item slot at (173, 112)
    let sx = win_x + 173.0 * s / aspect;
    let sy = win_y + (136.0 - 112.0 - 18.0) * s;
    if ndc_x >= sx && ndc_x <= sx + 18.0 * s / aspect && ndc_y >= sy && ndc_y <= sy + 18.0 * s {
        return Some(CreativeSlotAction::Destroy);
    }

    None
}
