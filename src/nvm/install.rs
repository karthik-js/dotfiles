use std::process::Command;

pub fn install_nvm() {
    // Check if NVM is already installed
    if Command::new("sh")
        .arg("-c")
        .arg("command -v nvm")
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
    {
        println!("NVM is already installed.");
        return;
    }

    println!("Installing NVM...");

    if !Command::new("sh")
        .arg("-c")
        .arg("curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | bash")
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
    {
        eprintln!("NVM installation failed.");
        return;
    }

    println!("NVM installed successfully.");
}
