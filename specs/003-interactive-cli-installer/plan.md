# Implementation Plan: Interactive Sentinel CLI

**Branch**: `003-interactive-cli-installer` | **Date**: 2026-03-14 | **Spec**: [spec.md](/Users/santiagoacosta/Documents/personal-projects/sentinel-cli/specs/003-interactive-cli-installer/spec.md)
**Input**: Feature specification from `/specs/003-interactive-cli-installer/spec.md`

## Summary

Deliver Sentinel as a fully interactive terminal product for end users: one
entry command, guided navigation, safety checks before touching network state, a
large default ad-domain blocklist, and a shell installer that installs,
updates, or reinstalls Sentinel into the user's `PATH`. The implementation will
optimize for end-user safety, restrained scope, and a polished terminal
experience rather than exposing flags or advanced automation in the first
release.

## Technical Context

**Language/Version**: Rust 1.90.0 (edition 2024) plus POSIX shell for installation workflows  
**Primary Dependencies**: `ratatui`, `crossterm`, `tokio`, `hickory-proto`, `serde`, `serde_json`, `toml`, `miette`, `thiserror`, `tracing`, `tracing-subscriber`, `directories`, `uuid`, shell tooling via `sh`, `curl`, `tar`, and `shasum`  
**Storage**: Local filesystem state under application support directories (`TOML` for user configuration, `JSON`/`JSONL` for runtime state, health checks, install metadata, and recovery events)  
**Testing**: `cargo test`, interactive flow integration tests, recovery-path integration tests, snapshot tests for terminal screens, installer script validation in shell-based test flows  
**Target Platform**: macOS 14+ for the first end-user release  
**Project Type**: Interactive system-level CLI application with a companion install/update shell script  
**Performance Goals**: Main interactive screen available in under 2 seconds, safety checks complete in under 5 seconds on healthy systems, status refresh under 300ms, installer/update flow completes in under 2 minutes on a healthy connection  
**Constraints**: Normal end-user flow must not depend on flags, network changes must be guarded by recoverable safety checks, installation must place Sentinel in the `PATH`, updater must safely replace existing installs, and the initial feature set must stay limited to core protection flows  
**Scale/Scope**: Single-device, single-user first release with guided activation, status, recovery, broad bundled ad blocklist, and a single install/update/reinstall script

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### Phase 0 Gate

- `PASS`: Command surface stays minimal because the end-user interaction is a
  single guided terminal experience plus one install/update script.
- `PASS`: The UX direction is explicit: visually clean terminal screens,
  focused menus, actionable health states, and recovery-first messaging.
- `PASS`: Network safety is central through pre-activation checks, recovery
  snapshots, rollback behavior, and installation flows that do not mutate live
  protection state silently.
- `PASS`: The structure separates UI, control, blocking runtime, platform
  networking, storage, and installer logic into narrow modules.
- `PASS`: Non-essential features are deferred, including scripting flags, remote
  management, GUI clients, multi-profile administration, and extensive rule
  editing.

### Post-Design Re-check

- `PASS`: Research decisions keep the user-facing flow interactive and
  low-noise instead of adding flag-heavy or multi-binary operator workflows.
- `PASS`: Contracts cover the interactive session and install/update script
  explicitly, which keeps public behavior testable and user-centered.
- `PASS`: The design preserves safety-first activation and recovery semantics
  across both runtime and installation flows.
- `PASS`: No constitution violations require justification at planning time.

## Project Structure

### Documentation (this feature)

```text
specs/003-interactive-cli-installer/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   ├── interaction-contract.md
│   └── install-script-contract.md
└── tasks.md
```

### Source Code (repository root)

```text
src/
├── main.rs
├── app.rs
├── tui/
│   ├── mod.rs
│   ├── app_state.rs
│   ├── input.rs
│   ├── screens.rs
│   └── theme.rs
├── control/
│   ├── mod.rs
│   ├── activation.rs
│   ├── recovery.rs
│   └── safety.rs
├── blocking/
│   ├── mod.rs
│   ├── runtime.rs
│   ├── resolver.rs
│   └── blocklist.rs
├── platform/
│   ├── mod.rs
│   └── macos.rs
├── storage/
│   ├── mod.rs
│   ├── config.rs
│   ├── install.rs
│   ├── state.rs
│   └── events.rs
└── install/
    ├── mod.rs
    └── version.rs

scripts/
└── install-sentinel.sh

assets/
└── blocklists/
    └── default-domains.txt

tests/
├── contract/
├── integration/
└── snapshot/
```

**Structure Decision**: Use a single Rust application for the interactive
product experience and a dedicated shell installer for distribution lifecycle.
This keeps end-user interaction simple while preserving modular boundaries for
UI, network safety, blocklist handling, and installation/update logic.

## Complexity Tracking

No constitution violations are expected from this plan. The design deliberately
avoids a separate GUI, background admin service manager, or power-user flag
surface in the first release.
