#!/bin/sh
set -eu

. "$(dirname -- "$0")/common.sh"

rewrite_first_match() {
  file=$1
  matcher=$2
  replacement=$3
  tmp_file=$(mktemp)
  awk -v matcher="$matcher" -v replacement="$replacement" '
    BEGIN { updated = 0 }
    !updated && $0 ~ matcher {
      print replacement
      updated = 1
      next
    }
    { print }
    END {
      if (!updated) {
        exit 9
      }
    }
  ' "$file" > "$tmp_file" || {
    rm -f "$tmp_file"
    release_fail "failed to update version surface in $file"
  }
  mv "$tmp_file" "$file"
}

repo_root=$(release_repo_root)
target_version=$(release_requested_version)
target_tag=$(release_expected_tag "$target_version")
previous_version=$(release_project_version)

release_validate_stable_version "$target_version" || \
  release_fail "release version must be a stable semantic version"

cargo_toml="$repo_root/Cargo.toml"
npm_package_json="$repo_root/packaging/npm/package.json"
homebrew_template="$repo_root/packaging/homebrew/sentinel.rb.tpl"

rewrite_first_match "$cargo_toml" '^version = ' "version = \"$target_version\""
rewrite_first_match "$npm_package_json" '"version"[[:space:]]*:' "  \"version\": \"$target_version\","
rewrite_first_match "$homebrew_template" '^[[:space:]]*version ' "  version \"$target_version\""

release_validate_version_alignment "$target_version"

if ! git -C "$repo_root" rev-parse --git-dir >/dev/null 2>&1; then
  release_fail "release versioning requires a git repository"
fi

if git -C "$repo_root" rev-parse "$target_tag" >/dev/null 2>&1; then
  release_fail "release tag already exists: $target_tag"
fi

git -C "$repo_root" config user.name "${RELEASE_GIT_USER_NAME:-github-actions[bot]}"
git -C "$repo_root" config user.email "${RELEASE_GIT_USER_EMAIL:-41898282+github-actions[bot]@users.noreply.github.com}"
git -C "$repo_root" add Cargo.toml packaging/npm/package.json packaging/homebrew/sentinel.rb.tpl

if git -C "$repo_root" diff --cached --quiet; then
  release_fail "no version surfaces changed for $target_version"
fi

git -C "$repo_root" commit -m "release: $target_tag" >/dev/null
release_commit=$(git -C "$repo_root" rev-parse HEAD)
git -C "$repo_root" tag "$target_tag" "$release_commit"

release_output "PREVIOUS_VERSION" "$previous_version"
release_output "RELEASE_VERSION" "$target_version"
release_output "RELEASE_TAG" "$target_tag"
release_output "RELEASE_COMMIT" "$release_commit"
release_output "UPDATED_FILES" "Cargo.toml,packaging/npm/package.json,packaging/homebrew/sentinel.rb.tpl"
