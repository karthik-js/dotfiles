autoload -U add-zsh-hook

export NVM_PREVIOUS_GIT_ROOT=""

set-project-node() {
  if ! command -v nvm &>/dev/null; then return; fi

  local git_root
  git_root=$(git rev-parse --show-toplevel 2>/dev/null) || return

  if [[ "$git_root" == "$NVM_PREVIOUS_GIT_ROOT" ]]; then return; fi

  local nvmrc_path="$git_root/.nvmrc"
  if [ ! -f "$nvmrc_path" ]; then return; fi

  local nvmrc_node_version
  nvmrc_node_version=$(<"$nvmrc_path")

  local current_node_version
  current_node_version=$(node -v 2>/dev/null | sed 's/^v//')

  if [[ "$nvmrc_node_version" != "$current_node_version" ]]; then
    nvm use --silent "$nvmrc_node_version"
  fi

  export NVM_PREVIOUS_GIT_ROOT="$git_root"
}

add-zsh-hook chpwd set-project-node
set-project-node
