mod logs;

use crate::config::{GameConfig, GameConfigError, GameConfigFile};
use crate::process_output_log::{ProcessOutputLog, ProcessOutputLogKind};
use logs::ActiveLaunchLog;
use phf::phf_map;
use std::process::Stdio;
use std::{env, path::PathBuf};
use tokio::{io, process::Command};
use which::which;

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

    #[error(r#"Failed to locate the cli tool "{0}", do you have {1} installed?"#)]
    MissingCliTool(String, String),

    #[error("Failed to run the launch or gamescope command, see: {0:#?}")]
    RunCommand(io::Error),
}

pub struct GameLauncher {}

impl GameLauncher {
    pub async fn launch_by_command(
        command: &str,
        config_file_name: &str,
        game_identifier: &str,
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

        let mut launch_command: Vec<String> = Vec::new();

        if config.gamemode {
            launch_command.push(find_executable_gml("gamemoderun")?);
        }

        if config.mangohud {
            launch_command.push(find_executable_gml("mangohud")?);
        }

        if let Some(gamescope_config) = config.gamescope {
            if let Ok(gamescope_path) = env::var(format!(
                "{}_GAMESCOPE_PATH",
                crate::UPPERCASE_PACKAGE_NAME.as_str()
            )) {
                launch_command.push(gamescope_config.as_command(&gamescope_path));
            } else {
                launch_command
                    .push(gamescope_config.as_command(&find_executable_gml("gamescope")?));
            }
        }

        if config.fps_limit > 0 {
            launch_command.push(format!(
                "{} {}",
                find_executable_gml("strangle")?,
                config.fps_limit
            ));
        }

        if let Some(vulkan_driver) = config.vulkan_driver.as_command() {
            launch_command.push(find_executable_gml(vulkan_driver)?);
        }

        config
            .environment_variables
            .iter()
            .for_each(|(key, value)| env::set_var(key, value));

        let launch_command_string = format!("{} {}", launch_command.join(" "), command);

        tracing::info!("Launching the game with [{launch_command_string}]");

        let logged_stderr = ActiveLaunchLog::create(game_identifier, ProcessOutputLogKind::Stderr)
            .unwrap()
            .as_stdio()
            .unwrap();

        let mut process = Command::new("sh")
            .arg("-c")
            .arg(launch_command_string)
            .stderr(logged_stderr)
            .spawn()
            .map_err(GameLauncherError::RunCommand)?;

        let _ = process.wait().await;

        Ok(())
    }
}

static CLI_TOOL_INFO: phf::Map<&'static str, &'static str> = phf_map! {
    "gamemoderun" => "[gamemode](https://github.com/FeralInteractive/gamemode)",
    "mangohud" => "[MangoHud](https://github.com/flightlessmango/MangoHud)",
    "gamescope" => "[gamescope](https://github.com/ValveSoftware/gamescope)",
    "libstrangle" => "[libstrangle](https://github.com/milaq/libstrangle)",

    "vk_amdvlk" => "[amd-vulkan-prefixes](https://gitlab.com/AndrewShark/amd-vulkan-prefixes)",
    "vk_radv" => "[amd-vulkan-prefixes](https://gitlab.com/AndrewShark/amd-vulkan-prefixes)",
};

fn find_executable_gml(name: &str) -> Result<String, GameLauncherError> {
    if let Ok(executable_path) =
        which(name).map(|executable| executable.to_string_lossy().to_string())
    {
        return Ok(executable_path);
    }

    if let Some(pkg_name) = CLI_TOOL_INFO.get(name) {
        return Err(GameLauncherError::MissingCliTool(
            name.to_string(),
            pkg_name.to_string(),
        ));
    }

    Err(GameLauncherError::MissingCliTool(
        name.to_string(),
        String::from("<[Undefined, please create an issue]>"),
    ))
}
