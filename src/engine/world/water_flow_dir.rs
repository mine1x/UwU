use super::block::BlockType;
use super::water_slope::get_fluid_own_height;

pub fn compute_water_flow_vector<F>(gx: i32, gy: i32, gz: i32, mut get_block: F) -> (f32, f32)
where
    F: FnMut(i32, i32, i32) -> BlockType,
{
    let own_b = get_block(gx, gy, gz);
    let above_b = get_block(gx, gy + 1, gz);
    let own_h = get_fluid_own_height(own_b, above_b.is_fluid());

    let mut flow_x = 0.0f32;
    let mut flow_z = 0.0f32;
    let dirs: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    for (dx, dz) in dirs {
        let nb = get_block(gx + dx, gy, gz + dz);
        let nab = get_block(gx + dx, gy + 1, gz + dz);
        let mut nh = get_fluid_own_height(nb, nab.is_fluid());
        let mut dist = 0.0f32;

        if nh == 0.0 {
            if !nb.is_solid() {
                let nbb = get_block(gx + dx, gy - 1, gz + dz);
                let below_h = get_fluid_own_height(nbb, false);
                if below_h > 0.0 {
                    nh = below_h;
                    dist = own_h - (nh - 0.8888889);
                }
            }
        } else if nh > 0.0 {
            dist = own_h - nh;
        }

        if dist != 0.0 {
            flow_x += dx as f32 * dist;
            flow_z += dz as f32 * dist;
        }
    }

    (flow_x, flow_z)
}

pub fn compute_water_top_uvs(flow_x: f32, flow_z: f32) -> ([[f32; 2]; 4], f32) {
    if flow_x.abs() < 0.001 && flow_z.abs() < 0.001 {
        let uvs = [[0.0, 1.0], [1.0, 1.0], [1.0, 0.0], [0.0, 0.0]];
        (uvs, 4.0)
    } else {
        let angle = flow_z.atan2(flow_x) - std::f32::consts::FRAC_PI_2;
        let s = angle.sin() * 0.25;
        let c = angle.cos() * 0.25;

        let u00 = [0.5 + (-c - s), 0.5 + (-c + s)]; // NorthWest
        let u01 = [0.5 + (-c + s), 0.5 + (c + s)];  // SouthWest
        let u10 = [0.5 + (c + s), 0.5 + (c - s)];   // SouthEast
        let u11 = [0.5 + (c - s), 0.5 + (-c - s)];  // NorthEast

        // Top face corners in chunk_mesh: [SouthWest, SouthEast, NorthEast, NorthWest]
        ( [u01, u10, u11, u00], 5.0 )
    }
}
