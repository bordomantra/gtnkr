use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Default)]
pub enum VulkanDriver {
    #[default]
    Default,
    Amdvlk,
    Radv,
}

impl VulkanDriver {
    pub fn as_command(&self) -> Option<&str> {
        match self {
            Self::Default => None,
            Self::Amdvlk => Some("/bin/vk_amdvlk"),
            Self::Radv => Some("/bin/vk_radv"),
        }
    }
}
