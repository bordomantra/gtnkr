use crate::LOWERCASE_PACKAGE_NAME;
use chrono::{Local, NaiveDateTime};
use nix::unistd::{getuid, User};

use std::{fs, fs::File, io::Error as IoError, path::PathBuf, process::Stdio};

pub const READABLE_TIMESTAMP_FORMAT: &str = "%Y-%m-%d_%H:%M:%S";

lazy_static::lazy_static! {
    pub static ref RUNTIME_PROCESS_OUTPUT_LOG_DIRECTORY: PathBuf = {
        let uid = getuid();
        let package_name = LOWERCASE_PACKAGE_NAME.as_str();

        PathBuf::from(format!("/run/user/{uid}/{package_name}/process-output-logs"))
    };

    pub static ref PERSISTENT_PROCESS_OUTPUT_LOG_DIRECTORY: PathBuf = {
        let uid = getuid();
        let user = User::from_uid(uid).expect("Should've been able get the current linux user by uid").expect("Linux user should've existed");

        let home_directory = user.dir;
        let package_name = LOWERCASE_PACKAGE_NAME.as_str();

        home_directory.join(format!("{package_name}/process-output-logs"))
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
    fn create<S: ToString>(identifier: S, kind: ProcessOutputLogKind) -> Result<Self, IoError>
    where
        Self: std::marker::Sized;

    fn as_path(&self) -> PathBuf;
    fn as_output_file(&self) -> Result<File, IoError>;
    fn as_stdio(&self) -> Result<Stdio, IoError>;
}

pub fn create_output_log_file(
    identifier: &str,
    timestamp: &NaiveDateTime,
    kind: &ProcessOutputLogKind,
    base_log_directory: PathBuf,
) -> Result<File, IoError> {
    let log_file_path =
        generate_output_log_file_path(identifier, timestamp, kind, base_log_directory);

    fs::create_dir_all(
        log_file_path
            .parent()
            .expect("output_log_file_path should've had a parent directory."),
    )?;

    File::create(log_file_path)
}

pub fn generate_output_log_file_path(
    identifier: &str,
    timestamp: &NaiveDateTime,
    kind: &ProcessOutputLogKind,
    base_log_directory: PathBuf,
) -> PathBuf {
    let readable_timestamp = timestamp.format(READABLE_TIMESTAMP_FORMAT);

    let file_extension = kind.as_file_extension();

    base_log_directory.join(format!(
        "{}/{}.{}",
        identifier, readable_timestamp, file_extension
    ))
}

pub fn as_output_file<L: ProcessOutputLog>(process_output_log: &L) -> Result<File, IoError> {
    let file_path = process_output_log.as_path();

    File::create(file_path)
}

pub fn as_stdio<L: ProcessOutputLog>(process_output_log: &L) -> Result<Stdio, IoError> {
    let log_file = process_output_log.as_output_file()?;

    Ok(Stdio::from(log_file))
}

pub fn create(
    identifier: String,
    kind: ProcessOutputLogKind,
    base_log_directory_path: PathBuf,
) -> (String, NaiveDateTime, ProcessOutputLogKind, PathBuf) {
    let current_timestamp: NaiveDateTime = Local::now().naive_local();

    let _ = create_output_log_file(
        &identifier,
        &current_timestamp,
        &kind,
        base_log_directory_path.clone(),
    );

    (identifier, current_timestamp, kind, base_log_directory_path)
}

pub struct ActiveLog {
    identifier: String,
    timestamp: NaiveDateTime,
    kind: ProcessOutputLogKind,
    base_log_directory_path: PathBuf,
}

impl ProcessOutputLog for ActiveLog {
    fn create<S: ToString>(identifier: S, kind: ProcessOutputLogKind) -> Result<Self, IoError> {
        let (identifier, timestamp, kind, base_log_directory_path) = create(
            identifier.to_string(),
            kind,
            RUNTIME_PROCESS_OUTPUT_LOG_DIRECTORY.to_path_buf(),
        );

        Ok(Self {
            identifier,
            timestamp,
            kind,
            base_log_directory_path,
        })
    }

    fn as_output_file(&self) -> Result<File, IoError> {
        as_output_file(self)
    }

    fn as_stdio(&self) -> Result<Stdio, IoError> {
        as_stdio(self)
    }

    fn as_path(&self) -> PathBuf {
        generate_output_log_file_path(
            &self.identifier,
            &self.timestamp,
            &self.kind,
            self.base_log_directory_path.clone(),
        )
    }
}

pub struct PersistentLog {
    identifier: String,
    timestamp: NaiveDateTime,
    kind: ProcessOutputLogKind,
    base_log_directory_path: PathBuf,
}

impl ProcessOutputLog for PersistentLog {
    fn create<S: ToString>(identifier: S, kind: ProcessOutputLogKind) -> Result<Self, IoError> {
        let (identifier, timestamp, kind, base_log_directory_path) = create(
            identifier.to_string(),
            kind,
            PERSISTENT_PROCESS_OUTPUT_LOG_DIRECTORY.to_path_buf(),
        );

        Ok(Self {
            identifier,
            timestamp,
            kind,
            base_log_directory_path,
        })
    }

    fn as_output_file(&self) -> Result<File, IoError> {
        as_output_file(self)
    }

    fn as_stdio(&self) -> Result<Stdio, IoError> {
        as_stdio(self)
    }

    fn as_path(&self) -> PathBuf {
        generate_output_log_file_path(
            &self.identifier,
            &self.timestamp,
            &self.kind,
            self.base_log_directory_path.clone(),
        )
    }
}
