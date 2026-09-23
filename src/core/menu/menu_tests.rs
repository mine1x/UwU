#[cfg(test)]
mod tests {
    use crate::core::session::GameState;
    use crate::core::menu::{handle_menu_click, MenuAction};
    use crate::network::DiscoveredServer;

    #[test]
    fn test_title_screen_clicks() {
        let aspect = 16.0 / 9.0;
        let servers: Vec<DiscoveredServer> = Vec::new();

        // Click Singleplayer button (id=1, y in 0.05..0.13, x near 0.0) -> Opens CreateWorld
        let click_sp = handle_menu_click(GameState::TitleScreen, (0.0, 0.08), aspect, &servers, "", "Survival");
        assert_eq!(click_sp, Some(MenuAction::OpenCreateWorld));

        // Click Multiplayer button (id=2, y in -0.07..0.01)
        let click_mp = handle_menu_click(GameState::TitleScreen, (0.0, -0.04), aspect, &servers, "", "Survival");
        assert_eq!(click_mp, Some(MenuAction::OpenLanLobby));

        // Click Quit button (id=3, y in -0.19..-0.11)
        let click_quit = handle_menu_click(GameState::TitleScreen, (0.0, -0.15), aspect, &servers, "", "Survival");
        assert_eq!(click_quit, Some(MenuAction::QuitGame));

        // Click outside buttons (e.g. top corner)
        let click_miss = handle_menu_click(GameState::TitleScreen, (0.9, 0.9), aspect, &servers, "", "Survival");
        assert_eq!(click_miss, None);
    }

    #[test]
    fn test_create_world_clicks() {
        let aspect = 16.0 / 9.0;

        // Click Game Mode toggle button (y = -0.10..-0.02)
        let click_gm = handle_menu_click(GameState::CreateWorld, (0.0, -0.06), aspect, &[], "", "Survival");
        assert_eq!(click_gm, Some(MenuAction::ToggleCreateGameMode));

        // Click Create New World button (y = -0.26..-0.18)
        let click_create = handle_menu_click(GameState::CreateWorld, (0.0, -0.22), aspect, &[], "", "Survival");
        assert_eq!(click_create, Some(MenuAction::CreateNewWorld));

        // Click Cancel button (y = -0.38..-0.30)
        let click_cancel = handle_menu_click(GameState::CreateWorld, (0.0, -0.34), aspect, &[], "", "Survival");
        assert_eq!(click_cancel, Some(MenuAction::QuitToTitle));

        // Click World Name input box (y = 0.54..0.61)
        let click_name_box = handle_menu_click(GameState::CreateWorld, (0.0, 0.56), aspect, &[], "", "Survival");
        assert_eq!(click_name_box, Some(MenuAction::SelectInputField(0)));

        // Click Seed input box (y = 0.28..0.35)
        let click_seed_box = handle_menu_click(GameState::CreateWorld, (0.0, 0.30), aspect, &[], "", "Survival");
        assert_eq!(click_seed_box, Some(MenuAction::SelectInputField(1)));
    }

    #[test]
    fn test_lan_lobby_and_pause_clicks() {
        let aspect = 16.0 / 9.0;
        let mut servers = Vec::new();
        servers.push(DiscoveredServer::new("Test World".into(), "192.168.1.5:25565".into()));

        // Lan lobby Host LAN World button (y = -0.48..-0.40)
        let click_host = handle_menu_click(GameState::LanLobby, (0.0, -0.44), aspect, &servers, "", "Survival");
        assert_eq!(click_host, Some(MenuAction::HostLan));

        // Lan lobby Direct Connect button (y = -0.60..-0.52) -> Opens DirectConnect screen
        let click_dc = handle_menu_click(GameState::LanLobby, (0.0, -0.56), aspect, &servers, "", "Survival");
        assert_eq!(click_dc, Some(MenuAction::OpenDirectConnect));

        // Lan lobby Discovered server 0 click (y = 0.50..0.575)
        let click_srv = handle_menu_click(GameState::LanLobby, (0.0, 0.53), aspect, &servers, "", "Survival");
        assert_eq!(click_srv, Some(MenuAction::ConnectLan("192.168.1.5:25565".into())));

        // Direct Connect screen: Join Server button (y = -0.20..-0.12)
        let click_join = handle_menu_click(GameState::DirectConnect, (0.0, -0.16), aspect, &[], "10.0.0.1:25565", "Survival");
        assert_eq!(click_join, Some(MenuAction::ConnectLan("10.0.0.1:25565".into())));

        // Direct Connect screen: Cancel button (y = -0.32..-0.24)
        let click_cancel = handle_menu_click(GameState::DirectConnect, (0.0, -0.28), aspect, &[], "", "Survival");
        assert_eq!(click_cancel, Some(MenuAction::OpenLanLobby));

        // Pause menu Back to Game (y = 0.12..0.20)
        let click_resume = handle_menu_click(GameState::Paused, (0.0, 0.15), aspect, &[], "", "Survival");
        assert_eq!(click_resume, Some(MenuAction::ResumeGame));
    }
}
