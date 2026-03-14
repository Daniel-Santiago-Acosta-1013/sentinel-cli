# Quickstart: Interactive Sentinel CLI

## Prerequisites

- Rust 1.90.0 or newer available through `cargo`
- macOS 14+ environment for the first release target
- Shell environment capable of running the official installer script

## Development Bootstrap

1. Build the Rust application with `cargo build`.
2. Validate the interactive flow in safe mode with
   `SENTINEL_FAKE_PLATFORM=1 SENTINEL_SCRIPT=exit cargo run`.
3. Install Sentinel into a target `PATH` directory with
   `sh scripts/install-sentinel.sh`.

## Manual Validation Flow

1. Run Sentinel with `SENTINEL_FAKE_PLATFORM=1` and confirm the home screen
   appears without flags.
2. Navigate with arrow keys, run safety checks, and confirm Sentinel explains
   whether activation can proceed.
3. Enable protection, verify the interface reports an active state, then
   disable or recover to restore the prior DNS state.
4. Execute the installer script on a clean environment and verify `sentinel` is
   callable from the chosen `PATH`.
5. Execute the same installer again on an existing install and verify it chooses
   update or reinstall correctly.

## Quality Checks

- `cargo test`
- Interactive flow integration tests for navigation and safety gating
- Recovery-path validation for interrupted or failed network changes
- Snapshot or golden-output checks for terminal screens
- Installer script verification for install, update, and reinstall outcomes
