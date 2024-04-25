use super::default_values::gamescope as gamescope_default_values;
use super::ScreenResolution;
use serde::Deserialize;
use std::{collections::HashMap, env};

const DEFAULT_GAMESCOPE_PATH: &str = "/bin/gamescope";

#[derive(Deserialize)]
pub struct Gamescope {
    #[serde(default = "gamescope_default_values::source_resolution")]
    pub source_resolution: ScreenResolution,

    #[serde(default = "gamescope_default_values::start_as_fullscreen")]
    pub start_as_fullscreen: bool,

    #[serde(default = "gamescope_default_values::force_grab_cursor")]
    pub force_grab_cursor: bool,

    #[serde(default = "gamescope_default_values::tearing")]
    pub tearing: bool,
}

impl Gamescope {
    pub async fn as_command(&self) -> String {
        let mut arguments: Vec<&str> = Vec::new();

        let screen_resolution_as_argument =
            self.source_resolution.as_gamescope_command_argument().await;

        arguments.push(&screen_resolution_as_argument);

        if self.start_as_fullscreen {
            arguments.push("--fullscreen")
        }

        if self.force_grab_cursor {
            arguments.push("--force-grab-cursor")
        }

        if self.tearing {
            arguments.push("--immediate-flips")
        }

        let gamescope_path = {
            let application_name = env!("CARGO_PKG_NAME").to_uppercase();

            if let Ok(path) = env::var(format!("{application_name}_GAMESCOPE_PATH")) {
                path
            } else {
                String::from(DEFAULT_GAMESCOPE_PATH)
            }
        };

        let arguments_as_string = arguments.join(" ");

        format!("{gamescope_path} {arguments_as_string}")
    }
}
