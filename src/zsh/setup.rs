use crate::utils::log_utils::log_success;
use crate::utils::spinner_utils::get_random_spinner_frame;
use crate::utils::spinner_utils::with_spinner;

use super::install::install_oh_my_zsh_if_needed;
use super::symlink::symlink_zsh_configs;
use std::time::Duration;

pub fn setup_zsh() {
    let spinner_frames = get_random_spinner_frame();
    let message = "Setting up Zsh configuration...";
    let delay = Duration::from_millis(100);

    with_spinner(message, &spinner_frames, delay, || {
        install_oh_my_zsh_if_needed();
        symlink_zsh_configs();
    });

    log_success("✅ Zsh configuration setup complete.");
}
