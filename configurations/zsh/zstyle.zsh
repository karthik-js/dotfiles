# ~/zstyle.zsh
# ============================
# Zsh Style Configurations
# ============================

# Enable case-insensitive completion (e.g., "Git" == "git")
zstyle ':completion:*' matcher-list 'm:{a-z}={A-Za-z}'

# Automatically show the completion menu when there are multiple matches
zstyle ':completion:*' menu select

# Show descriptions for completions
zstyle ':completion:*' format '%B%d%b'

# Enable colors in completion list (optional if LS_COLORS is set)
zstyle ':completion:*' list-colors ${(s.:.)LS_COLORS}

# Optional: Rehash completions automatically after installing new commands
zstyle ':completion:*' rehash true

# Enable smart case completion
zstyle ':completion:*' case-sensitive false

