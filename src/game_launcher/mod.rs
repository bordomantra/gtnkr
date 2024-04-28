#![allow(unused)]

use crate::game_config::{GameConfig, GameConfigError, GameConfigFile, Gamescope};
use lazy_static::lazy_static;
use nix::{
    errno::Errno,
    sys::{signal, signal::Signal, wait, wait::WaitPidFlag},
    unistd::Pid,
};
use regex::Regex;
use std::path::{Path, PathBuf};
use std::{env, process::Stdio};
use tokio::{
    fs, io,
    io::{unix::AsyncFd, AsyncBufReadExt, AsyncRead, BufReader},
    process::{ChildStderr, Command},
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

        let possible_gamescope_pid = if !config.gamescope.steam_overlay_fix {
            launch_command.push(&gamescope_command);

            None
        } else {
            match spawn_gamescope_and_extract_xwayland_display(&gamescope_command).await {
                Err(error) => {
                    tracing::error!(
                        "Disabled GAMESCOPE.steam_overlay_fix due to an error, see: {error}"
                    );

                    launch_command.push(&gamescope_command);

                    None
                }
                Ok(result) => {
                    let (display_number, pid) = result;

                    env::set_var("DISPLAY", format!(":{display_number}"));

                    Some(pid)
                }
            }
        };

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

        if let Some(gamescope_pid) = possible_gamescope_pid {
            match signal::kill(Pid::from_raw(gamescope_pid as i32), Signal::SIGINT) {
                Err(error) => {
                    if error != Errno::ESRCH {
                        tracing::error!("Failed to terminate the gamescope's process, it might still be running in the background. See: {error:#?}")
                    }
                }
                Ok(_) => {
                    if let Err(error) = wait::waitpid(
                        Pid::from_raw(gamescope_pid as i32),
                        Some(WaitPidFlag::__WALL),
                    ) {
                        tracing::error!(
                            "Failed to wait for gamescope process to terminate, See: {error:#?}"
                        );
                    }
                }
            }
        }

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

#[derive(Debug, thiserror::Error)]
pub enum SpawnGamescopeAndExtractXwaylandDisplayError {
    #[error("Gamescope's process doesn't have a STDERR")]
    Stderr,

    #[error("Failed to run the gamescope command, see: {0:#?}")]
    RunCommand(io::Error),

    #[error("Failed to read the next line in gamescope process's STDERR, see: {0:#?}")]
    NextLine(io::Error),

    #[error("Xwayland display number could not be found")]
    DisplayNumber,
}

fn extract_xwayland_display_from_string(string: &str) -> Option<u16> {
    lazy_static! {
        static ref REGEX: Regex =
            Regex::new(r#"Starting Xwayland on :(\d+)"#).expect("Failed to compile the regex");
    }

    if let Some(Ok(display_number)) = REGEX.captures(string).and_then(|captures| {
        captures
            .get(1)
            .map(|r#match| r#match.as_str().parse::<u16>())
    }) {
        return Some(display_number);
    }

    None
}

async fn extract_xwayland_display_from_gamescope_stderr(
    stderr: ChildStderr,
) -> Result<Option<u16>, SpawnGamescopeAndExtractXwaylandDisplayError> {
    let mut lines = BufReader::new(stderr).lines();

    while let Some(line) = lines
        .next_line()
        .await
        .map_err(SpawnGamescopeAndExtractXwaylandDisplayError::NextLine)?
    {
        if let Some(display_number) = extract_xwayland_display_from_string(&line) {
            return Ok(Some(display_number));
        }
    }

    Ok(None)
}

async fn spawn_gamescope_and_extract_xwayland_display(
    gamescope_command: &str,
) -> Result<(u16, u32), SpawnGamescopeAndExtractXwaylandDisplayError> {
    let gamescope_process = Command::new("/bin/sh")
        .arg("-c")
        .arg(gamescope_command)
        .stderr(Stdio::piped())
        .spawn()
        .map_err(SpawnGamescopeAndExtractXwaylandDisplayError::RunCommand)?;

    let gamescope_pid = gamescope_process
        .id()
        .expect("Failed to get gamescope's PID");

    if let Some(stderr) = gamescope_process.stderr {
        if let Some(display_number) = extract_xwayland_display_from_gamescope_stderr(stderr).await?
        {
            return Ok((display_number, gamescope_pid));
        }

        return Err(SpawnGamescopeAndExtractXwaylandDisplayError::DisplayNumber);
    }

    Err(SpawnGamescopeAndExtractXwaylandDisplayError::Stderr)
}
