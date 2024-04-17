use super::{Gamescope, Launch, ScreenResolution, VulkanDriver};

const LAUNCH: Launch = Launch {
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

pub mod launch {
    use super::*;

    pub fn gamemode() -> bool {
        LAUNCH.gamemode
    }

    pub fn mangohud() -> bool {
        LAUNCH.mangohud
    }

    pub fn vulkan_driver() -> VulkanDriver {
        LAUNCH.vulkan_driver
    }

    pub fn gamescope() -> Gamescope {
        LAUNCH.gamescope
    }

    pub fn environment_variables() -> Vec<(String, String)> {
        LAUNCH.environment_variables
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
