use nix::unistd::{Uid, User};
use std::path::PathBuf;
use tokio::{fs, fs::OpenOptions, io, io::AsyncReadExt};

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
    path: PathBuf,
}

impl ConfigFile {
    pub async fn from_filename(filename: &str) -> Option<Self> {
        let linux_username = get_linux_username();
        let application_name = env!("CARGO_PKG_NAME");

        if linux_username == "root" {
            panic!("Attempted to get the config file for the root user, this isn't possible. Run the application as a normal user.")
        }

        let config_directory_path = PathBuf::from(&format!(
            "/home/{linux_username}/.config/{application_name}"
        ));

        if !config_directory_path.is_dir() {
            match fs::create_dir_all(&config_directory_path).await {
                Ok(_) => (),
                Err(error) => panic!("Failed to create the config directory, see: {error:#?}"),
            }
        };

        let config_file_path = config_directory_path.join(format!("{filename}.ron"));

        if config_file_path.is_file() {
            return Some(ConfigFile {
                path: config_file_path,
            });
        }

        None
    }

    pub async fn read_to_string(&mut self) -> Result<String, io::Error> {
        let mut file = OpenOptions::new().read(true).open(&self.path).await?;

        let mut contents = String::new();

        file.read_to_string(&mut contents).await?;

        Ok(contents)
    }
}
