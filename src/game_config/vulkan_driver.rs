use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub enum VulkanDriver {
    Amdvlk,
    Radv,
}

impl VulkanDriver {
    pub async fn as_command(&self) -> &str {
        match self {
            Self::Amdvlk => "/bin/vk_amdvlk",
            Self::Radv => "/bin/vk_radv",
        }
    }
}
