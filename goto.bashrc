goto() {
    target_dir=$(desktop/goto "$@")
    if [ -d "$target_dir" ]; then
        cd "$target_dir"
    else
        echo "Directory not found: $target_dir"
    fi
}