#![allow(unused)]

use crate::game_config::{GameConfig, GameConfigError, GameConfigFile};
use std::env;
use std::path::{Path, PathBuf};
use tokio::process::Command;

#[derive(Debug, thiserror::Error)]
pub enum GameLauncherError {
    #[error("The path `{0}`, is not a valid executable path.")]
    InvalidExecutablePath(PathBuf),

    #[error("Failed to resolve the executable's path based on $PATH, see: ")]
    FailedToResolveExecutablePath(which::Error),

    #[error(transparent)]
    FailedToFindConfigFile(GameConfigError),

    #[error(transparent)]
    FailedToParseConfigFile(GameConfigError),
}

pub struct GameLauncher {
    config: GameConfig,
}

impl GameLauncher {
    pub async fn launch_by_command(
        command: &str,
        config_file_name: &str,
    ) -> Result<(), GameLauncherError> {
        let config_file = GameConfigFile::from_filename(config_file_name)
            .await
            .map_err(GameLauncherError::FailedToFindConfigFile)?;

        let config = {
            if let Some(config_file) = config_file {
                GameConfig::from_game_config_file(config_file)
                    .await
                    .map_err(GameLauncherError::FailedToParseConfigFile)?
            } else {
                tracing::warn!("Game config file with the name `{config_file_name}` doesn't exist, using the defaults.");

                GameConfig::default()
            }
        };

        let gamescope_command = config.gamescope.as_command();

        let mut launch_command: Vec<&str> = Vec::new();

        if config.gamemode {
            launch_command.push("/bin/gamemoderun")
        }

        if config.mangohud {
            launch_command.push("/bin/mangohud")
        }

        let gamescope_command = config.gamescope.as_command().await;

        launch_command.push(&gamescope_command);
        launch_command.push(config.vulkan_driver.as_command().await);

        config
            .environment_variables
            .iter()
            .for_each(|(key, value)| env::set_var(key, value));

        let launch_command_string = format!("{} {}", launch_command.join(" "), command);

        let process = Command::new("/bin/sh")
            .arg("-c")
            .arg(launch_command_string)
            .output()
            .await
            .expect("Failed to run launch command");

        Ok(())
    }

    pub async fn launch_by_executable(executable: &str) -> Result<(), GameLauncherError> {
        let executable_path =
            which::which(executable).map_err(GameLauncherError::FailedToResolveExecutablePath)?;

        if let Some(executable_file_name) = executable_path
            .file_name()
            .and_then(|file_name| file_name.to_str())
        {
            if let Some(executable_path_as_string) = executable_path.to_str() {
                Self::launch_by_command(
                    executable_path_as_string,
                    &format!("{executable_file_name}.ron"),
                )
                .await?;

                return Ok(());
            }
        }

        Err(GameLauncherError::InvalidExecutablePath(
            executable_path.to_path_buf(),
        ))
    }
}
