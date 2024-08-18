use super::config_file::GameConfigFile;
use super::{GameConfig, GameConfigError};

impl GameConfig {
    pub async fn from_game_config_file(mut file: GameConfigFile) -> Result<Self, GameConfigError> {
        let contents = file.read_to_string().await?;

        match ron::from_str::<GameConfig>(&contents) {
            Ok(launch_config) => Ok(launch_config),
            Err(error) => {
                let explanation = error.code.to_string();
                let position = error.position;

                let (line, column) = (position.line as u16, position.col as u16);

                Err(GameConfigError::ParseError(
                    file.path,
                    explanation,
                    line,
                    column,
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::{config_file::GameConfigFile, GameConfig, ScreenResolution, VulkanDriver};
    use crate::UPPERCASE_PACKAGE_NAME;
    use color_eyre::eyre;
    use lazy_static::lazy_static;
    use std::{env, path::Path};
    use tempdir::TempDir;
    use tokio::{fs, io::AsyncWriteExt};

    const GAME_CONFIG_RON_STRING: &str = r#"(
            gamemode: false,
            mangohud: false,
            vulkan_driver: Amdvlk,
            gamescope: (
                source_resolution: Custom(1920, 1080),
                start_as_fullscreen: true,
                tearing: true,
            ),
            environment_variables: [
                ("MESA_VK_WSI_PRESENT_MODE", "immediate"),
                ("vk_xwayland_wait_ready", "false"),
            ]
        )"#;

    lazy_static! {
        static ref GAME_CONFIG_DIR_ENV_KEY: String =
            format!("{}_GAME_CONFIG_DIR", UPPERCASE_PACKAGE_NAME.as_str());
    }

    #[tokio::test]
    async fn parse_game_config_file_into_launch_config() -> eyre::Result<()> {
        let temp_dir = TempDir::new("parse_config_file_into_launch_config")?;
        let file_path = temp_dir.path().join("test_config.ron");
        let mut file = fs::File::create(&file_path).await?;

        file.write_all(GAME_CONFIG_RON_STRING.as_bytes()).await?;

        // With this environment variable set, GameConfigFile::from_filename()
        // searches the file name in the specified directory, instead of the default
        // /home/<LINUX_USERNAME>/.config/<CARGO_PKG_NAME>/game_configs.
        // This should be unset if any of the other tests use tracing_test::traced_test, because
        // because those tests can use the environment values set by other tests
        env::set_var(GAME_CONFIG_DIR_ENV_KEY.as_str(), temp_dir.path());

        let config_file = GameConfigFile::from_filename("test_config.ron")
            .await?
            .expect("File `test_config.ron` couldn't be found");

        env::remove_var(GAME_CONFIG_DIR_ENV_KEY.as_str());

        let config = GameConfig::from_game_config_file(config_file).await?;

        assert!(!config.gamemode);
        assert!(!config.mangohud);

        assert_eq!(config.vulkan_driver, VulkanDriver::Amdvlk);
        assert_eq!(config.environment_variables.len(), 2);

        let gamescope_config = config.gamescope;

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
