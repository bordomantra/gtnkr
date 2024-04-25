#![allow(unused)]

use crate::game_config::{GameConfig, Gamescope};

#[derive(Debug, thiserror::Error)]
pub enum GameConfigError {
    //#[error(""))]
    //UserIsRoot,
}

pub struct GameExecutableLauncher {
    config: GameConfig,
}

impl GameExecutableLauncher {
    async fn launch_game_executable(&self) -> Result<u32, GameConfigError> {
        todo!()
    }
}
