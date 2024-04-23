use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub enum ScreenResolution {
    Native,
    Custom(u16, u16),
}
