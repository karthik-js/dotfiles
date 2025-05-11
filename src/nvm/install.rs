use std::process::Command;

use crate::utils::log_utils::{log_error, log_info, log_success};

pub fn install_nvm() {
    let nvm_dir = dirs::home_dir()
        .map(|home| home.join(".nvm"))
        .expect("Failed to determine home directory");

    if nvm_dir.exists() {
        log_info("NVM is already installed.");
        return;
    }

    println!("Installing NVM...");

    if !Command::new("sh")
        .arg("-c")
        .arg("curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | bash")
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
    {
        log_error("Failed to install NVM. Please check your internet connection and try again.");
        return;
    }

    log_success("✅ NVM installed successfully.");
}
