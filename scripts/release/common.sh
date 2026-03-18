#!/bin/sh
set -eu

release_script_dir() {
  CDPATH= cd -- "$(dirname -- "$0")" && pwd
}

release_repo_root() {
  script_dir=$(release_script_dir)
  CDPATH= cd -- "$script_dir/../.." && pwd
}

release_output() {
  key=$1
  value=$2
  printf '%s=%s\n' "$key" "$value"
  if [ -n "${GITHUB_OUTPUT:-}" ]; then
    printf '%s=%s\n' "$key" "$value" >> "$GITHUB_OUTPUT"
  fi
  if [ -n "${RELEASE_OUTPUT_FILE:-}" ]; then
    printf '%s=%s\n' "$key" "$value" >> "$RELEASE_OUTPUT_FILE"
  fi
}

release_fail() {
  printf '%s\n' "$1" >&2
  exit 1
}

release_project_version() {
  repo_root=$(release_repo_root)
  awk -F'"' '/^version = / { print $2; exit }' "$repo_root/Cargo.toml"
}

release_tag_name() {
  if [ -n "${RELEASE_TAG:-}" ]; then
    printf '%s' "$RELEASE_TAG"
    return
  fi
  if [ -n "${GITHUB_REF_NAME:-}" ]; then
    printf '%s' "$GITHUB_REF_NAME"
    return
  fi
  release_fail "RELEASE_TAG or GITHUB_REF_NAME is required"
}

release_validate_stable_tag() {
  printf '%s' "$1" | grep -Eq '^v[0-9]+\.[0-9]+\.[0-9]+$'
}

release_strip_v() {
  printf '%s' "${1#v}"
}

release_resolve_tag_commit() {
  tag_name=$1
  if [ -n "${RELEASE_TAG_COMMIT:-}" ]; then
    printf '%s' "$RELEASE_TAG_COMMIT"
    return
  fi
  repo_root=$(release_repo_root)
  git -C "$repo_root" rev-list -n 1 "$tag_name" 2>/dev/null || true
}

release_resolve_main_head() {
  if [ -n "${RELEASE_MAIN_HEAD:-}" ]; then
    printf '%s' "$RELEASE_MAIN_HEAD"
    return
  fi
  repo_root=$(release_repo_root)
  git -C "$repo_root" rev-parse refs/remotes/origin/main 2>/dev/null || \
    git -C "$repo_root" rev-parse refs/heads/main 2>/dev/null || \
    git -C "$repo_root" ls-remote origin refs/heads/main 2>/dev/null | awk 'NR == 1 { print $1 }' || true
}

release_artifact_dir() {
  if [ -n "${RELEASE_ARTIFACT_DIR:-}" ]; then
    printf '%s' "$RELEASE_ARTIFACT_DIR"
    return
  fi
  repo_root=$(release_repo_root)
  version=$(release_strip_v "$(release_tag_name)")
  printf '%s/.release-artifacts/%s' "$repo_root" "$version"
}

release_manifest_path() {
  artifact_dir=$1
  printf '%s/release-manifest.env' "$artifact_dir"
}

release_state_file() {
  channel=$1
  [ -n "${RELEASE_STATE_DIR:-}" ] || return 1
  printf '%s/%s.env' "$RELEASE_STATE_DIR" "$channel"
}

release_write_state() {
  channel=$1
  shift
  [ -n "${RELEASE_STATE_DIR:-}" ] || release_fail "RELEASE_STATE_DIR is required for mocked publication state"
  mkdir -p "$RELEASE_STATE_DIR"
  file=$(release_state_file "$channel")
  : > "$file"
  for line in "$@"; do
    printf '%s\n' "$line" >> "$file"
  done
}

release_read_state_value() {
  channel=$1
  key=$2
  file=$(release_state_file "$channel" 2>/dev/null || true)
  if [ -z "$file" ] || [ ! -f "$file" ]; then
    return 0
  fi
  awk -F= -v needle="$key" '$1 == needle { print substr($0, index($0, "=") + 1); exit }' "$file"
}

release_load_state() {
  channel=$1
  prefix=$2
  status=$(release_read_state_value "$channel" STATUS)
  version=$(release_read_state_value "$channel" VERSION)
  commit=$(release_read_state_value "$channel" COMMIT)
  artifact_ref=$(release_read_state_value "$channel" ARTIFACT_REF)
  details=$(release_read_state_value "$channel" DETAILS)

  if [ -z "$status" ]; then
    status="absent"
  fi

  eval "${prefix}_status=\$status"
  eval "${prefix}_version=\$version"
  eval "${prefix}_commit=\$commit"
  eval "${prefix}_artifact_ref=\$artifact_ref"
  eval "${prefix}_details=\$details"
}

release_next_safe_action() {
  status=$1
  case "$status" in
    blocked)
      printf '%s' "inspect tag, main HEAD, and version alignment before creating a new stable tag"
      ;;
    partial)
      printf '%s' "inspect channel states and resume only the missing or failed publication"
      ;;
    materialized)
      printf '%s' "verify external channels and close the release without republishing"
      ;;
    completed)
      printf '%s' "announce the release and retain the manifest and checksums for audit"
      ;;
    *)
      printf '%s' "inspect the release evidence and decide the next safe action"
      ;;
  esac
}

release_compute_overall_status() {
  release_load_state "github-release" github
  release_load_state "npm" npm
  release_load_state "homebrew" homebrew

  materialized_count=0
  incompatible_count=0
  failed_count=0
  pending_count=0

  for status in "$github_status" "$npm_status" "$homebrew_status"; do
    case "$status" in
      materialized) materialized_count=$((materialized_count + 1)) ;;
      incompatible) incompatible_count=$((incompatible_count + 1)) ;;
      failed) failed_count=$((failed_count + 1)) ;;
      pending) pending_count=$((pending_count + 1)) ;;
    esac
  done

  if [ "$incompatible_count" -gt 0 ]; then
    if [ "$materialized_count" -gt 0 ]; then
      OVERALL_STATUS="partial"
    else
      OVERALL_STATUS="blocked"
    fi
  elif [ "$failed_count" -gt 0 ] || [ "$pending_count" -gt 0 ]; then
    if [ "$materialized_count" -gt 0 ]; then
      OVERALL_STATUS="partial"
    else
      OVERALL_STATUS="blocked"
    fi
  elif [ "$github_status" = "materialized" ] && [ "$npm_status" = "materialized" ] && [ "$homebrew_status" = "materialized" ]; then
    if [ "${RELEASE_ALREADY_MATERIALIZED:-0}" = "1" ]; then
      OVERALL_STATUS="materialized"
    else
      OVERALL_STATUS="completed"
    fi
  elif [ "$materialized_count" -gt 0 ]; then
    OVERALL_STATUS="partial"
  else
    OVERALL_STATUS="blocked"
  fi

  NEXT_SAFE_ACTION=$(release_next_safe_action "$OVERALL_STATUS")
}

release_parse_template_value() {
  file=$1
  key=$2
  awk -F'"' -v needle="$key" '$0 ~ "\"" needle "\"" { print $4; exit }' "$file"
}
