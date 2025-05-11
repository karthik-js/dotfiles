use std::time::Duration;

use crate::utils::{
    log_utils::log_step,
    spinner_utils::{get_random_spinner_frame, with_spinner},
};

pub fn setup_brew() {
    let spinner_frames = get_random_spinner_frame();
    let message = "Setting up Homebrew configuration...";
    let delay = Duration::from_millis(100);

    with_spinner(message, &spinner_frames, delay, || {
        super::install::ensure_brew_installed();
    });

    log_step("✅ Homebrew configuration setup complete.");
}
