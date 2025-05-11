# ============================
# Zsh Configuration File
# ============================

# ----------------------------------------
# Load starship prompt
# ----------------------------------------
eval "$(starship init zsh)"

# ----------------------------------------
# History Configuration
# ----------------------------------------

HISTFILE=~/.zsh_history             # File where command history is saved
HISTSIZE=10000                      # Number of commands to remember in memory
SAVEHIST=10000                      # Number of commands to save in the history file
setopt HIST_IGNORE_ALL_DUPS        # Avoid duplicate entries in history
setopt HIST_REDUCE_BLANKS          # Strip unnecessary spaces
setopt SHARE_HISTORY                # Share history across all sessions

# ----------------------------------------
# Zsh theme
# ----------------------------------------
# ZSH_THEME="fino-time"

# ----------------------------------------
# Prompt (Pure)
# ----------------------------------------

autoload -Uz promptinit && promptinit
prompt pure                           # Use the 'pure' minimal prompt (install via `brew install pure`)

# Enable completion system
autoload -Uz compinit && compinit

# Enable fzf completion
[ -f $(brew --prefix)/opt/fzf/shell/completion.zsh ] && source $(brew --prefix)/opt/fzf/shell/completion.zsh

# Enable fzf key bindings
[ -f $(brew --prefix)/opt/fzf/shell/key-bindings.zsh ] && source $(brew --prefix)/opt/fzf/shell/key-bindings.zsh

# ============================
# Source all scripts from the scripts directory
# ============================

for script in "$DOTFILES/scripts/"*.zsh; do
  [[ -f $script ]] && source "$script"
done

# ============================
# Source all alias files from the aliases directory
# ============================

for alias_file in "$DOTFILES/aliases/"*.zsh; do
  [[ -f $alias_file ]] && source "$alias_file"
done

# ----------------------------------------
# Shell Behavior Tweaks
# ----------------------------------------

setopt AUTO_CD                     # Automatically change directory when typing `dirname`
setopt AUTO_PUSHD                  # Push old dir to directory stack on `cd`
setopt PUSHD_IGNORE_DUPS           # Don’t store duplicates in pushd stack
setopt CORRECT                     # Spell correction for commands
setopt EXTENDED_GLOB               # Extended globbing for filename matching
setopt NO_CASE_GLOB                # Case-insensitive globbing

# ----------------------------------------
# Source zstyle configuration file
# ----------------------------------------

[[ -f "$DOTFILES/zstyle.zsh" ]] && source "$DOTFILES/zstyle.zsh"

export NVM_DIR="$HOME/.nvm"
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"  # This loads nvm
[ -s "$NVM_DIR/bash_completion" ] && \. "$NVM_DIR/bash_completion"  # This loads nvm bash_completion
