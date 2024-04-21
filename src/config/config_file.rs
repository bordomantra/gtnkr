use super::ConfigError;
use nix::unistd::{Uid, User};
use std::path::PathBuf;
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
pub struct ConfigFile {
    pub path: PathBuf,
}

impl ConfigFile {
    pub async fn from_filename(filename: &str) -> Result<Option<Self>, ConfigError> {
        let linux_username = get_linux_username();
        let application_name = env!("CARGO_PKG_NAME");

        if linux_username == "root" {
            return Err(ConfigError::UserIsRoot);
        }

        let config_directory_path = PathBuf::from(&format!(
            "/home/{linux_username}/.config/{application_name}"
        ));

        let config_file_path = config_directory_path.join(format!("{filename}.ron"));

        if config_file_path.is_file() {
            return Ok(Some(ConfigFile {
                path: config_file_path,
            }));
        }

        Ok(None)
    }

    pub async fn read_to_string(&mut self) -> Result<String, ConfigError> {
        match OpenOptions::new().read(true).open(&self.path).await {
            Err(error) => match error.kind() {
                io::ErrorKind::PermissionDenied => {
                    Err(ConfigError::PermissionDenied(self.path.to_owned()))
                }
                io::ErrorKind::NotFound => Err(ConfigError::NotFound(self.path.to_owned())),
                _ => todo!(),
            },
            Ok(mut file) => {
                let mut contents = String::new();

                file.read_to_string(&mut contents).await.map_err(|error| {
                    ConfigError::InvalidFileEncoding(self.path.to_owned(), error)
                })?;

                Ok(contents)
            }
        }
    }
}
