#!/bin/bash
# set_dotfiles_path.sh
# This script sets the DOTFILES_PATH environment variable for both the current session
# and adds value to .zshenv file that stores environment variables.
# Usage: source ./set_dotfiles_path.sh

# Determine the absolute path to the dotfiles directory
if [[ -n "${BASH_SOURCE[0]}" ]]; then
  SCRIPT_PATH="${BASH_SOURCE[0]}"
else
  # Fallback method if BASH_SOURCE is not available
  SCRIPT_PATH="$0"
fi

# Get the absolute path of the dotfiles directory
DOTFILES_DIR="$(cd "$(dirname "$SCRIPT_PATH")" && pwd)"

# Export the variable for the current session
export DOTFILES_PATH="$DOTFILES_DIR"
echo "DOTFILES_PATH is set to: $DOTFILES_PATH for the current session"

# Define the environment file path
ENV_FILE="$HOME/.zshenv"

echo "Checking for existing .zshenv file at $ENV_FILE"
if [ ! -f "$ENV_FILE" ]; then
  echo "Creating new .zshenv file at $ENV_FILE"
  touch "$ENV_FILE"
fi

# Create or update the environment file
echo "Managing environment file at $ENV_FILE"

# Check if DOTFILES_PATH is already set in the environment file
if grep -q "^export DOTFILES_PATH=" "$ENV_FILE"; then
  # Update the existing DOTFILES_PATH value
  if [[ "$OSTYPE" == "darwin"* ]]; then
    sed -i '' "s|^export DOTFILES_PATH=.*|export DOTFILES_PATH=\"$DOTFILES_DIR\"|" "$ENV_FILE"
  else
    sed -i "s|^export DOTFILES_PATH=.*|export DOTFILES_PATH=\"$DOTFILES_DIR\"|" "$ENV_FILE"
  fi
  echo "Updated existing DOTFILES_PATH in $ENV_FILE"
else
  # Append the DOTFILES_PATH configuration
  cat >> "$ENV_FILE" << EOL
    # DOTFILES path configuration
    export DOTFILES_PATH="$DOTFILES_DIR"
EOL

  echo "Appended DOTFILES_PATH to $ENV_FILE"
fi

# Set proper permissions
chmod 644 "$ENV_FILE"

echo "Updated $ENV_FILE with DOTFILES_PATH=$DOTFILES_DIR"
echo "DOTFILES_PATH is now set for current and future shell sessions"

