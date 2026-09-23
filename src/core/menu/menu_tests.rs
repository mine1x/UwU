#[cfg(test)]
mod tests {
    use crate::core::session::GameState;
    use crate::core::menu::{handle_menu_click, MenuAction};
    use crate::network::DiscoveredServer;

    #[test]
    fn test_title_screen_clicks() {
        let aspect = 16.0 / 9.0;
        let servers: Vec<DiscoveredServer> = Vec::new();

        // Click Singleplayer button (id=1, y in 0.05..0.13, x near 0.0)
        let click_sp = handle_menu_click(GameState::TitleScreen, (0.0, 0.08), aspect, &servers, "");
        assert_eq!(click_sp, Some(MenuAction::StartSingleplayer));

        // Click Multiplayer button (id=2, y in -0.07..0.01)
        let click_mp = handle_menu_click(GameState::TitleScreen, (0.0, -0.04), aspect, &servers, "");
        assert_eq!(click_mp, Some(MenuAction::OpenLanLobby));

        // Click Quit button (id=3, y in -0.19..-0.11)
        let click_quit = handle_menu_click(GameState::TitleScreen, (0.0, -0.15), aspect, &servers, "");
        assert_eq!(click_quit, Some(MenuAction::QuitGame));

        // Click outside buttons (e.g. top corner)
        let click_miss = handle_menu_click(GameState::TitleScreen, (0.9, 0.9), aspect, &servers, "");
        assert_eq!(click_miss, None);
    }

    #[test]
    fn test_lan_lobby_and_pause_clicks() {
        let aspect = 16.0 / 9.0;
        let mut servers = Vec::new();
        servers.push(DiscoveredServer::new("Test World".into(), "192.168.1.5:25565".into()));

        // Lan lobby Host LAN World button (y = -0.48..-0.40)
        let click_host = handle_menu_click(GameState::LanLobby, (0.0, -0.44), aspect, &servers, "");
        assert_eq!(click_host, Some(MenuAction::HostLan));

        // Lan lobby Direct Connect button (y = -0.60..-0.52) -> Opens DirectConnect screen
        let click_dc = handle_menu_click(GameState::LanLobby, (0.0, -0.56), aspect, &servers, "");
        assert_eq!(click_dc, Some(MenuAction::OpenDirectConnect));

        // Lan lobby Discovered server 0 click (y = 0.50..0.575)
        let click_srv = handle_menu_click(GameState::LanLobby, (0.0, 0.53), aspect, &servers, "");
        assert_eq!(click_srv, Some(MenuAction::ConnectLan("192.168.1.5:25565".into())));

        // Direct Connect screen: Join Server button (y = -0.20..-0.12)
        let click_join = handle_menu_click(GameState::DirectConnect, (0.0, -0.16), aspect, &[], "10.0.0.1:25565");
        assert_eq!(click_join, Some(MenuAction::ConnectLan("10.0.0.1:25565".into())));

        // Direct Connect screen: Cancel button (y = -0.32..-0.24)
        let click_cancel = handle_menu_click(GameState::DirectConnect, (0.0, -0.28), aspect, &[], "");
        assert_eq!(click_cancel, Some(MenuAction::OpenLanLobby));

        // Pause menu Back to Game (y = 0.12..0.20)
        let click_resume = handle_menu_click(GameState::Paused, (0.0, 0.15), aspect, &[], "");
        assert_eq!(click_resume, Some(MenuAction::ResumeGame));
    }
}
