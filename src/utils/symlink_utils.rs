use std::path::Path;

use super::log_utils::{log_error, log_info};

pub fn create_symlink(source: &Path, target: &Path) {
    if target.exists() {
        log_info(&format!("⚠️ Target file already exists: {:?}", target));

        log_info("Backing up existing file...");
        let backup_path = format!("{}.bak", target.display());
        if let Err(e) = std::fs::rename(target, &backup_path) {
            log_error(&format!(
                "⚠️ Failed to back up existing file: {:?} - {}",
                target, e
            ));
            return;
        }
        log_info(&format!("✅ Backed up existing file to: {:?}", backup_path));
        log_info("Removing existing file...");

        if let Err(e) = std::fs::remove_file(target) {
            log_error(&format!(
                "⚠️ Failed to remove existing file: {:?} - {}",
                target, e
            ));
            return;
        }
    }

    log_info(&format!("Creating symlink: {:?} → {:?}", target, source));

    #[cfg(unix)]
    let result = std::os::unix::fs::symlink(source, target);

    #[cfg(windows)]
    let result = {
        if source.is_dir() {
            std::os::windows::fs::symlink_dir(source, target)
        } else {
            std::os::windows::fs::symlink_file(source, target)
        }
    };

    match result {
        Ok(_) => log_info(&format!("✅ Symlink created: {:?} → {:?}", target, source)),
        Err(e) => log_error(&format!(
            "❌ Could not symlink {:?} → {:?}: {}",
            source, target, e
        )),
    }
}
