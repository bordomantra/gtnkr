use super::{GameConfig, Gamescope, ScreenResolution, VulkanDriver};

pub const GAME: GameConfig = GameConfig {
    gamemode: true,
    mangohud: true,
    vulkan_driver: VulkanDriver::Amdvlk,
    gamescope: GAMESCOPE,
    environment_variables: vec![],
};

const GAMESCOPE: Gamescope = Gamescope {
    source_resolution: ScreenResolution::Native,
    start_as_fullscreen: true,
    force_grab_cursor: true,
    tearing: true,
};

pub mod game {
    use super::*;

    pub fn gamemode() -> bool {
        GAME.gamemode
    }

    pub fn mangohud() -> bool {
        GAME.mangohud
    }

    pub fn vulkan_driver() -> VulkanDriver {
        GAME.vulkan_driver
    }

    pub fn gamescope() -> Gamescope {
        GAME.gamescope
    }

    pub fn environment_variables() -> Vec<(String, String)> {
        GAME.environment_variables
    }
}

pub mod gamescope {
    use super::*;

    pub fn source_resolution() -> ScreenResolution {
        GAMESCOPE.source_resolution
    }

    pub fn start_as_fullscreen() -> bool {
        GAMESCOPE.start_as_fullscreen
    }

    pub fn force_grab_cursor() -> bool {
        GAMESCOPE.force_grab_cursor
    }

    pub fn tearing() -> bool {
        GAMESCOPE.tearing
    }
}
