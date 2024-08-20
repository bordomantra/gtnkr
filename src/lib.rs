pub mod cli;
mod config;
mod game_launcher;
pub mod process_output_log;

lazy_static::lazy_static! {
    pub static ref UPPERCASE_PACKAGE_NAME: String = {
        env!("CARGO_PKG_NAME").to_uppercase()
    };

    pub static ref LOWERCASE_PACKAGE_NAME: String = {
        env!("CARGO_PKG_NAME").to_lowercase()
    };
}
