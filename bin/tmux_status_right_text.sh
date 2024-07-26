#!/bin/bash

# Get the current branch name
get_git_branch() {
    local branch
    branch=$(git rev-parse --abbrev-ref HEAD 2>/dev/null)
    if [ -n "$branch" ]; then
        echo "$branch"
    else
        echo ""
    fi
}

echo $1

# Get the time since the current session started
pause=$(/Users/carlorosso/.config/programmini/studio2/target/release/studio2 --pause "$1")

echo "$(get_git_branch) | $pause "
