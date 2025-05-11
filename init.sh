#!/bin/bash

# Function to check if a command exists
command_exists() {
    command -v "$1" &> /dev/null
}

# Step 1: Check if Rust is installed
if command_exists rustc; then
    echo "Rust is already installed."
else
    echo "Rust is not installed. Installing Rust..."

    # Install Rust using rustup (Rust's official installer)
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        # Linux
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        # macOS
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    else
        echo "Unsupported OS type. Please install Rust manually."
        exit 1
    fi

    echo "Rust has been installed."
    # Reload shell to ensure `rustc` is available
    source $HOME/.cargo/env
fi

# Step 2: Run the project
echo "Running the project..."
cargo run
