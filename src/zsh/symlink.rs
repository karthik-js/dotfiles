use crate::utils::log_utils::log_error;
use crate::utils::symlink_utils::create_symlink;
use std::path::PathBuf;

pub fn symlink_zsh_configs() {
    let home_dir = std::env::var("HOME").expect("Could not get HOME directory");

    if !PathBuf::from(&home_dir).exists() {
        log_error("Home directory does not exist.");
        return;
    }

    let zshrc_source = PathBuf::from("configurations/zsh/.zshrc");
    let zprofile_source = PathBuf::from("configurations/zsh/.zprofile");

    let zshrc_target = PathBuf::from(format!("{}/.zshrc", home_dir));
    let zprofile_target = PathBuf::from(format!("{}/.zprofile", home_dir));

    create_symlink(&zshrc_source, &zshrc_target);
    create_symlink(&zprofile_source, &zprofile_target);
}
