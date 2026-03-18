#!/bin/sh
set -eu

. "$(dirname -- "$0")/common.sh"

tag_name=$(release_tag_name)
project_version=$(release_project_version)
tag_version=$(release_strip_v "$tag_name")
version_match="false"

if release_validate_stable_tag "$tag_name" && [ "$tag_version" = "$project_version" ]; then
  version_match="true"
fi

release_output "TAG" "$tag_name"
release_output "TAG_VERSION" "$tag_version"
release_output "PROJECT_VERSION" "$project_version"
release_output "VERSION_MATCH" "$version_match"
