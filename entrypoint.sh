#!/bin/sh
set -e

# Configure Git credential helper using GitHub PAT
if [ -f "/run/secrets/github_pat" ]; then
  GIT_TOKEN=$(cat /run/secrets/github_pat)
  echo "https://token:${GIT_TOKEN}@github.com" > ~/.git-credentials
  chmod 600 ~/.git-credentials
else
  echo "Warning: No GitHub PAT found at /run/secrets/github_pat"
fi

exec "$@"
