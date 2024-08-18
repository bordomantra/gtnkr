use crate::config::{GameConfig, GameConfigError, GameConfigFile};
use std::{env, path::PathBuf};
use tokio::{io, process::Command};

#[derive(Debug, thiserror::Error)]
pub enum GameLauncherError {
    #[error("The path `{0}`, is not a valid executable path.")]
    InvalidExecutablePath(PathBuf),

    #[error("Failed to resolve the executable's path based on $PATH, see: ")]
    ResolveExecutablePath(which::Error),

    #[error(transparent)]
    FindConfigFile(GameConfigError),

    #[error(transparent)]
    ParseConfigFile(GameConfigError),

    #[error("Failed to run the launch or gamescope command, see: {0:#?}")]
    RunCommand(io::Error),
}

pub struct GameLauncher {}

impl GameLauncher {
    pub async fn launch_by_command(
        command: &str,
        config_file_name: &str,
    ) -> Result<(), GameLauncherError> {
        let config_file = GameConfigFile::from_filename(config_file_name)
            .await
            .map_err(GameLauncherError::FindConfigFile)?;

        let config = {
            if let Some(config_file) = config_file {
                GameConfig::from_game_config_file(config_file)
                    .await
                    .map_err(GameLauncherError::ParseConfigFile)?
            } else {
                tracing::warn!("Game config file with the name `{config_file_name}` doesn't exist, using the defaults.");

                GameConfig::default()
            }
        };

        let mut launch_command: Vec<&str> = Vec::new();

        if config.gamemode {
            launch_command.push("/bin/gamemoderun")
        }

        if config.mangohud {
            launch_command.push("/bin/mangohud")
        }

        let gamescope_command = config.gamescope.as_command().await;

        launch_command.push(&gamescope_command);

        if let Some(vulkan_driver) = config.vulkan_driver.as_command() {
            launch_command.push(vulkan_driver);
        }


        config
            .environment_variables
            .iter()
            .for_each(|(key, value)| env::set_var(key, value));

        let launch_command_string = format!("{} {}", launch_command.join(" "), command);

        tracing::info!("Launching the game with [{launch_command_string}]");

        Command::new("/bin/sh")
            .arg("-c")
            .arg(launch_command_string)
            .output()
            .await
            .map_err(GameLauncherError::RunCommand)?;

        Ok(())
    }

    pub async fn launch_by_executable(executable: &str) -> Result<(), GameLauncherError> {
        let executable_path =
            which::which(executable).map_err(GameLauncherError::ResolveExecutablePath)?;

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
