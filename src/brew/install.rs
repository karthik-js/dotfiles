use crate::utils::log_utils::{log_error, log_info, log_success};
use std::process::Command;

const HOMEBREW_INSTALL_SCRIPT: &str =
    "https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh";

pub fn ensure_brew_installed() {
    log_info("🔍 Checking for Homebrew installation...");
    if !is_brew_installed() {
        log_info("ℹ️ Homebrew is not installed. Installing...");
        if install_brew() {
            log_success("✅ Homebrew installed successfully.");
        } else {
            log_error("❌ Failed to install Homebrew.");
        }
    } else {
        log_success("✅ Homebrew is already installed.");
    }
}

fn is_brew_installed() -> bool {
    Command::new("brew")
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn install_brew() -> bool {
    Command::new("sh")
        .arg("-c")
        .arg(format!("curl -fsSL {} | bash", HOMEBREW_INSTALL_SCRIPT))
        .status()
        .map(|status| status.success())
        .unwrap_or_else(|err| {
            log_error(&format!("❌ Error running install script: {}", err));
            false
        })
}
