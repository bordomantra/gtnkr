#![allow(unused)]

mod config_file;
mod default_values;
mod gamescope;
mod parsing;
mod screen_resolution;
mod vulkan_driver;

pub use config_file::GameConfigFile;
pub use gamescope::Gamescope;
pub use screen_resolution::ScreenResolution;
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

use default_values::game as game_config_default_values;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct GameConfig {
    #[serde(default = "game_config_default_values::gamemode")]
    pub gamemode: bool,

    #[serde(default = "game_config_default_values::mangohud")]
    pub mangohud: bool,

    #[serde(default = "game_config_default_values::vulkan_driver")]
    pub vulkan_driver: VulkanDriver,

    #[serde(default = "game_config_default_values::gamescope")]
    pub gamescope: Gamescope,

    #[serde(default = "game_config_default_values::environment_variables")]
    pub environment_variables: Vec<(String, String)>,
}

impl Default for GameConfig {
    fn default() -> Self {
        default_values::GAME
    }
}
