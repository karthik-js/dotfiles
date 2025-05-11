use std::path::Path;
use std::process::Command;

use crate::utils::log_utils::{log_error, log_info};

pub fn run_brew_bundle(brewfile_path: &str) {
    let path = Path::new(brewfile_path);

    // Validate the Brewfile path
    if !path.exists() || !path.is_file() {
        log_error(&format!(
            "🚫 Error: Brewfile not found or invalid at '{}'. Please provide a valid file path.",
            brewfile_path,
        ));
        return;
    }

    // Execute the `brew bundle` command
    match Command::new("brew")
        .args(["bundle", "--file", brewfile_path])
        .status()
    {
        Ok(status) if status.success() => {
            log_info("✅ Brew bundle completed successfully.");
        }
        Ok(status) => {
            log_error(&format!(
                "❌ Brew bundle failed with exit code: {}.",
                status.code().unwrap_or_else(|| -1)
            ));
        }
        Err(e) => {
            log_error(&format!("⚠️ Error: Failed to execute 'brew bundle': {}", e));
        }
    }
}
