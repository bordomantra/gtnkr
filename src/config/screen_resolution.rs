use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use tokio::{io, process::Command};

use hyprland::{
    data::{Monitor as HyprlandMonitor, Monitors as HyprlandMonitors},
    shared::{HyprData, HyprDataActive},
};

#[derive(Deserialize, Debug, PartialEq, Default)]
pub enum ScreenResolution {
    #[default]
    Native,
    Custom(u16, u16),
}

impl ScreenResolution {
    pub async fn as_gamescope_command_argument(&self) -> String {
        match self {
            Self::Custom(width, height) => format!("-w {width} -h {height}"),
            Self::Native => match get_native_screen_resolution().await {
                Err(error) => panic!("Failed to get the native screen resolution, see: {error:#?}"),
                Ok(screen_resolution) => {
                    let (width, height) = screen_resolution;

                    format!("-w {width} -h {height}")
                }
            },
        }
    }
}

#[derive(Deserialize, Debug)]
struct Monitor {
    width: u32,
    height: u32,
    focused: bool,
}

async fn get_native_screen_resolution() -> hyprland::Result<(u16, u16)> {
    let active_monitor_info = HyprlandMonitor::get_active()?;

    Ok((active_monitor_info.width, active_monitor_info.height))
}

#[cfg(test)]
mod tests {
    use super::get_native_screen_resolution;
    use color_eyre::eyre::Result;

    #[tokio::test]
    async fn test_get_native_screen_resolution() -> Result<()> {
        get_native_screen_resolution().await?;

        Ok(())
    }
}
