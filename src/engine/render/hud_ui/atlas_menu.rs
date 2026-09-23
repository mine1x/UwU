use super::atlas::HudAtlas;

pub fn blit_menu_textures(p: &mut [u8]) {
    HudAtlas::blit(p, include_bytes!("../../../assets/textures/gui/widget/button.png"), 0, 200);
    HudAtlas::blit(p, include_bytes!("../../../assets/textures/gui/widget/button_highlighted.png"), 0, 225);
    HudAtlas::blit(p, include_bytes!("../../../assets/textures/block/dirt.png"), 210, 200);
    HudAtlas::blit(p, include_bytes!("../../../assets/textures/gui/title/minecraft_256.png"), 0, 850);
}
