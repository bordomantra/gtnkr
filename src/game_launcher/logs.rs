use crate::process_output_log::{
    ProcessOutputLog, ProcessOutputLogKind, READABLE_TIMESTAMP_FORMAT,
    RUNTIME_PROCESS_OUTPUT_LOG_DIRECTORY,
};
use chrono::{Local, NaiveDateTime};
use std::fs;
use std::fs::File;
use std::io::Error as IoError;
use std::path::PathBuf;
use std::process::Stdio;

pub struct ActiveLaunchLog {
    game_identifier: String,
    timestamp: NaiveDateTime,
    kind: ProcessOutputLogKind,
}

impl ProcessOutputLog for ActiveLaunchLog {
    fn create(game_identifier: &str, kind: ProcessOutputLogKind) -> Result<Self, IoError> {
        let current_timestamp: NaiveDateTime = Local::now().naive_local();

        let process_output_log = Self {
            game_identifier: game_identifier.to_owned(),
            timestamp: current_timestamp.to_owned(),
            kind,
        };

        let log_file_path = process_output_log.as_path();

        fs::create_dir_all(
            log_file_path
                .parent()
                .expect("log_file_path in ActiveLaunchLog should've had a parent directory."),
        )?;

        File::create(log_file_path)?;

        Ok(process_output_log)
    }

    fn as_output_file(&self) -> Result<File, IoError> {
        let file_path = self.as_path();

        File::create(file_path)
    }

    fn as_stdio(&self) -> Result<Stdio, IoError> {
        let log_file = self.as_output_file()?;

        Ok(Stdio::from(log_file))
    }

    fn as_path(&self) -> PathBuf {
        let log_directory_path = RUNTIME_PROCESS_OUTPUT_LOG_DIRECTORY.to_path_buf();
        let readable_timestamp = self.timestamp.format(READABLE_TIMESTAMP_FORMAT);

        let file_extension = self.kind.as_file_extension();

        log_directory_path.join(format!(
            "{}/{}.{}",
            self.game_identifier, readable_timestamp, file_extension
        ))
    }
}
