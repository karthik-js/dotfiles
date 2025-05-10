#!/bin/zsh

# Check if nvim (Neovim) is installed; if not, fall back to vim or another editor
if command -v nvim &> /dev/null; then
  export EDITOR="nvim"             # Set nvim as default editor
  export VISUAL="nvim"
else
  export EDITOR="vim"              # Fallback to vim if nvim isn't installed
  export VISUAL="vim"
fi

