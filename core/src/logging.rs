
//! logger and utilities

use log::LevelFilter;
use simplelog::{Config, WriteLogger};

use std::fs::File;

/// Logger errors
#[derive(Debug)]
pub enum LoggerError {
    /// Failed to create log file
    LogFileCreationError(String),

    /// Failed logger::init
    LoggerInitError
}

/// Initializes the logger with the given logging level and a dated name
pub fn init_logging(level: LevelFilter) -> Result<(), LoggerError> {
    let config = Config::default();
    
    let log_filename = format!("logs/{}.log", chrono::Local::now().format("%Y%m%d"));
    match File::create(&log_filename) {
        Ok(file) => WriteLogger::init( level, config, file ).or(Err(LoggerError::LoggerInitError)),
        Err(_) => Err(LoggerError::LogFileCreationError(log_filename))
    }
}
