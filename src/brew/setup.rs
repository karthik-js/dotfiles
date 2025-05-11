use std::time::Duration;

use crate::utils::{
    log_utils::log_success,
    spinner_utils::{get_random_spinner_frame, with_spinner},
};

pub fn setup_brew() {
    let spinner_frames = get_random_spinner_frame();
    let message = "Setting up Homebrew configuration...";
    let delay = Duration::from_millis(100);

    with_spinner(message, &spinner_frames, delay, || {
        super::install::ensure_brew_installed();
        super::install_packages::run_brew_bundle("configurations/brew/Brewfile");
    });

    log_success("✅ Homebrew configuration setup complete.");
}
