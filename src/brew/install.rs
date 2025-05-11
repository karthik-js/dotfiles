use crate::utils::log_utils::{log_error, log_info, log_success};
use std::process::Command;

pub fn ensure_brew_installed() -> bool {
    log_info("🔍 Checking for Homebrew installation...");
    if !is_brew_installed() {
        log_info("ℹ️ Homebrew is not installed.");
        log_info("Homebrew installation requires admin permissions and should be done manually.");
        log_info("\nTo install Homebrew, please run the following command in your terminal:");
        log_info(
            "  /bin/bash -c \"$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\"",
        );
        log_info("\nAfter installing Homebrew, please run init.sh again to continue setup.");
        false
    } else {
        log_success("✅ Homebrew is already installed.");
        true
    }
}

fn is_brew_installed() -> bool {
    match Command::new("brew").arg("--version").output() {
        Ok(output) => {
            if output.status.success() {
                let version = String::from_utf8_lossy(&output.stdout);
                log_info(&format!("🔍 Homebrew version: {}", version.trim()));
                true
            } else {
                log_error(&format!(
                    "❌ Homebrew check failed with error: {}",
                    String::from_utf8_lossy(&output.stderr).trim()
                ));
                false
            }
        }
        Err(err) => {
            log_error(&format!("❌ Failed to execute brew command: {}", err));
            false
        }
    }
}
