# Research: Sentinel CLI MVP

## Decision: Use a single Rust binary with an internal control plane and protection runtime

**Rationale**: A single binary reduces operational complexity for the first
release while still allowing a clean architectural split between command parsing,
protection lifecycle, recovery orchestration, and engine internals. This keeps
installation simple and matches the constitution's minimal surface requirement.

**Alternatives considered**:
- Separate CLI and daemon executables: clearer process isolation, but more setup,
  packaging, and coordination burden for an MVP.
- Shelling out to platform scripts for all protection behavior: faster to start,
  but too brittle and not aligned with owning the engine.

## Decision: Adopt a DNS-first blocking model inside the owned tunnel runtime

**Rationale**: Domain-level decisions provide the best precision-to-risk ratio
for an MVP. Blocking at the DNS decision point captures most ad traffic without
expanding immediately into broad packet inspection or content heuristics, which
would increase breakage risk and complexity.

**Alternatives considered**:
- Full packet payload inspection: more powerful, but high complexity and greater
  risk of false positives and privacy concerns.
- Hosts-file-only blocking: simpler, but weaker recovery control and less aligned
  with the goal of owning a protection engine.

## Decision: Use `clap` for command ergonomics and stable command parsing

**Rationale**: `clap` gives predictable subcommands, strong help generation, and
validation for a terminal-first product. It supports the constitution's need for
clear command names, concise help, and a minimal command surface.

**Alternatives considered**:
- `argh`: lighter, but less expressive for a CLI expected to grow moderately.
- Hand-rolled parsing: possible, but wasteful and error-prone for user-facing UX.

## Decision: Use `miette` + `thiserror` for readable, structured terminal errors

**Rationale**: The user explicitly delegated terminal error presentation. This
pair provides precise domain errors with polished output and actionable recovery
guidance, which matches the constitution's emphasis on clarity and aesthetics.

**Alternatives considered**:
- `anyhow` only: simpler, but weaker for domain-specific, user-facing error
  messages.
- Plain stderr strings: less consistent and harder to maintain as flows grow.

## Decision: Use `tokio` to coordinate long-lived runtime work

**Rationale**: The protection runtime must supervise tunnel lifecycle, DNS
decisions, health checks, and recovery sequencing. An async runtime keeps those
concerns coordinated without blocking the CLI on slow operations.

**Alternatives considered**:
- Pure synchronous execution: simpler on paper, but brittle once runtime tasks
  and background supervision become concurrent.
- A custom event loop: unnecessary for an MVP and harder to reason about.

## Decision: Use `tun` to own the local tunnel interface

**Rationale**: The feature requires owning the engine rather than wrapping an
existing VPN product. A tunnel abstraction provides a practical path to manage
the interface directly while keeping the public CLI independent from third-party
GUI tools.

**Alternatives considered**:
- Wrapping an external VPN app: explicitly rejected by project intent.
- Direct platform-specific syscalls everywhere: viable later, but too coupled
  and repetitive for the first implementation layer.

## Decision: Use `hickory-proto` for DNS packet parsing and decision flow

**Rationale**: DNS-first blocking needs reliable message parsing and response
construction. `hickory-proto` supplies the protocol building blocks without
forcing a full resolver architecture onto the MVP.

**Alternatives considered**:
- Hand-written DNS parsing: too error-prone for a correctness-sensitive path.
- Full recursive resolver stack: more than the MVP requires.

## Decision: Store config and state locally using `TOML` plus `JSON`/`JSONL`

**Rationale**: Human-edited configuration works well in `TOML`, while runtime
state and event streams benefit from machine-friendly structured records. This
supports clear diagnostics, safe recovery, and possible future automation.

**Alternatives considered**:
- SQLite: powerful, but unnecessary for first-release state volume.
- Plain text ad-hoc files: lighter, but weaker for validation and recovery.

## Decision: Use `tracing` + `tracing-subscriber` for operational events

**Rationale**: The CLI needs concise human output, but the runtime also needs
structured events for diagnosis and recovery. `tracing` provides that split
cleanly without bloating the user-facing interface.

**Alternatives considered**:
- `log` + `env_logger`: adequate for simple logs, but weaker for structured
  context and event-driven flows.
- Custom logging framework: not justified for an MVP.

## Decision: Use `comfy-table` for status and rule summaries

**Rationale**: The CLI must feel intentionally designed, not raw. Short,
well-aligned tables for status and rules improve readability while preserving a
restrained terminal aesthetic.

**Alternatives considered**:
- Manual string alignment: workable, but more fragile and repetitive.
- Rich TUI framework: overkill for a command-oriented MVP.
