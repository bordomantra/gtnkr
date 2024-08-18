use serde::Deserialize;
use super::find_executable;

const REQUIRED_PACKAGE: &str =
    "[amd-vulkan-prefixes](https://gitlab.com/AndrewShark/amd-vulkan-prefixes)";

#[derive(Deserialize, Debug, PartialEq, Default)]
pub enum VulkanDriver {
    #[default]
    Default,
    Amdvlk,
    Radv,
}

impl VulkanDriver {
    pub fn as_command(&self) -> Option<String> {
        match self {
            Self::Default => None,
            Self::Amdvlk => Some(find_executable("vk_amdvlk", REQUIRED_PACKAGE)),
            Self::Radv => Some(find_executable("vk_radv", REQUIRED_PACKAGE)),
        }
    }
}
