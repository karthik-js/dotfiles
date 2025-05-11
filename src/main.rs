mod brew;
mod nvm;
mod utils;
mod zsh;

use utils::log_utils::log_info;

fn main() {
    log_info("Initializing your application setup process...");

    brew::setup::setup_brew();
    nvm::install::install_nvm();
    zsh::setup::setup_zsh();
}
