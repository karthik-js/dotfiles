mod brew;
mod utils;
mod zsh;

use utils::log_utils::log_info;

fn main() {
    log_info("Initializing your application setup process...");

    zsh::setup::setup_zsh();
    brew::setup::setup_brew();
}
