use crate::game_launcher::{GameLauncher, GameLauncherError};
use clap::{Parser, Subcommand};
use regex::Regex;

#[derive(Parser)]
#[command(version, about)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    subcommand: SubCommands,
}

#[derive(Subcommand)]
enum SubCommands {
    Launch {
        #[arg(value_parser = launch_subcommand_parser)]
        game: GameArgument,
    },
}

#[derive(Clone)]
enum GameArgument {
    Executable(String),
    SteamLaunchCommand(String, u32),
}

fn launch_subcommand_parser(string: &str) -> Result<GameArgument, String> {
    // TODO: This regex could be improved, it's very easy to trick it.
    let steam_command_regex =
        Regex::new(r#"SteamLaunch AppId=(\d+)"#).expect("Failed to compile the regex");

    if let Ok(executable_path) = which::which(string) {
        if let Some(path_string) = executable_path.to_str() {
            return Ok(GameArgument::Executable(path_string.to_string()));
        }

        return Err(String::from("Provided argument is a valid executable, but I wasn't able to convert it's path to string."));
    }

    if let Some(steam_app_id) = steam_command_regex.captures(string).and_then(|captures| {
        captures
            .get(1)
            .and_then(|r#match| r#match.as_str().parse::<u32>().ok())
    }) {
        return Ok(GameArgument::SteamLaunchCommand(
            string.to_string(),
            steam_app_id,
        ));
    }

    Err(String::from(
        "Provided argument is neither a valid executable or a Steam launch %command%",
    ))
}

pub async fn run() -> Result<(), GameLauncherError> {
    let commands = Cli::parse();

    match &commands.subcommand {
        SubCommands::Launch { game } => match game {
            GameArgument::Executable(path_string) => {
                GameLauncher::launch_by_executable(path_string).await
            }
            GameArgument::SteamLaunchCommand(command, steam_app_id) => {
                GameLauncher::launch_by_command(command, &format!("{steam_app_id}.ron")).await
            }
        },
    }
}
