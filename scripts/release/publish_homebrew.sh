#!/bin/sh
set -eu

. "$(dirname -- "$0")/common.sh"

[ -n "${RELEASE_MANIFEST_PATH:-}" ] || release_fail "RELEASE_MANIFEST_PATH is required"
. "$RELEASE_MANIFEST_PATH"
release_validate_version_alignment "$RELEASE_VERSION"

repo_root=$(release_repo_root)
template_path="$repo_root/packaging/homebrew/sentinel.rb.tpl"
rendered_formula="$ARTIFACT_DIR/sentinel.rb"
archive_url="${RELEASE_ARCHIVE_URL:-https://github.com/${GITHUB_REPOSITORY:-Daniel-Santiago-Acosta-1013/sentinel-cli}/releases/download/$RELEASE_TAG/$(basename "$CANONICAL_ARCHIVE")}"
template_version=$(release_homebrew_template_version)
existing_status=$(release_read_state_value "homebrew" STATUS)
existing_version=$(release_read_state_value "homebrew" VERSION)

[ "$template_version" = "$RELEASE_VERSION" ] || \
  release_fail "Homebrew formula template version does not match release manifest"

if [ "$existing_status" = "incompatible" ]; then
  release_fail "incompatible Homebrew publication state already exists"
fi

if [ "$existing_status" = "materialized" ] && [ "$existing_version" = "$RELEASE_VERSION" ]; then
  release_output "STATUS" "materialized"
  release_output "CHANNEL" "homebrew"
  exit 0
fi

sed \
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
[ -n "${HOMEBREW_TAP_REPO:-}" ] || release_fail "HOMEBREW_TAP_REPO is required for real Homebrew publication"
[ -n "${HOMEBREW_TAP_TOKEN:-}" ] || release_fail "HOMEBREW_TAP_TOKEN is required for real Homebrew publication"

clone_url="https://x-access-token:${HOMEBREW_TAP_TOKEN}@github.com/${HOMEBREW_TAP_REPO}.git"
formula_path="$HOMEBREW_TAP_PATH/Formula/sentinel.rb"

rm -rf "$HOMEBREW_TAP_PATH"
git clone --depth 1 "$clone_url" "$HOMEBREW_TAP_PATH"
mkdir -p "$HOMEBREW_TAP_PATH/Formula"

if [ -f "$formula_path" ] && cmp -s "$rendered_formula" "$formula_path"; then
  release_output "STATUS" "materialized"
  release_output "CHANNEL" "homebrew"
  exit 0
fi

cp "$rendered_formula" "$formula_path"
git -C "$HOMEBREW_TAP_PATH" config user.name "github-actions[bot]"
git -C "$HOMEBREW_TAP_PATH" config user.email "41898282+github-actions[bot]@users.noreply.github.com"
git -C "$HOMEBREW_TAP_PATH" add Formula/sentinel.rb

if git -C "$HOMEBREW_TAP_PATH" diff --cached --quiet; then
  release_output "STATUS" "materialized"
  release_output "CHANNEL" "homebrew"
  exit 0
fi

git -C "$HOMEBREW_TAP_PATH" commit -m "sentinel ${RELEASE_VERSION}"
git -C "$HOMEBREW_TAP_PATH" push origin HEAD:main

release_output "STATUS" "published"
release_output "CHANNEL" "homebrew"
