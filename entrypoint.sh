#!/bin/sh
set -e

echo "Setting up Git credentials..."

# Configure Git credential helper using GitHub PAT
if [ -f "/run/secrets/github_pat" ]; then
    GIT_TOKEN=$(cat /run/secrets/github_pat)

    # Set credential store
    git config --global credential.helper store

    # Create credentials file (no username needed, GitHub accepts any value)
    echo "https://token:${GIT_TOKEN}@github.com" > ~/.git-credentials
    chmod 600 ~/.git-credentials

    # Convert SSH URLs to HTTPS
    git config --global url."https://github.com/".insteadOf "ssh://git@github.com/"
    git config --global url."https://github.com/".insteadOf "git@github.com:"
    git config --global url."https://github.com/".insteadOf "git+ssh://git@github.com/"

    echo "GitHub PAT configured for repository access"
else
    echo "Warning: No GitHub PAT found at /run/secrets/github_pat"
fi

echo "Starting hooked..."
exec "$@"
