# Dotfiles Setup

This project automates the setup of a new system with essential tools and configurations, such as Zsh, Homebrew, NVM, Neovim, and more. It ensures a consistent development environment across machines.

## Features

- **Zsh Configuration**:
   - Installs Oh My Zsh if not already installed.
   - Symlinks `.zshrc` and `.zprofile` from the `configurations/zsh` directory.
   - Sources additional scripts and aliases for enhanced functionality.

- **Homebrew**:
   - Ensures Homebrew is installed.
   - Installs packages listed in the `configurations/brew/Brewfile`.

- **Utilities**:
   - Cross-platform symlink creation with backup support.
   - Logging utilities for clear feedback during the setup process.
   - Spinner animations for a better user experience.

## Project Structure

```
.
├── Cargo.toml                # Rust project configuration
├── configurations/           # Configuration files for tools
│   ├── brew/
│   │   └── Brewfile          # List of Homebrew packages to install
│   └── zsh/
│       ├── .zshrc            # Zsh configuration file
│       ├── .zprofile         # Zsh profile file
│       ├── zstyle.zsh        # Zsh style configuration
│       └── scripts/          # Additional Zsh scripts
├── src/                      # Rust source code
│   ├── main.rs               # Entry point for the application
│   ├── brew/                 # Homebrew-related functionality
│   ├── utils/                # Utility modules (logging, symlinks, etc.)
│   └── zsh/                  # Zsh-related functionality
└── target/                   # Build artifacts
```

## Prerequisites

- Rust (for running this project)
- Git (for cloning repositories)
- Curl (for downloading scripts)

## Usage

### Method 1: Clone and Run

1. Clone this repository:
    ```bash
    git clone https://github.com/karthik-js/dotfiles.git
    cd dotfiles
    ```

2. Make the `init.sh` script executable:
    ```bash
    chmod +x init.sh
    ```

3. Run the `init.sh` script:
    ```bash
    ./init.sh
    ```

4. Reload your Zsh configuration:
    ```bash
    source ~/.zshrc
    ```

### Method 2: Fork, Clone, and Run

1. Fork this repository to your GitHub account.

2. Clone your forked repository:
    ```bash
    git clone https://github.com/your-username/dotfiles.git
    cd dotfiles
    ```

3. Make the `init.sh` script executable:
    ```bash
    chmod +x init.sh
    ```

4. Run the `init.sh` script:
    ```bash
    ./init.sh
    ```

5. Reload your Zsh configuration:
    ```bash
    source ~/.zshrc
    ```

## Customization

- **Zsh Configurations**: Modify files in the `configurations/zsh` directory to customize your Zsh setup.
- **Homebrew Packages**: Add or remove packages in the `configurations/brew/Brewfile`.

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.

## License

This project is licensed under the [MIT License](LICENSE).