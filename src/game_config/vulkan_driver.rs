use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Default)]
pub enum VulkanDriver {
    #[default]
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
