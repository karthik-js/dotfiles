#!/bin/bash
# set_dotfiles_path.sh
# This script sets the DOTFILES_PATH environment variable for both the current session
# and ensures it persists across shell sessions by adding it to .zshrc.
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

# Set the path to the configurations directory
CONFIG_DIR="$DOTFILES_DIR/configurations/zsh"

# Define the target .zshrc file
ZSHRC_FILE="$CONFIG_DIR/.zshrc"
echo "Using managed .zshrc at $ZSHRC_FILE"

# We'll use a fixed insertion point (line 4) for consistency
HEADER_END=4

# Create temporary file
TEMP_FILE=$(mktemp)

# Check if DOTFILES_PATH export already exists
if grep -q "export DOTFILES_PATH=" "$ZSHRC_FILE"; then
  echo "Updating DOTFILES_PATH in $ZSHRC_FILE"
else
  echo "Adding DOTFILES_PATH to $ZSHRC_FILE after initial comments"
fi

# First part: Copy the header block (lines 1-4)
head -n $HEADER_END "$ZSHRC_FILE" > "$TEMP_FILE"

# Add an empty line for spacing
echo "" >> "$TEMP_FILE"

# Add the DOTFILES_PATH export
echo "# Set the dotfiles path" >> "$TEMP_FILE"
echo "export DOTFILES_PATH=\"$DOTFILES_DIR\"" >> "$TEMP_FILE"
echo "" >> "$TEMP_FILE"

# Extract the remaining content, excluding any line with DOTFILES_PATH export
tail -n +$((HEADER_END + 1)) "$ZSHRC_FILE" | grep -v "export DOTFILES_PATH=" >> "$TEMP_FILE"

# Replace original file
mv "$TEMP_FILE" "$ZSHRC_FILE"
echo "DOTFILES_PATH has been positioned correctly in $ZSHRC_FILE"

echo "DOTFILES_PATH is now set for current and future shell sessions"

