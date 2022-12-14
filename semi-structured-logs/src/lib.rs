// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

/// various log levels
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
    Debug
}
/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {

    let mut log_message= String::from("");

    match level {
        LogLevel::Info => log_message = info(message),
        LogLevel::Warning => log_message = warn(message),
        LogLevel::Error => log_message = error(message),
        LogLevel::Debug => log_message = debug(message),
        _=> log_message.push_str("Invalid Log Level")
    }


    log_message

}
pub fn info(message: &str) -> String {

    let mut level_to_string= String::from("[INFO]: ");
    level_to_string.push_str(message);

    level_to_string

}
pub fn warn(message: &str) -> String {
    let mut level_to_string= String::from("[WARNING]: ");
    level_to_string.push_str(message);

    level_to_string
}
pub fn error(message: &str) -> String {
    let mut level_to_string= String::from("[ERROR]: ");
    level_to_string.push_str(message);

    level_to_string
}

pub fn debug(message: &str) -> String {
    let mut level_to_string= String::from("[DEBUG]: ");
    level_to_string.push_str(message);

    level_to_string
}