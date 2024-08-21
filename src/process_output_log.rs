use crate::LOWERCASE_PACKAGE_NAME;
use nix::unistd::getuid;
use std::{fs::File, io::Error as IoError, path::PathBuf, process::Stdio};

pub const READABLE_TIMESTAMP_FORMAT: &str = "%Y-%m-%d_%H:%M:%S";

lazy_static::lazy_static! {
    pub static ref RUNTIME_PROCESS_OUTPUT_LOG_DIRECTORY: PathBuf = {
        let uid = getuid();

        println!("/var/run/{}/{}/logs", uid, LOWERCASE_PACKAGE_NAME.as_str());

        PathBuf::from(format!("/run/user/{}/{}/process-output-logs", uid, LOWERCASE_PACKAGE_NAME.as_str()))

    };
}

pub enum ProcessOutputLogKind {
    Stderr,
    Stdout,
}

impl ProcessOutputLogKind {
    pub fn as_file_extension(&self) -> &str {
        match self {
            Self::Stderr => "errlog",
            Self::Stdout => "outlog",
        }
    }
}

pub trait ProcessOutputLog {
    fn create(identifier: &str, kind: ProcessOutputLogKind) -> Result<Self, IoError>
    where
        Self: std::marker::Sized;

    fn as_path(&self) -> PathBuf;
    fn as_output_file(&self) -> Result<File, IoError>;
    fn as_stdio(&self) -> Result<Stdio, IoError>;
}
