pub struct HudZone;

impl HudZone {
    pub fn is_point_in_hud(mx: f32, my: f32, sw: f32, sh: f32, inventory_open: bool) -> bool {
        let aspect = sw / sh;
        let ndc_x = (mx / sw) * 2.0 - 1.0;
        let ndc_y = 1.0 - (my / sh) * 2.0;

        let total_w = 0.728 / aspect;
        let total_h = 0.088;
        let hb_x = -total_w * 0.5;
        let hb_y = -0.96;

        if ndc_x >= hb_x && ndc_x <= hb_x + total_w && ndc_y >= hb_y && ndc_y <= hb_y + total_h {
            return true;
        }

        if inventory_open {
            let s = 0.0065f32;
            let win_w = 195.0 * s / aspect;
            let win_h = 166.0 * s;
            let win_x = -win_w * 0.5;
            let win_y = -win_h * 0.5;

            if ndc_x >= win_x && ndc_x <= win_x + win_w && ndc_y >= win_y && ndc_y <= win_y + win_h {
                return true;
            }
        }
        false
    }

    pub fn get_clicked_inventory_slot(mx: f32, my: f32, sw: f32, sh: f32) -> Option<usize> {
        let aspect = sw / sh;
        let ndc_x = (mx / sw) * 2.0 - 1.0;
        let ndc_y = 1.0 - (my / sh) * 2.0;

        let s = 0.0065f32;
        let win_w = 176.0 * s / aspect;
        let win_x = -win_w * 0.5;
        let win_y = -166.0 * s * 0.5;

        for slot in 0..46 {
            if let Some((gx, gy)) = crate::core::ui::slot_coords::get_slot_pos(slot) {
                let sx = win_x + gx * s / aspect;
                let sy = win_y + (166.0 - gy - 18.0) * s;
                if ndc_x >= sx && ndc_x <= sx + 18.0 * s / aspect && ndc_y >= sy && ndc_y <= sy + 18.0 * s {
                    return Some(slot);
                }
            }
        }
        None
    }

    pub fn get_clicked_hotbar_slot(mx: f32, my: f32, sw: f32, sh: f32) -> Option<usize> {
        let aspect = sw / sh;
        let ndc_x = (mx / sw) * 2.0 - 1.0;
        let ndc_y = 1.0 - (my / sh) * 2.0;

        let total_w = 0.728 / aspect;
        let total_h = 0.088;
        let hb_x = -total_w * 0.5;
        let hb_y = -0.96;
        let slot_w = total_w * (20.0 / 182.0);

        if ndc_y >= hb_y && ndc_y <= hb_y + total_h {
            for idx in 0..9 {
                let sx = hb_x + (idx as f32) * slot_w;
                if ndc_x >= sx && ndc_x <= sx + slot_w {
                    return Some(idx);
                }
            }
        }
        None
    }
}
