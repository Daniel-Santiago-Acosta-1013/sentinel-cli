#!/bin/sh
set -eu

. "$(dirname -- "$0")/common.sh"

[ -n "${RELEASE_MANIFEST_PATH:-}" ] || release_fail "RELEASE_MANIFEST_PATH is required"
. "$RELEASE_MANIFEST_PATH"

repo_root=$(release_repo_root)
template_path="$repo_root/packaging/homebrew/sentinel.rb.tpl"
rendered_formula="$ARTIFACT_DIR/sentinel.rb"
archive_url="${RELEASE_ARCHIVE_URL:-https://github.com/${GITHUB_REPOSITORY:-sentinel-cli/sentinel-cli}/releases/download/$RELEASE_TAG/$(basename "$CANONICAL_ARCHIVE")}"
existing_status=$(release_read_state_value "homebrew" STATUS)
existing_version=$(release_read_state_value "homebrew" VERSION)

if [ "$existing_status" = "incompatible" ]; then
  release_fail "incompatible Homebrew publication state already exists"
fi

if [ "$existing_status" = "materialized" ] && [ "$existing_version" = "$RELEASE_VERSION" ]; then
  release_output "STATUS" "materialized"
  release_output "CHANNEL" "homebrew"
  exit 0
fi

sed \
  -e "s|__VERSION__|$RELEASE_VERSION|g" \
  -e "s|__ARCHIVE_URL__|$archive_url|g" \
  -e "s|__SHA256__|$CANONICAL_ARCHIVE_SHA256|g" \
  "$template_path" > "$rendered_formula"

if [ -n "${RELEASE_STATE_DIR:-}" ]; then
  release_write_state "homebrew" \
    "STATUS=materialized" \
    "VERSION=$RELEASE_VERSION" \
    "COMMIT=$SOURCE_COMMIT" \
    "ARTIFACT_REF=$rendered_formula" \
    "DETAILS=Homebrew formula rendered from canonical manifest"
  release_output "STATUS" "published"
  release_output "CHANNEL" "homebrew"
  if [ "${RELEASE_FAIL_HOMEBREW:-0}" = "1" ]; then
    release_write_state "homebrew" \
      "STATUS=failed" \
      "VERSION=$RELEASE_VERSION" \
      "COMMIT=$SOURCE_COMMIT" \
      "ARTIFACT_REF=$rendered_formula" \
      "DETAILS=simulated Homebrew publication failure"
    release_fail "simulated Homebrew publication failure"
  fi
  exit 0
fi

[ -n "${HOMEBREW_TAP_PATH:-}" ] || release_fail "HOMEBREW_TAP_PATH is required for real Homebrew publication"
mkdir -p "$HOMEBREW_TAP_PATH/Formula"
cp "$rendered_formula" "$HOMEBREW_TAP_PATH/Formula/sentinel.rb"
release_output "STATUS" "published"
release_output "CHANNEL" "homebrew"
