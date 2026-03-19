#!/bin/sh
set -eu

. "$(dirname -- "$0")/common.sh"

[ -n "${RELEASE_MANIFEST_PATH:-}" ] || release_fail "RELEASE_MANIFEST_PATH is required"
. "$RELEASE_MANIFEST_PATH"

repo_root=$(release_repo_root)
template_dir="$repo_root/packaging/npm"
npm_docs_dir="$repo_root/docs/npm"
stage_dir="$ARTIFACT_DIR/npm-package"
package_name=$(release_parse_template_value "$template_dir/package.json" "name")
existing_status=$(release_read_state_value "npm" STATUS)
existing_version=$(release_read_state_value "npm" VERSION)

if [ "$existing_status" = "incompatible" ]; then
  release_fail "incompatible npm publication state already exists"
fi

if [ "$existing_status" = "materialized" ] && [ "$existing_version" = "$RELEASE_VERSION" ]; then
  release_output "STATUS" "materialized"
  release_output "CHANNEL" "npm"
  exit 0
fi

rm -rf "$stage_dir"
mkdir -p "$stage_dir/bin"
cp "$npm_docs_dir/README.md" "$stage_dir/README.md"
cp "$repo_root/LICENSE" "$stage_dir/LICENSE"
cp "$template_dir/.npmignore" "$stage_dir/.npmignore"
sed "s/__VERSION__/$RELEASE_VERSION/g" "$template_dir/package.json" > "$stage_dir/package.json"
tar -xzf "$CANONICAL_ARCHIVE" -C "$stage_dir/bin"
chmod +x "$stage_dir/bin/sentinel"

if [ -n "${RELEASE_STATE_DIR:-}" ]; then
  release_write_state "npm" \
    "STATUS=materialized" \
    "VERSION=$RELEASE_VERSION" \
    "COMMIT=$SOURCE_COMMIT" \
    "ARTIFACT_REF=$CANONICAL_ARCHIVE" \
    "DETAILS=npm package materialized from canonical archive"
  release_output "STATUS" "published"
  release_output "CHANNEL" "npm"
  if [ "${RELEASE_FAIL_AFTER_NPM:-0}" = "1" ]; then
    release_fail "simulated npm publication failure"
  fi
  exit 0
fi

if ! command -v npm >/dev/null 2>&1; then
  release_fail "npm CLI is required for real npm publication"
fi

if npm view "$package_name@$RELEASE_VERSION" version >/dev/null 2>&1; then
  release_output "STATUS" "materialized"
  release_output "CHANNEL" "npm"
  exit 0
fi

(cd "$stage_dir" && npm publish --provenance --access public)
release_output "STATUS" "published"
release_output "CHANNEL" "npm"
