use colored::*;
use std::{
    io::Write,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::Duration,
};

pub fn show_spinner(
    stop_flag: Arc<AtomicBool>,
    spinner_frames: &[&str],
    message: &str,
    delay: Duration,
) {
    while !stop_flag.load(Ordering::Relaxed) {
        for frame in spinner_frames {
            if stop_flag.load(Ordering::Relaxed) {
                break;
            }
            print!("\r{} {}", frame.cyan(), message.yellow());
            std::io::stdout().flush().unwrap();
            thread::sleep(delay);
        }
    }
    println!("\r{} {}", "✨".green(), message.yellow()); // Clear spinner line
}

pub fn with_spinner<F>(message: &str, spinner_frames: &[&str], delay: Duration, setup_logic: F)
where
    F: FnOnce(),
{
    let stop_flag = Arc::new(AtomicBool::new(false));
    let spinner_flag = stop_flag.clone();

    let message = message.to_string();
    let spinner_frames: Vec<String> = spinner_frames.iter().map(|&s| s.to_string()).collect();

    let spinner_thread = thread::spawn(move || {
        show_spinner(
            spinner_flag,
            &spinner_frames
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>(),
            &message,
            delay,
        );
    });

    setup_logic();

    stop_flag.store(true, Ordering::Relaxed);
    spinner_thread.join().unwrap();
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

// get random spinner frame
pub fn get_random_spinner_frame() -> &'static [&'static str] {
    let mut rng = rand::rng();
    let index = rand::Rng::random_range(&mut rng, 0..SPINNER_FRAMES.len());
    SPINNER_FRAMES[index]
}
