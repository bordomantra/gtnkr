use super::ScreenResolution;
use crate::UPPERCASE_PACKAGE_NAME;
use serde::Deserialize;
use std::{collections::HashMap, env};

const fn _default_start_as_fullscreen() -> bool {
    true
}

const fn _default_force_grab_cursor() -> bool {
    true
}

const fn _default_tearing() -> bool {
    true
}

const fn _default_mangoapp() -> bool {
    true
}

const fn _default_expose_wayland() -> bool {
    false
}

#[derive(Deserialize)]
pub struct Gamescope {
    #[serde(default)]
    pub source_resolution: ScreenResolution,

    #[serde(default = "_default_start_as_fullscreen")]
    pub start_as_fullscreen: bool,

    #[serde(default = "_default_force_grab_cursor")]
    pub force_grab_cursor: bool,

    #[serde(default = "_default_tearing")]
    pub tearing: bool,

    #[serde(default = "_default_mangoapp")]
    pub mangoapp: bool,

    #[serde(default)]
    pub backend: GamescopeBackend,

    #[serde(default = "_default_expose_wayland")]
    pub expose_wayland: bool,
}

impl Default for Gamescope {
    fn default() -> Self {
        Self {
            source_resolution: ScreenResolution::default(),
            start_as_fullscreen: _default_start_as_fullscreen(),
            force_grab_cursor: _default_force_grab_cursor(),
            tearing: _default_tearing(),
            mangoapp: _default_mangoapp(),
            backend: GamescopeBackend::default(),
            expose_wayland: _default_expose_wayland(),
        }
    }
}

#[derive(Deserialize, Default, PartialEq)]
pub enum GamescopeBackend {
    #[default]
    Auto,
    Wayland,
}

impl GamescopeBackend {
    fn as_gamescope_command_argument(&self) -> String {
        let argument_var = match self {
            Self::Auto => "auto",
            Self::Wayland => "wayland",
        };

        format!("--backend {argument_var}")
    }
}

impl Gamescope {
    pub fn as_command(&self, gamescope_executable_path: &str) -> String {
        let mut arguments: Vec<&str> = Vec::new();

        let screen_resolution_as_argument = self.source_resolution.as_gamescope_command_argument();

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

        if self.mangoapp {
            arguments.push("--mangoapp")
        }

        let backend_as_argument = self.backend.as_gamescope_command_argument();

        if self.backend != GamescopeBackend::Auto {
            arguments.push(&backend_as_argument);
        }

        if self.expose_wayland {
            arguments.push("--expose-wayland")
        }

        let arguments_as_string = arguments.join(" ");

        format!("{gamescope_executable_path} {arguments_as_string}")
    }
}
