use rand::Rng;

use colored::*;
use std::{
    io::{self, Write},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::{Duration, Instant},
};

use crate::utils::log_utils::{log_error, log_success};

/// Runs a task while displaying an animated spinner with a message.
///
/// # Arguments
///
/// * `message` - A message to display alongside the spinner.
/// * `frames` - An array of frame strings used for the spinner animation.
/// * `delay` - Duration to wait between spinner frame updates.
/// * `task` - The task to run while the spinner is active.
pub fn with_spinner<F>(message: &str, frames: &[&str], delay: Duration, task: F)
where
    F: FnOnce(),
{
    let message = message.to_string(); // Clone to own the message string
    let frames: Vec<String> = frames.iter().map(|s| s.to_string()).collect();

    // Print the message once
    if let Err(e) = write!(io::stdout(), "{} {}", "ℹ️".blue(), message.yellow()) {
        log_error(&format!("Error writing to stdout: {}", e));
        return;
    }
    if let Err(e) = io::stdout().flush() {
        log_error(&format!("Error flushing stdout: {}", e));
        return;
    }

    let stop_flag = Arc::new(AtomicBool::new(false));
    let spinner_flag = Arc::clone(&stop_flag);
    let message_for_spinner = message.clone();

    // Start the spinner in a separate thread
    let spinner_thread = thread::spawn(move || {
        while !spinner_flag.load(Ordering::Relaxed) {
            for frame in &frames {
                if spinner_flag.load(Ordering::Relaxed) {
                    break;
                }
                if let Err(e) = write!(
                    io::stdout(),
                    "\r{} {}",
                    frame.cyan(),
                    message_for_spinner.yellow()
                ) {
                    log_error(&format!("Error writing to stdout: {}", e));
                    return;
                }
                if let Err(e) = io::stdout().flush() {
                    log_error(&format!("Error flushing stdout: {}", e));
                    return;
                }
                thread::sleep(delay);
            }
        }
    });

    // Measure the time taken for the task
    let start_time = Instant::now();
    task();
    let duration = start_time.elapsed();

    // Stop the spinner
    stop_flag.store(true, Ordering::Relaxed);
    if let Err(e) = spinner_thread.join() {
        log_error(&format!("Error joining spinner thread: {:?}", e));
    }

    // Print the completion message with time taken
    log_success(&format!(
        "\r{} {} (completed in {:.2?})",
        "✅".green(),
        message.yellow(),
        duration
    ));
}

const SPINNER_FRAMES: &[&[&str]] = &[
    &["|", "/", "-", "\\", "|", "/", "-", "\\", "|"],
    &["◐", "◓", "◑", "◒", "◐", "◓", "◑", "◒"],
    &["◜", "◝", "◞", "◟", "◜", "◝", "◞", "◟"],
    &["◠", "◡", "◢", "◣", "◠", "◡", "◢", "◣"],
    &["◌", "◍", "◌", "◍", "◌", "◍", "◌", "◍"],
    &["◉", "◎", "◉", "◎", "◉", "◎", "◉", "◎"],
    &["◈", "◉", "◈", "◉", "◈", "◉", "◈", "◉"],
    &["◯", "◌", "◯", "◌", "◯", "◌", "◯", "◌"],
    &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
];

/// Get a random spinner frame set.
pub fn get_random_spinner_frame() -> &'static [&'static str] {
    let mut rng = rand::rng();
    let index = rng.random_range(0..SPINNER_FRAMES.len());
    SPINNER_FRAMES[index]
}
