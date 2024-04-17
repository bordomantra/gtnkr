use super::{default_values::launch as launch_default_values, Gamescope, VulkanDriver};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Launch {
    #[serde(default = "launch_default_values::gamemode")]
    pub gamemode: bool,

    #[serde(default = "launch_default_values::mangohud")]
    pub mangohud: bool,

    #[serde(default = "launch_default_values::vulkan_driver")]
    pub vulkan_driver: VulkanDriver,

    #[serde(default = "launch_default_values::gamescope")]
    pub gamescope: Gamescope,

    #[serde(default = "launch_default_values::environment_variables")]
    pub environment_variables: Vec<(String, String)>,
}
