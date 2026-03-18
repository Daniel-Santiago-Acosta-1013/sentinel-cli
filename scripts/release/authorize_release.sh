#!/bin/sh
set -eu

. "$(dirname -- "$0")/common.sh"

tag_name=$(release_tag_name)
project_version=$(release_project_version)
tag_version=""
tag_commit=""
main_head=""
authorized="false"
block_reason=""

if ! release_validate_stable_tag "$tag_name"; then
  block_reason="invalid_tag_format"
else
  tag_version=$(release_strip_v "$tag_name")
  tag_commit=$(release_resolve_tag_commit "$tag_name")
  main_head=$(release_resolve_main_head)

  if [ -z "$tag_commit" ]; then
    block_reason="unable_to_resolve_tag_commit"
  elif [ -z "$main_head" ]; then
    block_reason="unable_to_resolve_main_head"
  elif [ "$tag_version" != "$project_version" ]; then
    block_reason="tag_version_mismatch"
  elif [ "$tag_commit" != "$main_head" ]; then
    block_reason="tag_not_at_main_head"
  else
    authorized="true"
    block_reason="authorized"
  fi
fi

release_output "TAG" "$tag_name"
release_output "TAG_VERSION" "${tag_version:-$(release_strip_v "$tag_name")}"
release_output "PROJECT_VERSION" "$project_version"
release_output "TAG_COMMIT" "$tag_commit"
release_output "MAIN_HEAD" "$main_head"
release_output "AUTHORIZED" "$authorized"
release_output "BLOCK_REASON" "$block_reason"
