# Quickstart: Sentinel CLI MVP

## Prerequisites

- Rust 1.90.0 or newer available through `cargo`
- macOS 14+ environment for the first implementation target
- Elevated privileges available for commands that modify system networking

## Bootstrap

1. Create the Rust application skeleton and module layout defined in [plan.md](/Users/santiagoacosta/Documents/personal-projects/sentinel-cli/specs/002-build-cli-mvp/plan.md).
2. Add the CLI, runtime, storage, and platform dependencies from the technical
   context.
3. Implement the command surface exactly as defined in [cli-contract.md](/Users/santiagoacosta/Documents/personal-projects/sentinel-cli/specs/002-build-cli-mvp/contracts/cli-contract.md).

## First-run Development Flow

1. Run `cargo fmt` and `cargo test` after scaffolding the crate structure.
2. Implement `sentinel status` and its output contract before activation logic.
3. Implement `enable` with snapshot validation and safe rollback behavior.
4. Implement `disable` and `recover` before expanding rule management.
5. Add `allow add`, `allow remove`, `rules list`, and `events` once lifecycle
   and recovery flows are stable.

## Manual Verification Flow

1. Execute `sentinel status` on a clean environment and confirm the tool reports
   `inactive`.
2. Execute `sentinel enable` and verify:
   - protection becomes active
   - a network snapshot is recorded
   - human-readable output explains the result
3. Execute `sentinel allow add example.com` and verify the exclusion is listed.
4. Execute `sentinel events --limit 5` and confirm recent lifecycle events
   appear in order.
5. Execute `sentinel disable` and verify the original network state is restored.

## Quality Checks

- `cargo test`
- CLI integration coverage for `enable`, `disable`, `status`, `allow`, and
  `recover`
- Snapshot or golden-output checks for human-readable terminal output
- Recovery-path validation for interrupted or failed activation flows
