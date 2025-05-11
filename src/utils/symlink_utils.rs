use std::fs;
use std::io::ErrorKind;
use std::path::Path;

use super::log_utils::{log_error, log_info, log_success, log_warning};

pub fn create_symlink(source: &Path, target: &Path) {
    if let Ok(metadata) = fs::symlink_metadata(target) {
        if metadata.file_type().is_symlink() {
            log_warning(&format!("⚠️ Target is a symlink: {:?}", target));
            log_info("Removing existing symlink...");

            if let Err(e) = fs::remove_file(target) {
                log_error(&format!(
                    "❌ Failed to remove existing symlink: {:?} - {}",
                    target, e
                ));
                return;
            }
            log_success(&format!("✅ Removed existing symlink: {:?}", target));
        } else {
            log_warning(&format!("⚠️ Target file already exists: {:?}", target));

            log_info("Backing up existing file...");
            let backup_path = format!("{}.bak", target.display());
            if let Err(e) = fs::rename(target, &backup_path) {
                log_error(&format!(
                    "❌ Failed to back up existing file: {:?} - {}",
                    target, e
                ));
                return;
            }
            log_info(&format!("✅ Backed up existing file to: {:?}", backup_path));
        }
    } else if let Err(e) = fs::symlink_metadata(target) {
        if e.kind() != ErrorKind::NotFound {
            log_error(&format!(
                "❌ Failed to check target metadata: {:?} - {}",
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
        Ok(_) => log_success(&format!("✅ Symlink created: {:?} → {:?}", target, source)),
        Err(e) => log_error(&format!(
            "❌ Could not symlink {:?} → {:?}: {}",
            source, target, e
        )),
    }
}
