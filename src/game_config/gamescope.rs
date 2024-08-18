use super::ScreenResolution;
use crate::UPPERCASE_PACKAGE_NAME;
use serde::Deserialize;
use std::{collections::HashMap, env};

const DEFAULT_GAMESCOPE_PATH: &str = "/bin/gamescope";

const fn _default_source_resolution() -> ScreenResolution {
    ScreenResolution::Native
}

const fn _default_start_as_fullscreen() -> bool {
    true
}

const fn _default_force_grab_cursor() -> bool {
    true
}

const fn _default_tearing() -> bool {
    true
}

const fn _default_steam_overlay_fix() -> bool {
    true
}

#[derive(Deserialize, Default)]
pub struct Gamescope {
    #[serde(default = "_default_source_resolution")]
    pub source_resolution: ScreenResolution,

    #[serde(default = "_default_start_as_fullscreen")]
    pub start_as_fullscreen: bool,

    #[serde(default = "_default_force_grab_cursor")]
    pub force_grab_cursor: bool,

    #[serde(default = "_default_tearing")]
    pub tearing: bool,

    #[serde(default = "_default_steam_overlay_fix")]
    pub steam_overlay_fix: bool,
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
            if let Ok(path) = env::var(format!(
                "{}_GAMESCOPE_PATH",
                UPPERCASE_PACKAGE_NAME.as_str()
            )) {
                path
            } else {
                String::from(DEFAULT_GAMESCOPE_PATH)
            }
        };

        let arguments_as_string = arguments.join(" ");

        format!("{gamescope_path} {arguments_as_string}")
    }
}
