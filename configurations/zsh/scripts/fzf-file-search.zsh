fzf-file-widget() {
    selected_file=$(fzf --preview "cat {}" $(find ~/projects -type f))
    if [[ -n "$selected_file" ]]; then
        if [[ -n "$EDITOR" ]]; then
            $EDITOR "$selected_file"  # Open file with the editor specified in $EDITOR
        else
            echo "No editor specified in the \$EDITOR environment variable."
        fi
    fi
}

bindkey '^F' fzf-file-widget  # Bind to Ctrl+F for custom search
