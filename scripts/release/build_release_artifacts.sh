#!/bin/sh
set -eu

. "$(dirname -- "$0")/common.sh"

tag_name=$(release_tag_name)
version=$(release_strip_v "$tag_name")
project_version=$(release_project_version)
tag_commit=$(release_resolve_tag_commit "$tag_name")
artifact_dir=$(release_artifact_dir)
bundle_dir="$artifact_dir/bundle"
binary_path="${RELEASE_BINARY_PATH:-}"
archive_path="$artifact_dir/sentinel-$version.tar.gz"
checksum_file="$artifact_dir/SHASUMS256.txt"
manifest_path=$(release_manifest_path "$artifact_dir")

[ "$version" = "$project_version" ] || release_fail "tag version does not match Cargo.toml version"
[ -n "$tag_commit" ] || release_fail "unable to resolve release tag commit"

mkdir -p "$artifact_dir" "$bundle_dir"

if [ -z "$binary_path" ]; then
  if [ "${RELEASE_USE_MOCK_BUILD:-0}" = "1" ]; then
    binary_path="$artifact_dir/mock-sentinel"
    cat > "$binary_path" <<EOF
#!/bin/sh
if [ "\${SENTINEL_INTERNAL_MODE:-}" = "print-version" ]; then
  echo "$version"
  exit 0
fi
echo "Sentinel $version"
EOF
    chmod +x "$binary_path"
  else
    repo_root=$(release_repo_root)
    cargo build --release --manifest-path "$repo_root/Cargo.toml"
    binary_path="$repo_root/target/release/sentinel"
  fi
fi

[ -x "$binary_path" ] || release_fail "release binary is not executable: $binary_path"

cp "$binary_path" "$bundle_dir/sentinel"
chmod +x "$bundle_dir/sentinel"
tar -czf "$archive_path" -C "$bundle_dir" sentinel
archive_sha256=$(shasum -a 256 "$archive_path" | awk '{ print $1 }')
printf '%s  %s\n' "$archive_sha256" "$(basename "$archive_path")" > "$checksum_file"

cat > "$manifest_path" <<EOF
RELEASE_TAG=$tag_name
RELEASE_VERSION=$version
PROJECT_VERSION=$project_version
SOURCE_COMMIT=$tag_commit
ARTIFACT_DIR=$artifact_dir
CANONICAL_ARCHIVE=$archive_path
CANONICAL_ARCHIVE_SHA256=$archive_sha256
CHECKSUM_FILE=$checksum_file
EOF

release_output "ARTIFACT_DIR" "$artifact_dir"
release_output "MANIFEST_PATH" "$manifest_path"
release_output "CANONICAL_ARCHIVE" "$archive_path"
release_output "CANONICAL_ARCHIVE_SHA256" "$archive_sha256"
release_output "RELEASE_VERSION" "$version"
release_output "SOURCE_COMMIT" "$tag_commit"
