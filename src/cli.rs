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
        #[arg(value_parser = launch_subcommand_parser, long, short, env = "STEAM_LAUNCH_CMD")]
        steam_launch_command: (String, u32),

        #[arg(long, short, env = "LOG_LAUNCH_CMD_OUTPUT")]
        log_output: bool,
    },
}

fn launch_subcommand_parser(string: &str) -> Result<(String, u32), String> {
    // TODO: This regex could be improved, it's very easy to trick it.
    let steam_command_regex =
        Regex::new(r#"SteamLaunch AppId=(\d+)"#).expect("Failed to compile the regex");

    if let Some(steam_app_id) = steam_command_regex.captures(string).and_then(|captures| {
        captures
            .get(1)
            .and_then(|r#match| r#match.as_str().parse::<u32>().ok())
    }) {
        return Ok((string.to_string(), steam_app_id));
    }

    Err(String::from(
        "Provided argument is not a valid Steam launch %command%",
    ))
}

pub async fn run() -> Result<(), GameLauncherError> {
    let commands = Cli::parse();

    match &commands.subcommand {
        SubCommands::Launch {
            steam_launch_command,
            log_output,
        } => {
            let (launch_command, steam_app_id) = steam_launch_command;

            GameLauncher::launch_by_command(launch_command, &steam_app_id.to_string(), *log_output)
            .await
        }
    }
}
