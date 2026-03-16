#!/bin/sh
set -eu

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
REPO_ROOT=$(CDPATH= cd -- "$SCRIPT_DIR/.." && pwd)
INSTALL_DIR="${SENTINEL_INSTALL_DIR:-$HOME/.local/bin}"
TARGET_BIN="$INSTALL_DIR/sentinel"
TMP_BIN="$INSTALL_DIR/.sentinel.new"
SOURCE_BIN="${SENTINEL_INSTALL_SOURCE:-}"
TARGET_VERSION=$(awk -F'"' '/^version = / { print $2; exit }' "$REPO_ROOT/Cargo.toml")

mkdir -p "$INSTALL_DIR"

read_version() {
  if [ ! -x "$1" ]; then
    return 1
  fi
  SENTINEL_INTERNAL_MODE=print-version "$1" 2>/dev/null || return 1
}

read_sha() {
  if [ ! -e "$1" ]; then
    return 1
  fi
  shasum -a 256 "$1" | awk '{ print $1 }'
}

detect_action() {
  if [ "${SENTINEL_REINSTALL:-0}" = "1" ]; then
    echo "reinstall"
    return
  fi

  if [ ! -e "$TARGET_BIN" ]; then
    echo "install"
    return
  fi

  CURRENT_VERSION=$(read_version "$TARGET_BIN" || true)
  if [ -z "$CURRENT_VERSION" ]; then
    echo "reinstall"
  elif [ "$CURRENT_VERSION" = "$TARGET_VERSION" ]; then
    CURRENT_SHA=$(read_sha "$TARGET_BIN" || true)
    SOURCE_SHA=$(read_sha "$SOURCE_BIN" || true)
    if [ -n "$CURRENT_SHA" ] && [ -n "$SOURCE_SHA" ] && [ "$CURRENT_SHA" != "$SOURCE_SHA" ]; then
      echo "reinstall"
    else
      echo "none"
    fi
  else
    echo "update"
  fi
}

ensure_source() {
  if [ -n "$SOURCE_BIN" ]; then
    return
  fi

  echo "Building Sentinel $TARGET_VERSION"
  cargo build --release --manifest-path "$REPO_ROOT/Cargo.toml"
  SOURCE_BIN="$REPO_ROOT/target/release/sentinel"
}

ensure_source

ACTION=$(detect_action)
if [ "$ACTION" = "none" ]; then
  echo "Sentinel $TARGET_VERSION is already installed at $TARGET_BIN"
  exit 0
fi

if [ ! -x "$SOURCE_BIN" ]; then
  echo "The installation source is not executable: $SOURCE_BIN" >&2
  exit 1
fi

SOURCE_SHA=$(shasum -a 256 "$SOURCE_BIN" | awk '{ print $1 }')
cp "$SOURCE_BIN" "$TMP_BIN"
chmod +x "$TMP_BIN"
TMP_VERSION=$(read_version "$TMP_BIN" || true)

if [ -z "$TMP_VERSION" ]; then
  echo "The new Sentinel artifact failed validation before replacement." >&2
  rm -f "$TMP_BIN"
  exit 1
fi

mv "$TMP_BIN" "$TARGET_BIN"

if [ -x "$TARGET_BIN" ]; then
  export PATH="$INSTALL_DIR:$PATH"
fi

echo "Sentinel action: $ACTION"
echo "Installed version: $TMP_VERSION"
echo "Executable path: $TARGET_BIN"
echo "Artifact SHA256: $SOURCE_SHA"

if ! printf '%s' "$PATH" | tr ':' '\n' | grep -qx "$INSTALL_DIR"; then
  echo "Add $INSTALL_DIR to PATH to call sentinel directly."
fi
