#!/bin/bash
# set_dotfiles_path.sh
# This script sets the DOTFILES_PATH environment variable for both the current session
# and ensures it persists across shell sessions by adding it to .zshrc if needed.
# Usage: source ./set_dotfiles_path.sh

# Determine the absolute path to the dotfiles directory
# When sourced, $0 isn't reliable, so we use BASH_SOURCE if available
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

# Check if the configuration files directory exists
CONFIG_DIR="$DOTFILES_DIR/configurations/zsh"

# Check which file we should modify - the actual file, not the symlink
if [[ -d "$CONFIG_DIR" && -f "$CONFIG_DIR/.zprofile" ]]; then
  # The .zprofile is managed in the dotfiles repo
  PROFILE_FILE="$CONFIG_DIR/.zprofile"
  echo "Using managed .zprofile at $PROFILE_FILE"
else
  # Fallback to the user's .zprofile
  PROFILE_FILE="$HOME/.zprofile"
  echo "Using user .zprofile at $PROFILE_FILE"
fi

# Check if DOTFILES_PATH export is already in the .zprofile file
if ! grep -q "export DOTFILES_PATH=" "$PROFILE_FILE"; then
  echo "Adding DOTFILES_PATH to $PROFILE_FILE"
  echo "" >> "$PROFILE_FILE"
  echo "# Set the dotfiles path" >> "$PROFILE_FILE"
  echo "export DOTFILES_PATH=\"$DOTFILES_DIR\"" >> "$PROFILE_FILE"
  echo "DOTFILES_PATH has been added to $PROFILE_FILE"
else
  echo "DOTFILES_PATH is already configured in $PROFILE_FILE"
  
  # Update the path if it's different
  CURRENT_PATH=$(grep "export DOTFILES_PATH=" "$PROFILE_FILE" | sed 's/export DOTFILES_PATH="\(.*\)"/\1/')
  if [[ "$CURRENT_PATH" != "$DOTFILES_DIR" ]]; then
    echo "Updating DOTFILES_PATH in $PROFILE_FILE from $CURRENT_PATH to $DOTFILES_DIR"
    sed -i.bak "s|export DOTFILES_PATH=.*|export DOTFILES_PATH=\"$DOTFILES_DIR\"|" "$PROFILE_FILE"
    echo "DOTFILES_PATH has been updated in $PROFILE_FILE"
  fi
fi

echo "DOTFILES_PATH is now set for current and future shell sessions (effective after login)"

