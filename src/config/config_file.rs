use super::GameConfigError;
use nix::unistd::{Uid, User};
use std::{env, path::PathBuf};
use tokio::{fs::OpenOptions, io, io::AsyncReadExt};

fn get_linux_username() -> String {
    let uid = Uid::current();

    match User::from_uid(uid) {
        Err(error) => {
            panic!("Failed to get [nix::unistd::User] from [nix::unistd::Uid], see: {error:#?}")
        }
        Ok(potential_user) => {
            if let Some(user) = potential_user {
                return user.name;
            }

            panic!(
                "The [nix::unistd::User] that was received from [nix::unistd::User::from_uid()] is `None`"
            )
        }
    }
}

#[derive(Debug)]
pub struct GameConfigFile {
    pub path: PathBuf,
}

impl GameConfigFile {
    pub async fn from_filename(filename: &str) -> Result<Option<Self>, GameConfigError> {
        let linux_username = get_linux_username();
        let application_name = env!("CARGO_PKG_NAME");
        let config_dir_env_var_key = format!("{}_GAME_CONFIG_DIR", application_name.to_uppercase());

        let config_dir_path = match env::var(config_dir_env_var_key) {
            Ok(dir) => PathBuf::from(dir),
            Err(_) => {
                if linux_username == "root" {
                    return Err(GameConfigError::UserIsRoot);
                }

                PathBuf::from(&format!(
                    "/home/{}/.config/{}/game_configs",
                    linux_username, application_name
                ))
            }
        };

        let mut config_file_path = config_dir_path.join(filename);

        config_file_path.set_extension("ron");

        if config_file_path.is_file() {
            return Ok(Some(GameConfigFile {
                path: config_file_path,
            }));
        }

        Ok(None)
    }

    pub async fn read_to_string(&mut self) -> Result<String, GameConfigError> {
        match OpenOptions::new().read(true).open(&self.path).await {
            Err(error) => match error.kind() {
                io::ErrorKind::PermissionDenied => {
                    Err(GameConfigError::PermissionDenied(self.path.to_owned()))
                }
                io::ErrorKind::NotFound => Err(GameConfigError::NotFound(self.path.to_owned())),
                _ => Err(GameConfigError::UnexpectedIoError(error)),
            },
            Ok(mut file) => {
                let mut contents = String::new();

                file.read_to_string(&mut contents).await.map_err(|error| {
                    GameConfigError::InvalidFileEncoding(self.path.to_owned(), error)
                })?;

                Ok(contents)
            }
        }
    }
}
