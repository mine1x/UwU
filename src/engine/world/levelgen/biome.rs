// Minecraft Java Biome classification based on Multi-Noise climate parameters
// (Temperature, Vegetation, Continentalness, Erosion, Weirdness)

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BiomeType {
    Plains,
    Forest,
    BirchForest,
    Taiga,
    Desert,
    Swamp,
    Jungle,
    Savanna,
    SnowyPlains,
    Beach,
    Ocean,
    DeepOcean,
    Mountains,
    StonyPeaks,
}

impl BiomeType {
    pub fn classify(temp: f32, veg: f32, cont: f32, erosion: f32, weird: f32) -> Self {
        let _ = weird;
        if cont < -0.45 {
            return BiomeType::DeepOcean;
        }
        if cont < -0.15 {
            return BiomeType::Ocean;
        }
        if cont < -0.1 {
            return BiomeType::Beach;
        }

        if erosion < -0.4 {
            if temp < -0.2 {
                return BiomeType::Mountains;
            } else {
                return BiomeType::StonyPeaks;
            }
        }

        if temp < -0.4 {
            BiomeType::SnowyPlains
        } else if temp < -0.1 {
            BiomeType::Taiga
        } else if temp > 0.55 {
            if veg < -0.2 {
                BiomeType::Desert
            } else {
                BiomeType::Savanna
            }
        } else if veg > 0.4 {
            if temp > 0.3 {
                BiomeType::Jungle
            } else if temp > 0.0 {
                BiomeType::Forest
            } else {
                BiomeType::BirchForest
            }
        } else if veg < -0.3 && temp > 0.1 && cont > 0.0 {
            BiomeType::Swamp
        } else {
            BiomeType::Plains
        }
    }
}
