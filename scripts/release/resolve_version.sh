#!/bin/sh
set -eu

. "$(dirname -- "$0")/common.sh"

if [ -n "${RELEASE_VERSION_INPUT:-}${INPUT_VERSION:-}${RELEASE_VERSION:-}" ]; then
  release_version=$(release_requested_version)
  tag_name=$(release_expected_tag "$release_version")
else
  tag_name=$(release_tag_name)
  release_version=$(release_strip_v "$tag_name")
fi

project_version=$(release_project_version)
version_match="false"

if release_validate_stable_tag "$tag_name" && [ "$release_version" = "$project_version" ]; then
  version_match="true"
fi

release_output "TAG" "$tag_name"
release_output "TAG_VERSION" "$release_version"
release_output "PROJECT_VERSION" "$project_version"
release_output "VERSION_MATCH" "$version_match"
