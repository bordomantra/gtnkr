#![allow(unused)]

mod config_file;
mod gamescope;
mod parsing;
mod screen_resolution;
mod vulkan_driver;

pub use config_file::GameConfigFile;
pub use gamescope::Gamescope;
pub use screen_resolution::ScreenResolution;
use serde::Deserialize;
use std::path::PathBuf;
use tokio::io;
pub use vulkan_driver::VulkanDriver;

#[derive(Debug, thiserror::Error)]
pub enum GameConfigError {
    #[error(
        "The root user can't have a game configuration directory. Run the command as a normal user or specify a configuration directory with {}", format!("${}_GAME_CONFIG_DIR", env!("CARGO_PKG_NAME").to_uppercase())
    )]
    UserIsRoot,

    #[error("User lacks the necessary permissions to access a file/directory in the path `{0}`")]
    PermissionDenied(PathBuf),

    #[error("A file/directory in the path `{0}` couldn't be found")]
    NotFound(PathBuf),

    #[error("The configuration file at `{0}` is not encoded in valid UTF-8, see: {1:#?}")]
    InvalidFileEncoding(PathBuf, io::Error),

    #[error("Failed to parse the configuration file at `{0}`, position {2}:{3}. {1}.")]
    ParseError(PathBuf, String, u16, u16),

    #[error("Unexpected IO error, see: {0:#?}")]
    UnexpectedIoError(io::Error),
}

const fn _default_gamemode() -> bool {
    true
}
const fn _default_mangohud() -> bool {
    true
}

const fn _default_fps_limit() -> u32 {
    0
}

const fn _default_environment_variables() -> Vec<(String, String)> {
    vec![]
}

#[derive(Deserialize)]
pub struct GameConfig {
    #[serde(default = "_default_gamemode")]
    pub gamemode: bool,

    #[serde(default = "_default_mangohud")]
    pub mangohud: bool,

    #[serde(default)]
    pub vulkan_driver: VulkanDriver,

    #[serde(default = "_default_fps_limit")]
    pub fps_limit: u32,

    #[serde(default)]
    pub gamescope: Option<Gamescope>,

    #[serde(default = "_default_environment_variables")]
    pub environment_variables: Vec<(String, String)>,
}

impl Default for GameConfig {
    fn default() -> GameConfig {
        Self {
            gamemode: _default_gamemode(),
            mangohud: _default_mangohud(),
            vulkan_driver: VulkanDriver::default(),
            fps_limit: _default_fps_limit(),
            gamescope: Some(Gamescope::default()),
            environment_variables: _default_environment_variables(),
        }
    }
}
