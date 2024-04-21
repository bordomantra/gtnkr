mod config_file;
mod default_values;
mod gamescope;
mod launch;
mod parsing;
mod screen_resolution;
mod vulkan_driver;

use gamescope::Gamescope;
use launch::Launch;
use screen_resolution::ScreenResolution;
use std::path::PathBuf;
use tokio::io;
use vulkan_driver::VulkanDriver;

#[derive(Debug, thiserror::Error)]
enum ConfigError {
    #[error(
        "The root user can't have a configuration directory, run the application as a normal user."
    )]
    UserIsRoot,

    #[error("User lacks the necessary permissions to access a file/directory in the path {0}")]
    PermissionDenied(PathBuf),

    #[error("A file/directory in the path {0} couldn't be found")]
    NotFound(PathBuf),

    #[error("The configuration file at {0} is not encoded in valid UTF-8, see: {1:#?}")]
    InvalidFileEncoding(PathBuf, io::Error),

    #[error("Failed to parse the config file at {0}, position {2}:{3}, {1}.")]
    ParseError(PathBuf, String, u16, u16),
}
