mod brew;
mod nvm;
mod utils;
mod zsh;

use utils::log_utils::log_info;

fn main() {
    log_info("Initializing your application setup process...");

    nvm::install::install_nvm();
    zsh::setup::setup_zsh();
    // setup brew can be blocking operation so moving it down to not interrupt non blocking operations
    brew::setup::setup_brew();
}
