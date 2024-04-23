use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub enum VulkanDriver {
    Amdvlk,
    Radv,
}
