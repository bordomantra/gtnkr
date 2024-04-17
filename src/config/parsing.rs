#[cfg(test)]
mod tests {
    use super::super::{Launch, ScreenResolution, VulkanDriver};

    #[test]
    fn parse_ron_into_launch_config() {
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

        let launch_config_result: Result<Launch, _> = ron::from_str(launch_config_string);

        match launch_config_result {
            Err(error) => {
                panic!(
                    "Failed to deserialize the launch config from RON, see: {:?}",
                    error
                );
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
            }
        }
    }
}
