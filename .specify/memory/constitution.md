<!--
Sync Impact Report
- Version change: template -> 1.0.0
- Modified principles:
  - Template principle 1 -> I. Minimal Command Surface
  - Template principle 2 -> II. Exceptional Terminal Experience
  - Template principle 3 -> III. Safety Before Blocking
  - Template principle 4 -> IV. Focused, Modular Quality
  - Template principle 5 -> V. Deliberate Delivery
- Added sections:
  - Product Experience Standards
  - Delivery Workflow
- Removed sections:
  - None
- Templates requiring updates:
  - ✅ .specify/templates/plan-template.md
  - ✅ .specify/templates/spec-template.md
  - ✅ .specify/templates/tasks-template.md
  - ✅ .specify/templates/constitution-template.md
  - ✅ .specify/templates/commands/*.md not present; no update required
- Follow-up TODOs:
  - None
-->

# Sentinel CLI Constitution

## Core Principles

### I. Minimal Command Surface
The CLI MUST expose only commands, flags, and configuration paths that directly
support blocking, recovery, rule management, and diagnostics. Every addition
MUST justify user value, operating frequency, and why an existing flow cannot
cover the need. Speculative features, duplicate entry points, and low-usage
options MUST be rejected or deferred.

Rationale: A small command surface keeps the tool learnable, reduces operator
error, and prevents the product from becoming saturated with marginal features.

### II. Exceptional Terminal Experience
Every command MUST produce output that is clean, legible, and action-oriented.
Human-readable output MUST show current state, risk, result, and next step
without visual noise; machine-readable output, when offered, MUST remain stable
and structured. Errors MUST explain what failed, why it matters, and the safest
recovery path.

Rationale: This project lives in the terminal. Quality is visible in naming,
copy, spacing, defaults, and the speed with which a user understands the result.

### III. Safety Before Blocking
Any action that can affect network behavior MUST protect connectivity first.
Before applying changes, the system MUST preserve recoverable state, validate a
safe rollback path, and define failure handling. If safety cannot be guaranteed,
the operation MUST stop or revert rather than partially succeed.

Rationale: A system-level network tool loses trust immediately if it can leave
the device disconnected or in an unknown state.

### IV. Focused, Modular Quality
The codebase MUST be organized into small modules with one operational
responsibility and explicit boundaries. Public modules, commands, and critical
flows MUST include only the essential documentation needed for a human
maintainer to understand behavior, risks, and invariants. Architectural
complexity, broad abstractions, and documentation bulk without proven need MUST
be removed.

Rationale: The project must scale in capability without scaling in confusion.
Clean structure and concise documentation are required to sustain quality.

### V. Deliberate Delivery
Every change MUST identify the user journey it improves, the minimal scope
required to ship it, and the acceptance checks that prove it works. Plans and
tasks MUST include CLI UX validation, safety validation for network-affecting
flows, and explicit deferral of non-essential work. Visible polish is required,
but feature sprawl is forbidden.

Rationale: Precision in scope and validation is how the project stays useful,
high quality, and intuitive over time.

## Product Experience Standards

- Command names MUST use direct verbs and consistent nouns so the next action is
  obvious without reading extensive help.
- Default output MUST be brief and readable; extra detail MUST be opt-in rather
  than dumped by default.
- Interactive prompts MUST be reserved for decisions that can cause risk or data
  loss; routine commands MUST remain scriptable.
- Help text MUST prioritize examples, safe usage, and recovery commands over
  exhaustive option descriptions.
- Visual styling MUST stay restrained: aligned sections, meaningful emphasis,
  and no decorative output that competes with the message.

## Delivery Workflow

- Specifications MUST describe user value, scope boundaries, CLI experience
  expectations, operational safety considerations, and measurable outcomes.
- Implementation plans MUST pass a Constitution Check covering minimal surface,
  terminal UX, safety and rollback behavior, modular structure, and deferral of
  non-essential complexity.
- Task lists MUST separate foundational safety and UX work from feature-specific
  tasks so each user story remains independently deliverable.
- Reviews MUST reject unclear copy, unnecessary options, hidden side effects,
  and any change that increases scope without improving the core user journey.

## Governance

This constitution overrides conflicting local habits and planning shortcuts for
Sentinel CLI. Amendments MUST include a written rationale, a Sync Impact Report,
and updates to any affected templates or guidance files in the same change.

Versioning policy:
- MAJOR: Remove or redefine a core principle or governance rule in a
  backward-incompatible way.
- MINOR: Add a new principle, a new mandatory section, or materially expand
  project-wide guidance.
- PATCH: Clarify wording, tighten examples, or make non-semantic refinements.

Compliance review expectations:
- Every specification, plan, task list, and implementation review MUST check for
  alignment with all five core principles.
- Any exception MUST be documented in the relevant artifact with a clear reason,
  impact, and the simpler alternative that was rejected.
- Work that fails the constitution check MUST not proceed until brought into
  compliance or explicitly amended in this document.

**Version**: 1.0.0 | **Ratified**: 2026-03-14 | **Last Amended**: 2026-03-14
