# Implementation Plan: Sentinel CLI MVP

**Branch**: `002-build-cli-mvp` | **Date**: 2026-03-14 | **Spec**: [spec.md](/Users/santiagoacosta/Documents/personal-projects/sentinel-cli/specs/002-build-cli-mvp/spec.md)
**Input**: Feature specification from `/specs/002-build-cli-mvp/spec.md`

## Summary

Build the first usable Sentinel CLI as a Rust-based system utility with a
minimal command surface, readable terminal output, safe recovery behavior, and
precise ad blocking centered on domain decisions. The implementation will use a
single Rust workspace with a user-facing CLI control plane and an internal
protection runtime that owns tunnel lifecycle, DNS-first blocking decisions, and
network state restoration.

## Technical Context

**Language/Version**: Rust 1.90.0 (edition 2024)  
**Primary Dependencies**: `clap`, `tokio`, `tun`, `hickory-proto`, `serde`, `serde_json`, `toml`, `miette`, `thiserror`, `tracing`, `tracing-subscriber`, `comfy-table`, `directories`  
**Storage**: Local filesystem state in application support directories (`TOML` for config, `JSON`/`JSONL` for runtime state and events)  
**Testing**: `cargo test`, unit tests, `assert_cmd` CLI integration tests, snapshot checks for terminal output, recovery-path integration tests  
**Target Platform**: macOS 14+ for the first release  
**Project Type**: System-level CLI application with a background protection runtime  
**Performance Goals**: Status command under 200ms, activation under 5s on healthy systems, recovery under 10s, rule evaluation fast enough for 50k active rule entries without visible CLI lag  
**Constraints**: Must preserve connectivity first, must own the protection engine rather than wrapping a third-party VPN app, must keep the command surface minimal, must support human-readable and JSON output, must persist recoverable network state locally  
**Scale/Scope**: Single-device, single-operator MVP with essential commands only: enable, disable, status, rules, allow, recover, events

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### Phase 0 Gate

- `PASS`: Command surface stays minimal through seven user-facing commands and
  excludes dashboards, GUI flows, remote sync, and automation extras.
- `PASS`: The terminal UX is intentional: concise defaults, structured status
  views, JSON mode for scripting, and `miette`-style error reporting for clear
  recovery guidance.
- `PASS`: Safety is a first-class design axis through network snapshots,
  explicit rollback, idempotent commands, and a dedicated recovery flow.
- `PASS`: The architecture keeps modules narrow: CLI, control plane, engine,
  storage, and platform logic are isolated by responsibility.
- `PASS`: Non-essential work is explicitly deferred, including cross-platform
  support, advanced analytics, auto-updating rule feeds, and GUI polish.

### Post-Design Re-check

- `PASS`: Research decisions preserve a narrow DNS-first MVP instead of broad
  packet inspection or multi-mode filtering.
- `PASS`: Contracts and data model keep the public interface focused on user
  actions and recovery, not on internal implementation details.
- `PASS`: The planned structure supports essential Rustdoc comments on public
  behavior and invariants without adding documentation bulk.
- `PASS`: No constitution violations require justification at planning time.

## Project Structure

### Documentation (this feature)

```text
specs/002-build-cli-mvp/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── cli-contract.md
└── tasks.md
```

### Source Code (repository root)

```text
src/
├── main.rs
├── app.rs
├── cli/
│   ├── mod.rs
│   ├── commands.rs
│   ├── output.rs
│   └── styles.rs
├── control/
│   ├── mod.rs
│   ├── coordinator.rs
│   ├── recovery.rs
│   └── snapshot.rs
├── core/
│   ├── mod.rs
│   ├── events.rs
│   ├── rules.rs
│   └── state.rs
├── engine/
│   ├── mod.rs
│   ├── runtime.rs
│   ├── tunnel.rs
│   └── dns.rs
├── platform/
│   ├── mod.rs
│   └── macos.rs
└── storage/
    ├── mod.rs
    ├── config.rs
    ├── state.rs
    └── events.rs

tests/
├── contract/
├── integration/
└── unit/
```

**Structure Decision**: Use a single Rust application with explicit module
boundaries so the CLI experience, recovery logic, protection runtime, and
platform-specific networking can evolve independently without inflating the
public command surface.

## Complexity Tracking

No constitution violations are expected from this plan. The design intentionally
avoids extra binaries, plugin systems, remote control APIs, or generalized
abstraction layers in the MVP.
