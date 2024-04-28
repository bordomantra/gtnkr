use crate::game_launcher::{GameLauncher, GameLauncherError};
use regex::Regex;

pub async fn run() -> Result<(), GameLauncherError> {
    let mut arguments: Vec<String> = std::env::args().collect();

    arguments.remove(0); // Current directory's path, we don't need that.

    let argument_as_string = arguments.join(" ");

    let steam_app_id_regex =
        Regex::new(r#"SteamLaunch AppId=(\d+)"#).expect("Failed to compile regex");

    if let Some(steam_app_id) =
        steam_app_id_regex
            .captures(&argument_as_string)
            .and_then(|captures| {
                captures
                    .get(1)
                    .and_then(|r#match| r#match.as_str().parse::<u64>().ok())
            })
    {
        GameLauncher::launch_by_command(&argument_as_string, &format!("{steam_app_id}.ron"))
            .await?;

        return Ok(());
    }

    if let Ok(executable_path) = which::which(argument_as_string) {
        GameLauncher::launch_by_executable(
            executable_path
                .file_name()
                .expect("Failed to get the file name")
                .to_str()
                .expect("Failed to convert into string"),
        )
        .await?;

        return Ok(());
    }

    Ok(())
}
