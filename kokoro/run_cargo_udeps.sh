#!/usr/bin/env bash

# shellcheck source=./kokoro/common.sh
source "$(dirname "$0")/common.sh"

./scripts/docker_pull
./scripts/docker_run nix develop .#ci --command just cargo-udeps
./scripts/git_check_diff

kokoro_cleanup
