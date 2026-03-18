#!/bin/sh
set -eu

. "$(dirname -- "$0")/common.sh"

tag_name=$(release_tag_name)
tag_commit="${RELEASE_TAG_COMMIT:-$(release_resolve_tag_commit "$tag_name")}"
main_head="${RELEASE_MAIN_HEAD:-$(release_resolve_main_head)}"
release_version="${RELEASE_VERSION:-$(release_strip_v "$tag_name")}"
block_reason="${BLOCK_REASON:-}"

if [ -n "${RELEASE_STATE_DIR:-}" ]; then
  release_compute_overall_status
else
  if [ "${AUTHORIZED:-}" = "true" ]; then
    OVERALL_STATUS="completed"
  else
    OVERALL_STATUS="blocked"
  fi
  NEXT_SAFE_ACTION=$(release_next_safe_action "$OVERALL_STATUS")
fi

release_output "TAG" "$tag_name"
release_output "AUTHORIZED_COMMIT" "$tag_commit"
release_output "MAIN_HEAD" "$main_head"
release_output "RELEASE_VERSION" "$release_version"
release_output "GLOBAL_STATUS" "$OVERALL_STATUS"
release_output "BLOCK_REASON" "$block_reason"
release_output "GITHUB_RELEASE_STATUS" "${github_status:-absent}"
release_output "NPM_STATUS" "${npm_status:-absent}"
release_output "HOMEBREW_STATUS" "${homebrew_status:-absent}"
release_output "NEXT_SAFE_ACTION" "$NEXT_SAFE_ACTION"
