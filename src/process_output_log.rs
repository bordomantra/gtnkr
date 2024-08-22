use crate::LOWERCASE_PACKAGE_NAME;
use chrono::{Local, NaiveDateTime};
use nix::unistd::{getuid, User};
use std::{
    fs,
    fs::{copy, File},
    io::Error as IoError,
    path::PathBuf,
    process::Stdio,
};

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

#[derive(Debug, thiserror::Error)]
pub enum ProcessOutputLogError {
    #[error("IO error while attempting to create the unique log directory `{1:#?}`, see: {0:#?}`")]
    CreateUniqueLogDirectory(IoError, PathBuf),

    #[error("IO error while attempting to create the output log `{1:#?}`, see: {0:#?}")]
    CreateOutputLogFile(IoError, PathBuf),

    #[error(
        "IO error while attempting to create a Stdio from the output log `{1:#?}`, see: {0:#?}"
    )]
    CreateStdioFromOutputLog(IoError, PathBuf),

    #[error("IO error while attempting to copy the contents of the runtime output log `{1:#?}` to the persistent output log `{2:#?}`, see: {0:#?}")]
    CopyRuntimeToPersistent(IoError, PathBuf, PathBuf),
}

type ProcessOutputLogResult<T> = Result<T, ProcessOutputLogError>;

pub trait ProcessOutputLog {
    fn create<S: ToString>(
        identifier: S,
        kind: ProcessOutputLogKind,
    ) -> ProcessOutputLogResult<Self>
    where
        Self: std::marker::Sized;

    fn as_path(&self) -> PathBuf;
    fn as_output_file(&self) -> ProcessOutputLogResult<File>;
    fn as_stdio(&self) -> ProcessOutputLogResult<Stdio>;
}

pub fn create_output_log_file(
    identifier: &str,
    timestamp: &NaiveDateTime,
    kind: &ProcessOutputLogKind,
    base_log_directory: PathBuf,
) -> Result<File, ProcessOutputLogError> {
    let log_file_path =
        generate_output_log_file_path(identifier, timestamp, kind, base_log_directory);

    let log_file_path_parent = log_file_path
        .parent()
        .expect("output_log_file_path should've had a parent directory.");

    fs::create_dir_all(log_file_path_parent).map_err(|error| {
        ProcessOutputLogError::CreateUniqueLogDirectory(error, log_file_path_parent.to_path_buf())
    })?;

    File::create(&log_file_path)
        .map_err(|error| ProcessOutputLogError::CreateOutputLogFile(error, log_file_path))
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

pub fn as_output_file<L: ProcessOutputLog>(
    process_output_log: &L,
) -> Result<File, ProcessOutputLogError> {
    let output_file_path = process_output_log.as_path();

    File::create(output_file_path.clone())
        .map_err(|error| ProcessOutputLogError::CreateOutputLogFile(error, output_file_path))
}

pub fn as_stdio<L: ProcessOutputLog>(
    process_output_log: &L,
) -> Result<Stdio, ProcessOutputLogError> {
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

pub struct ActiveOutputLog {
    identifier: String,
    timestamp: NaiveDateTime,
    kind: ProcessOutputLogKind,
    base_log_directory_path: PathBuf,
}

impl ProcessOutputLog for ActiveOutputLog {
    fn create<S: ToString>(
        identifier: S,
        kind: ProcessOutputLogKind,
    ) -> ProcessOutputLogResult<Self> {
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

    fn as_output_file(&self) -> ProcessOutputLogResult<File> {
        as_output_file(self)
    }

    fn as_stdio(&self) -> ProcessOutputLogResult<Stdio> {
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

pub struct PersistentOutputLog {
    identifier: String,
    timestamp: NaiveDateTime,
    kind: ProcessOutputLogKind,
    base_log_directory_path: PathBuf,
}

impl ProcessOutputLog for PersistentOutputLog {
    fn create<S: ToString>(
        identifier: S,
        kind: ProcessOutputLogKind,
    ) -> ProcessOutputLogResult<Self> {
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

    fn as_output_file(&self) -> ProcessOutputLogResult<File> {
        as_output_file(self)
    }

    fn as_stdio(&self) -> ProcessOutputLogResult<Stdio> {
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

impl PersistentOutputLog {
    pub fn from_active_output_log(active_log: ActiveOutputLog) -> ProcessOutputLogResult<Self> {
        let active_file_path = active_log.as_path();
        let persistent_log = PersistentOutputLog::create(active_log.identifier, active_log.kind)?;
        let persistent_file_path = persistent_log.as_path();

        copy(&active_file_path, &persistent_file_path).map_err(|error| {
            ProcessOutputLogError::CopyRuntimeToPersistent(
                error,
                active_file_path,
                persistent_file_path,
            )
        })?;

        Ok(persistent_log)
    }
}
