#![allow(unused)]

use crate::game_config::{GameConfig, GameConfigError, GameConfigFile};
use lazy_static::lazy_static;
use regex::Regex;
use std::path::{Path, PathBuf};
use std::{env, process::Stdio};
use tokio::{
    fs, io,
    io::{unix::AsyncFd, AsyncBufReadExt, AsyncRead, BufReader},
    process::Command,
};

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

        let gamescope_command = config.gamescope.as_command();

        let mut launch_command: Vec<&str> = Vec::new();

        if config.gamemode {
            launch_command.push("/bin/gamemoderun")
        }

        if config.mangohud {
            launch_command.push("/bin/mangohud")
        }

        let gamescope_command = config.gamescope.as_command().await;

        let default_display_value =
            env::var("DISPLAY").expect("Failed to get the default $DISPLAY");

        if !config.gamescope.steam_overlay_fix {
            launch_command.push(&gamescope_command);
        } else {
            let gamescope_process = Command::new("/bin/sh")
                .arg("-c")
                .arg(gamescope_command)
                .stderr(Stdio::piped())
                .spawn()
                .map_err(GameLauncherError::RunCommand)?;

            let stderr = gamescope_process
                .stderr
                .expect("Failed to get the STDERR of the gamescope command");

            let mut stderr_lines = BufReader::new(stderr).lines();

            lazy_static! {
                static ref REGEX: Regex = Regex::new(r#"Starting Xwayland on :(\d+)"#)
                    .expect("Failed to compile the regex");
            }

            let (tx, mut rx) = tokio::sync::oneshot::channel::<u16>();

            let task_handle = tokio::spawn(async move {
                while let Some(line) = stderr_lines.next_line().await.expect("Failed next line") {
                    if let Some(Ok(display_number)) = REGEX.captures(&line).and_then(|captures| {
                        captures
                            .get(1)
                            .map(|r#match| r#match.as_str().parse::<u16>())
                    }) {
                        tx.send(display_number)
                            .expect("Failed to send the gamescope display number through the oneshot channel");

                        break;
                    }
                }
            });

            let display_number = rx.await.expect(
                "Failed to receive the gamescope display number through the oneshot channel",
            );

            env::set_var("DISPLAY", format!(":{display_number}"));
        }

        launch_command.push(config.vulkan_driver.as_command().await);

        config
            .environment_variables
            .iter()
            .for_each(|(key, value)| env::set_var(key, value));

        let launch_command_string = format!("{} {}", launch_command.join(" "), command);

        Command::new("/bin/sh")
            .arg("-c")
            .arg(launch_command_string)
            .output()
            .await
            .map_err(GameLauncherError::RunCommand)?;

        env::set_var("DISPLAY", default_display_value);

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
