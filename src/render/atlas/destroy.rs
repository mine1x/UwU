use super::loader::load_atlas_layer;

pub fn append_destroy_stages(pixels: &mut Vec<u8>) {
    // 18..=27: Destroy stages 0 to 9
    for stage_bytes in [
        include_bytes!("../../../assets/textures/block/destroy_stage_0.png").as_slice(),
        include_bytes!("../../../assets/textures/block/destroy_stage_1.png").as_slice(),
        include_bytes!("../../../assets/textures/block/destroy_stage_2.png").as_slice(),
        include_bytes!("../../../assets/textures/block/destroy_stage_3.png").as_slice(),
        include_bytes!("../../../assets/textures/block/destroy_stage_4.png").as_slice(),
        include_bytes!("../../../assets/textures/block/destroy_stage_5.png").as_slice(),
        include_bytes!("../../../assets/textures/block/destroy_stage_6.png").as_slice(),
        include_bytes!("../../../assets/textures/block/destroy_stage_7.png").as_slice(),
        include_bytes!("../../../assets/textures/block/destroy_stage_8.png").as_slice(),
        include_bytes!("../../../assets/textures/block/destroy_stage_9.png").as_slice(),
    ] {
        pixels.extend(load_atlas_layer(stage_bytes, None, [0, 0, 0, 0]));
    }
}
