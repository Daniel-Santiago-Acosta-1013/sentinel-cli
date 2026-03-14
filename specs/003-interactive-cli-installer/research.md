# Research: Interactive Sentinel CLI

## Decision: Use an interactive terminal UI as the primary user experience

**Rationale**: The feature explicitly removes flags from the normal user flow.
A terminal UI allows Sentinel to guide installation health, protection state,
and recovery decisions inside one clean interaction model.

**Alternatives considered**:
- Traditional subcommands and flags: simpler for operators, but conflicts with
  the product goal of a guided end-user experience.
- Full desktop GUI: visually richer, but outside the scope and install surface
  of this phase.

## Decision: Use `ratatui` plus `crossterm` for screen rendering and input

**Rationale**: The interface must feel elaborate, clean, and controlled rather
than like a sequence of plain prompts. This pairing supports structured screens,
keyboard input, and a polished terminal layout without leaving the CLI domain.

**Alternatives considered**:
- Prompt-only flow with line-by-line questions: lightweight, but too limited for
  the requested visual quality.
- A custom renderer from scratch: possible, but wasteful and harder to maintain.

## Decision: Keep a single end-user entry command and one official install script

**Rationale**: The constitution requires a minimal command surface. A single
launch command for runtime interaction and a single shell script for install,
update, and reinstall produces a cleaner experience than multiple user-facing
commands.

**Alternatives considered**:
- Separate scripts for install and update: more explicit, but increases
  discovery and maintenance burden.
- Multi-command CLI for different maintenance operations: adds mental load and
  conflicts with the interactive-first product direction.

## Decision: Use broad bundled ad-domain coverage from a maintained local blocklist asset

**Rationale**: The feature requires strong blocking coverage from first use.
Bundling a curated default blocklist allows Sentinel to start working
immediately without requiring the user to fetch or configure lists manually.

**Alternatives considered**:
- Small starter list only: safer to curate, but too weak for the stated goal.
- Runtime-only remote list download on first run: flexible, but adds fragility
  and makes the first-run experience dependent on network availability.

## Decision: Preserve safety through explicit preflight checks and recovery snapshots

**Rationale**: The product goal is to work on a real end-user device without
damaging network integrity. Safety checks before activation and stored recovery
snapshots are the clearest way to enforce that constraint.

**Alternatives considered**:
- Best-effort activation without preflight validation: faster to implement, but
  unacceptable for end-user safety.
- Manual backup instructions only: too error-prone for a consumer-grade flow.

## Decision: Keep the blocking methodology DNS-first for the first end-user release

**Rationale**: DNS-first blocking offers a strong balance between broad ad
domain coverage, operational clarity, and lower risk to legitimate traffic when
compared with broad packet-level inspection.

**Alternatives considered**:
- Full packet filtering from the first release: more comprehensive, but much
  riskier and more complex to ship safely.
- Hosts-file-only blocking: simpler, but weaker for health checks, recovery, and
  future runtime control.

## Decision: Let the install script own install, update, and reinstall decisions

**Rationale**: Users should not have to determine installation state manually.
The script can inspect the current install, compare versions, and choose the
safe action path while keeping the user-facing workflow short.

**Alternatives considered**:
- Manual reinstall instructions: too fragile and inconsistent for end users.
- Self-updating binary logic only: useful later, but less transparent and less
  portable than an explicit shell workflow.

## Decision: Validate installer integrity with version and artifact checks before replacement

**Rationale**: Updating a system-level network tool must be careful. The
installer should verify what it is replacing and only swap binaries after a
valid download and integrity check.

**Alternatives considered**:
- Blind overwrite of the existing binary: simplest, but too risky.
- Manual download plus copy steps: safer for experts, but poor end-user UX.
