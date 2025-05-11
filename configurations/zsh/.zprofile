# ============================
# Zsh Profile Configuration
# ============================

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
# Load rustup
# ----------------------------------------
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env"

# ----------------------------------------
# Load Homebrew
# ----------------------------------------
eval "$(/opt/homebrew/bin/brew shellenv)"

# ----------------------------------------
# NVM Configuration
# ----------------------------------------
export NVM_DIR="$([ -z "${XDG_CONFIG_HOME-}" ] && printf %s "${HOME}/.nvm" || printf %s "${XDG_CONFIG_HOME}/nvm")"
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"  # This loads nvm
[ -s "$NVM_DIR/bash_completion" ] && \. "$NVM_DIR/bash_completion"  # This loads nvm bash_completion
