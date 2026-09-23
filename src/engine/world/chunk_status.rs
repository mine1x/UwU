#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum ChunkStatus {
    Empty = 0,
    StructureStarts = 1,
    StructureReferences = 2,
    Biomes = 3,
    Noise = 4,
    Surface = 5,
    Carvers = 6,
    Features = 7,
    InitializeLight = 8,
    Light = 9,
    Spawn = 10,
    Full = 11,
}

impl ChunkStatus {
    pub const fn is_at_least(&self, required: ChunkStatus) -> bool {
        (*self as usize) >= (required as usize)
    }

    pub const fn name(&self) -> &'static str {
        match self {
            ChunkStatus::Empty => "empty",
            ChunkStatus::StructureStarts => "structure_starts",
            ChunkStatus::StructureReferences => "structure_references",
            ChunkStatus::Biomes => "biomes",
            ChunkStatus::Noise => "noise",
            ChunkStatus::Surface => "surface",
            ChunkStatus::Carvers => "carvers",
            ChunkStatus::Features => "features",
            ChunkStatus::InitializeLight => "initialize_light",
            ChunkStatus::Light => "light",
            ChunkStatus::Spawn => "spawn",
            ChunkStatus::Full => "full",
        }
    }
}