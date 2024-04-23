use super::default_values::gamescope as gamescope_default_values;
use super::ScreenResolution;
use serde::Deserialize;

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
