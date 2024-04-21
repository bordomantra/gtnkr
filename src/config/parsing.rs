use super::config_file::ConfigFile;
use super::ConfigError;
use super::Launch;
use ron::de::SpannedError;

impl Launch {
    pub async fn from_config_file(mut file: ConfigFile) -> Result<Self, ConfigError> {
        let contents = &file.read_to_string().await?;

        match parse_launch_config_from_ron_string(contents) {
            Ok(launch_config) => Ok(launch_config),
            Err(info) => {
                let (explanation, line, column) = info;

                Err(ConfigError::ParseError(
                    file.path,
                    explanation,
                    line,
                    column,
                ))
            }
        }
    }
}

fn parse_launch_config_from_ron_string(string: &str) -> Result<Launch, (String, u16, u16)> {
    match ron::from_str::<Launch>(string) {
        Ok(launch_config) => Ok(launch_config),
        Err(error) => {
            let explanation = error.code.to_string();
            let position = error.position;

            let (line, column) = (position.line as u16, position.col as u16);

            Err((explanation, line, column))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::{ScreenResolution, VulkanDriver};
    use super::parse_launch_config_from_ron_string;
    use color_eyre::eyre::{eyre, Result};

    #[test]
    fn parse_ron_into_launch_config() -> Result<()> {
        let launch_config_string = r#"(
            gamemode: false,
            mangohud: false,
            vulkan_driver: Amdvlk,
            gamescope: (
                source_resolution: Custom(1920, 1080),
                start_as_fullscreen: true,
                force_grab_cursor: false,
                tearing: true,
            ),
            environment_variables: [
                ("MESA_VK_WSI_PRESENT_MODE", "immediate"),
                ("vk_xwayland_wait_ready", "false"),
            ]
        )"#;

        match parse_launch_config_from_ron_string(launch_config_string) {
            Err(info) => {
                let (explanation, line, column) = info;

                Err(eyre!("Failed to parse the launch configuration from RON. Position {line}:{column}, {explanation}."))
            }
            Ok(launch_config) => {
                assert!(!launch_config.gamemode);
                assert!(!launch_config.mangohud);

                assert_eq!(launch_config.vulkan_driver, VulkanDriver::Amdvlk);
                assert_eq!(launch_config.environment_variables.len(), 2);

                let gamescope_config = launch_config.gamescope;

                assert_eq!(
                    gamescope_config.source_resolution,
                    ScreenResolution::Custom(1920, 1080)
                );

                assert!(gamescope_config.start_as_fullscreen);
                assert!(!gamescope_config.force_grab_cursor);
                assert!(gamescope_config.tearing);

                Ok(())
            }
        }
    }
}
