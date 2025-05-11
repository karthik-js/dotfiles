use std::path::Path;
use std::process::Command;

use crate::utils::log_utils::{log_error, log_info, log_success};

pub fn install_oh_my_zsh_if_needed() {
    let home_dir = std::env::var("HOME").expect("Could not get HOME directory");
    let omz_dir = format!("{}/.oh-my-zsh", home_dir);

    if Path::new(&omz_dir).exists() {
        log_success("Oh My Zsh is already installed.");
        return;
    }

    log_info("Installing Oh My Zsh...");

    let install_cmd = r#"curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh | sh"#;

    let status = Command::new("sh").arg("-c").arg(install_cmd).status();

    match status {
        Ok(s) if s.success() => log_success("Oh My Zsh installed successfully."),
        Ok(s) => log_error(&format!(
            "Oh My Zsh installer exited with status code: {}",
            s
        )),
        Err(e) => log_error(&format!("Failed to run Oh My Zsh installer: {}", e)),
    }
}
