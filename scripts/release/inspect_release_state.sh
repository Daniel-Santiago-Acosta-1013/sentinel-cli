#!/bin/sh
set -eu

. "$(dirname -- "$0")/common.sh"

release_compute_overall_status

release_output "GITHUB_RELEASE_STATUS" "$github_status"
release_output "NPM_STATUS" "$npm_status"
release_output "HOMEBREW_STATUS" "$homebrew_status"
release_output "OVERALL_STATUS" "$OVERALL_STATUS"
release_output "NEXT_SAFE_ACTION" "$NEXT_SAFE_ACTION"
