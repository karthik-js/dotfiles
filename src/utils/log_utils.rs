use colored::*;

/// Logs a message to the console with a timestamp.
///
/// # Arguments
/// * `message` - The message to log.
pub fn log_to_console(message: &str) {
    println!("{}", message);
}

/// Logs an informational message to the console with color.
///
/// # Arguments
/// * `message` - The informational message to log.
pub fn log_info(message: &str) {
    let info_message = format!("INFO: {}", message).bold().magenta();
    log_to_console(&info_message.to_string());
}

/// Logs an error message to the console with color.
///
/// # Arguments
/// * `message` - The error message to log.
pub fn log_error(message: &str) {
    let error_message = format!("ERROR: {}", message).red();
    log_to_console(&error_message.to_string());
}

pub fn log_success(message: &str) {
    let success_message = format!("SUCCESS: {}", message).green();
    log_to_console(&success_message.to_string());
}

pub fn log_warning(message: &str) {
    let warning_message = format!("WARNING: {}", message).yellow();
    log_to_console(&warning_message.to_string());
}
