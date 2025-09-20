#!/bin/sh
# Docker operations and required docker-socket-proxy permissions:
#
# docker compose build:
#   - BUILD=1        # Allow building images
#   - IMAGES=1       # Allow listing and managing images
#   - NETWORKS=1     # Allow network operations during build
#   - VOLUMES=1      # Allow volume operations during build
#
# docker compose down:
#   - CONTAINERS=1   # Allow stopping and removing containers
#   - NETWORKS=1     # Allow removing networks
#   - VOLUMES=1      # Allow removing volumes
#   - SERVICES=1     # Allow compose service management
#
# docker compose up -d:
#   - CONTAINERS=1   # Allow creating and starting containers
#   - IMAGES=1       # Allow pulling images
#   - NETWORKS=1     # Allow creating networks
#   - VOLUMES=1      # Allow creating volumes
#   - SERVICES=1     # Allow compose service management
#   - POST=1         # Allow POST requests (create operations)
#
# docker builder prune -f:
#   - BUILDER=1      # Allow managing builder and build cache
#
# docker image prune -f:
#   - IMAGES=1       # Allow listing and deleting images
#   - POST=1         # Allow POST requests (delete operations)
set -ex

commit_id=$1

# # Put your build and deployment commands here.
# # Example of the docker deployment:
# cd /path/to/repo
# git fetch
# git checkout $commit_id
# docker compose --progress plain build
# docker compose up -d --force-recreate
# docker builder prune -f
# docker image prune -f
