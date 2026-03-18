#!/bin/sh
set -eu

. "$(dirname -- "$0")/common.sh"

[ -n "${RELEASE_MANIFEST_PATH:-}" ] || release_fail "RELEASE_MANIFEST_PATH is required"
. "$RELEASE_MANIFEST_PATH"

existing_status=$(release_read_state_value "github-release" STATUS)
existing_version=$(release_read_state_value "github-release" VERSION)
existing_commit=$(release_read_state_value "github-release" COMMIT)

if [ "$existing_status" = "incompatible" ]; then
  release_fail "incompatible GitHub release state already exists"
fi

if [ "$existing_status" = "materialized" ] && [ "$existing_version" = "$RELEASE_VERSION" ] && [ "$existing_commit" = "$SOURCE_COMMIT" ]; then
  release_output "STATUS" "materialized"
  release_output "CHANNEL" "github-release"
  exit 0
fi

if [ -n "${RELEASE_STATE_DIR:-}" ]; then
  release_write_state "github-release" \
    "STATUS=materialized" \
    "VERSION=$RELEASE_VERSION" \
    "COMMIT=$SOURCE_COMMIT" \
    "ARTIFACT_REF=$CANONICAL_ARCHIVE" \
    "DETAILS=canonical GitHub release materialized"
  release_output "STATUS" "published"
  release_output "CHANNEL" "github-release"
  if [ "${RELEASE_FAIL_AFTER_GITHUB:-0}" = "1" ]; then
    release_fail "simulated GitHub release publication failure"
  fi
  exit 0
fi

if ! command -v gh >/dev/null 2>&1; then
  release_fail "gh CLI is required for real GitHub release publication"
fi

if gh release view "$RELEASE_TAG" >/dev/null 2>&1; then
  release_output "STATUS" "materialized"
  release_output "CHANNEL" "github-release"
  exit 0
fi

gh release create "$RELEASE_TAG" "$CANONICAL_ARCHIVE" "$CHECKSUM_FILE" --title "$RELEASE_TAG" --notes "Sentinel $RELEASE_VERSION"
release_output "STATUS" "published"
release_output "CHANNEL" "github-release"
