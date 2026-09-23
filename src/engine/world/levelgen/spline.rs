// Minecraft Java CubicSpline & TerrainProvider splines
// Reference: net.minecraft.util.CubicSpline / net.minecraft.data.worldgen.TerrainProvider

#[derive(Clone)]
pub enum CubicSpline {
    Constant(f32),
    Multipoint {
        coordinate_idx: usize, // 0 = continents, 1 = erosion, 2 = ridges, 3 = weirdness
        locations: Vec<f32>,
        values: Vec<CubicSpline>,
        derivatives: Vec<f32>,
    },
}

impl CubicSpline {
    pub fn sample(&self, coords: &[f32; 4]) -> f32 {
        match self {
            CubicSpline::Constant(v) => *v,
            CubicSpline::Multipoint { coordinate_idx, locations, values, derivatives } => {
                let input = coords[*coordinate_idx];
                let last = locations.len() - 1;
                let start = match locations.binary_search_by(|probe| probe.partial_cmp(&input).unwrap()) {
                    Ok(idx) => idx as i32,
                    Err(idx) => (idx as i32) - 1,
                };

                if start < 0 {
                    let d = derivatives[0];
                    let v = values[0].sample(coords);
                    if d == 0.0 { v } else { v + d * (input - locations[0]) }
                } else if start >= (last as i32) {
                    let d = derivatives[last];
                    let v = values[last].sample(coords);
                    if d == 0.0 { v } else { v + d * (input - locations[last]) }
                } else {
                    let idx = start as usize;
                    let x1 = locations[idx];
                    let x2 = locations[idx + 1];
                    let t = (input - x1) / (x2 - x1);
                    let d1 = derivatives[idx];
                    let d2 = derivatives[idx + 1];
                    let y1 = values[idx].sample(coords);
                    let y2 = values[idx + 1].sample(coords);
                    let a = d1 * (x2 - x1) - (y2 - y1);
                    let b = -d2 * (x2 - x1) + (y2 - y1);
                    let lerp_val = y1 + t * (y2 - y1);
                    let curve = a + t * (b - a);
                    lerp_val + t * (1.0 - t) * curve
                }
            }
        }
    }
}

pub struct SplineBuilder {
    coord_idx: usize,
    locations: Vec<f32>,
    values: Vec<CubicSpline>,
    derivatives: Vec<f32>,
}

impl SplineBuilder {
    pub fn new(coord_idx: usize) -> Self {
        Self {
            coord_idx,
            locations: Vec::new(),
            values: Vec::new(),
            derivatives: Vec::new(),
        }
    }

    pub fn add_point(mut self, location: f32, value: f32) -> Self {
        self.locations.push(location);
        self.values.push(CubicSpline::Constant(value));
        self.derivatives.push(0.0);
        self
    }

    pub fn add_point_with_derivative(mut self, location: f32, value: f32, derivative: f32) -> Self {
        self.locations.push(location);
        self.values.push(CubicSpline::Constant(value));
        self.derivatives.push(derivative);
        self
    }

    pub fn add_spline(mut self, location: f32, spline: CubicSpline) -> Self {
        self.locations.push(location);
        self.values.push(spline);
        self.derivatives.push(0.0);
        self
    }

    pub fn build(self) -> CubicSpline {
        CubicSpline::Multipoint {
            coordinate_idx: self.coord_idx,
            locations: self.locations,
            values: self.values,
            derivatives: self.derivatives,
        }
    }
}

pub fn peaks_and_valleys(weirdness: f32) -> f32 {
    -((weirdness.abs() - 0.6666667).abs() - 0.33333334) * 3.0
}

// Build standard Minecraft Overworld Offset Spline
// 0: continents, 1: erosion, 2: ridges, 3: weirdness
pub fn build_overworld_offset_spline() -> CubicSpline {
    let beach = build_erosion_offset(-0.15, 0.0, 0.0, 0.1, 0.0, -0.03);
    let low = build_erosion_offset(-0.1, 0.03, 0.1, 0.1, 0.01, -0.03);
    let mid = build_erosion_offset(-0.1, 0.03, 0.1, 0.7, 0.01, -0.03);
    let high = build_erosion_offset(-0.05, 0.03, 0.1, 1.0, 0.01, 0.01);

    SplineBuilder::new(0)
        .add_point(-1.1, 0.044)
        .add_point(-1.02, -0.2222)
        .add_point(-0.51, -0.2222)
        .add_point(-0.44, -0.12)
        .add_point(-0.18, -0.12)
        .add_spline(-0.16, beach.clone())
        .add_spline(-0.15, beach)
        .add_spline(-0.1, low)
        .add_spline(0.25, mid)
        .add_spline(1.0, high)
        .build()
}

fn build_erosion_offset(
    low_valley: f32,
    hill: f32,
    tall_hill: f32,
    mountain_factor: f32,
    plain: f32,
    swamp: f32,
) -> CubicSpline {
    let plains = ridge_spline(low_valley, plain, plain, hill, tall_hill);
    let mountains = ridge_spline(low_valley - 0.15, plain * mountain_factor, hill * mountain_factor, 0.5 * mountain_factor, 0.6 * mountain_factor);
    let swamps = ridge_spline(-0.02, swamp, swamp, hill, tall_hill);

    SplineBuilder::new(1)
        .add_spline(-0.85, mountains.clone())
        .add_spline(-0.4, mountains)
        .add_spline(0.2, plains.clone())
        .add_spline(0.4, plains)
        .add_spline(0.7, swamps)
        .build()
}

fn ridge_spline(valley: f32, low: f32, mid: f32, high: f32, peaks: f32) -> CubicSpline {
    let d1 = 0.5 * (low - valley).max(0.5);
    let d2 = 5.0 * (mid - low);
    SplineBuilder::new(2)
        .add_point_with_derivative(-1.0, valley, d1)
        .add_point_with_derivative(-0.4, low, d1.min(d2))
        .add_point_with_derivative(0.0, mid, d2)
        .add_point_with_derivative(0.4, high, 2.0 * (high - mid))
        .add_point_with_derivative(1.0, peaks, 0.7 * (peaks - high))
        .build()
}

// Build standard Minecraft Overworld Factor Spline
pub fn build_overworld_factor_spline() -> CubicSpline {
    let base_spline = SplineBuilder::new(3).add_point(-0.2, 6.3).add_point(0.2, 5.08).build();
    let erosion_points = SplineBuilder::new(1)
        .add_spline(-0.6, base_spline.clone())
        .add_spline(-0.25, base_spline.clone())
        .add_spline(0.03, base_spline)
        .add_point(0.35, 5.08)
        .add_point(0.62, 5.08)
        .build();

    SplineBuilder::new(0)
        .add_point(-0.19, 3.95)
        .add_spline(-0.15, erosion_points.clone())
        .add_spline(0.03, erosion_points)
        .build()
}
