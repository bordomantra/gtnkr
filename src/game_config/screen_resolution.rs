use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;
use tokio::{io, process::Command};

#[derive(Deserialize, Debug, PartialEq)]
pub enum ScreenResolution {
    Native,
    Custom(u16, u16),
}

impl ScreenResolution {
    pub async fn as_gamescope_command_argument(&self) -> String {
        match self {
            Self::Custom(width, height) => format!("-w {width} -h {height}"),
            Self::Native => match get_native_screen_resolution().await {
                Err(error) => panic!("Failed to get the native screen resolution, see: {error:#?}"),
                Ok(potential_screen_resolution) => {
                    if let Some(screen_resolution) = potential_screen_resolution {
                        let (width, height) = screen_resolution;

                        return format!("-w {width} -h {height}");
                    }

                    panic!("Failed to get the native screen resolution, you can set it yourself in the game configuration file with ScreenResolution::Custom(WIDTH, HEIGHT)")
                }
            },
        }
    }
}

async fn get_native_screen_resolution() -> Result<Option<(u16, u16)>, io::Error> {
    let output = Command::new("/bin/xrandr").output().await?;

    let stdout_as_string = String::from_utf8_lossy(&output.stdout);

    lazy_static! {
        static ref REGEX: Regex =
            Regex::new(r"connected (\d+)x(\d+)").expect("Failed to compile the regex");
    };

    if let Some(captures) = &REGEX.captures(&stdout_as_string) {
        if let (Some(width_string), Some(height_string)) = (captures.get(1), captures.get(2)) {
            if let Ok(width) = width_string.as_str().parse::<u16>() {
                if let Ok(height) = height_string.as_str().parse::<u16>() {
                    return Ok(Some((width, height)));
                }
            }
        }
    }

    Ok(None)
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
