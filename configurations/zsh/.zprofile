# ============================
# Zsh Profile Configuration
# ============================

# Set the dotfiles base path (if using a custom dotfiles repo)
export DOTFILES="$HOME/dotfiles/zsh"

# ----------------------------------------
# Key Environment Variables
# ----------------------------------------

export PAGER="less"                 # Use `less` for paging
export LESS="-FRX"                  # Less options for smooth scrolling
export PATH="$HOME/bin:$PATH"       # User-specific bin directory

# ----------------------------------------
# FZF Configuration
# ----------------------------------------
export FZF_DEFAULT_OPTS="--height 40% --reverse --border"
export FZF_DEFAULT_COMMAND='rg --files'

# ----------------------------------------
# NVM Configuration
# ----------------------------------------
export NVM_DIR="$([ -z "${XDG_CONFIG_HOME-}" ] && printf %s "${HOME}/.nvm" || printf %s "${XDG_CONFIG_HOME}/nvm")"
