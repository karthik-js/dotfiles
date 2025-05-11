mod brew;
mod utils;
mod zsh;

use utils::log_utils::log_info;

fn main() {
    log_info("Initializing your application setup process...");

    brew::setup::setup_brew();
    zsh::setup::setup_zsh();
}
