---

description: "Task list for Sentinel CLI MVP implementation"

---

# Tasks: Sentinel CLI MVP

**Input**: Design documents from `/specs/002-build-cli-mvp/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/cli-contract.md, quickstart.md

**Tests**: This feature requires unit, contract, integration, recovery, and CLI output validation because the specification demands safe network behavior, precise rule handling, and a polished terminal experience.

**Organization**: Tasks are grouped by user story to keep each increment independently testable and shippable.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel when files and dependencies do not overlap
- **[Story]**: Maps the task to a specific user story (`[US1]`, `[US2]`, `[US3]`)
- Every task includes the exact file path to change

## Path Conventions

- Application code lives in `src/`
- Automated tests live in `tests/`
- Feature design artifacts live in `specs/002-build-cli-mvp/`

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Bootstrap the Rust application and establish the CLI presentation baseline

- [X] T001 Create the Rust application manifest and dependency set in `Cargo.toml`
- [X] T002 Create the crate entrypoints and module skeleton in `src/main.rs`, `src/app.rs`, `src/cli/mod.rs`, `src/control/mod.rs`, `src/core/mod.rs`, `src/engine/mod.rs`, `src/platform/mod.rs`, and `src/storage/mod.rs`
- [X] T003 [P] Configure development defaults in `.gitignore`, `.cargo/config.toml`, and `rustfmt.toml`
- [X] T004 [P] Define CLI copy, help, and styling conventions in `src/cli/output.rs` and `src/cli/styles.rs`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Establish the safety, persistence, and engine primitives required by every story

**⚠️ CRITICAL**: No user story work should begin until this phase is complete

- [X] T005 Implement application bootstrapping and shared error types in `src/app.rs`
- [X] T006 [P] Implement persistent config loading and validation in `src/storage/config.rs`
- [X] T007 [P] Implement protection state and event persistence in `src/storage/state.rs` and `src/storage/events.rs`
- [X] T008 [P] Implement core domain models for protection state, rules, and events in `src/core/state.rs`, `src/core/rules.rs`, and `src/core/events.rs`
- [X] T009 Implement network snapshot capture and validation in `src/control/snapshot.rs`
- [X] T010 Implement recovery primitives around saved snapshots in `src/control/recovery.rs`
- [X] T011 Implement the macOS networking adapter for DNS, routes, and rollback hooks in `src/platform/macos.rs`
- [X] T012 Implement tunnel lifecycle primitives in `src/engine/tunnel.rs`
- [X] T013 Implement runtime coordination boundaries between control and engine layers in `src/control/coordinator.rs` and `src/engine/runtime.rs`

**Checkpoint**: Foundation ready for story work

---

## Phase 3: User Story 1 - Activar protección con claridad y seguridad (Priority: P1) 🎯 MVP

**Goal**: Let the user enable, disable, and inspect protection safely with clear terminal output and recoverable networking

**Independent Test**: Run `sentinel status`, `sentinel enable`, and `sentinel disable` on a healthy macOS environment and confirm the CLI reports the right state, preserves connectivity, and restores the previous network state on shutdown or failure.

### Validation for User Story 1

- [X] T014 [P] [US1] Add CLI contract tests for `enable`, `disable`, and `status` in `tests/contract/test_cli_lifecycle.rs`
- [X] T015 [P] [US1] Add integration tests for activation, rollback, and idempotent lifecycle behavior in `tests/integration/test_activation_flow.rs`
- [X] T016 [P] [US1] Add output and exit-code checks for lifecycle commands in `tests/integration/test_cli_output.rs`

### Implementation for User Story 1

- [X] T017 [US1] Implement lifecycle command parsing for `enable`, `disable`, and `status` in `src/cli/commands.rs` and `src/main.rs`
- [X] T018 [US1] Implement DNS-first blocking decisions for built-in rules in `src/engine/dns.rs`
- [X] T019 [US1] Implement activation and deactivation orchestration in `src/control/coordinator.rs`
- [X] T020 [US1] Wire tunnel runtime supervision and protection state transitions in `src/engine/runtime.rs` and `src/engine/tunnel.rs`
- [X] T021 [US1] Implement lifecycle status rendering, summaries, and safe error presentation in `src/cli/output.rs` and `src/cli/styles.rs`
- [X] T022 [US1] Persist lifecycle state changes and operator-visible events in `src/storage/state.rs` and `src/storage/events.rs`

**Checkpoint**: User Story 1 is independently functional and demonstrable

---

## Phase 4: User Story 2 - Ajustar precisión sin saturar la herramienta (Priority: P2)

**Goal**: Let the user add, remove, and inspect essential allow rules without expanding the command surface beyond the MVP

**Independent Test**: Activate protection, add an allow rule for a legitimate domain, confirm traffic remains protected elsewhere, and verify `rules list` shows a concise summary of block and allow entries.

### Validation for User Story 2

- [X] T023 [P] [US2] Add CLI contract tests for `allow add`, `allow remove`, and `rules list` in `tests/contract/test_cli_rules.rs`
- [X] T024 [P] [US2] Add integration tests for allow-rule precedence, normalization, and duplicate handling in `tests/integration/test_rule_management.rs`
- [X] T025 [P] [US2] Add output checks for concise rule summaries in `tests/integration/test_rule_output.rs`

### Implementation for User Story 2

- [X] T026 [US2] Implement rule normalization and repository behavior in `src/core/rules.rs` and `src/storage/config.rs`
- [X] T027 [US2] Implement `allow add` and `allow remove` command handlers in `src/cli/commands.rs` and `src/app.rs`
- [X] T028 [US2] Implement allow-over-block precedence in the DNS evaluator in `src/engine/dns.rs`
- [X] T029 [US2] Implement `rules list` rendering for human and JSON modes in `src/cli/output.rs`
- [X] T030 [US2] Persist user rule changes and related events in `src/storage/config.rs` and `src/storage/events.rs`

**Checkpoint**: User Stories 1 and 2 both work independently

---

## Phase 5: User Story 3 - Ver estado y recuperar confianza operativa (Priority: P3)

**Goal**: Let the user inspect runtime health, review recent events, and recover the network after interrupted or degraded sessions

**Independent Test**: Put the tool into an active or degraded state, run `sentinel status`, `sentinel events --limit 5`, and `sentinel recover`, then verify the CLI reports recent events and restores a valid network state.

### Validation for User Story 3

- [X] T031 [P] [US3] Add CLI contract tests for `recover` and `events` in `tests/contract/test_cli_recovery.rs`
- [X] T032 [P] [US3] Add integration tests for interrupted-session recovery and degraded-state handling in `tests/integration/test_recovery_flow.rs`
- [X] T033 [P] [US3] Add output checks for degraded status, events, and recovery guidance in `tests/integration/test_status_events_output.rs`

### Implementation for User Story 3

- [X] T034 [US3] Implement `recover` and `events` command parsing in `src/cli/commands.rs` and `src/main.rs`
- [X] T035 [US3] Implement recovery orchestration using the latest valid snapshot in `src/control/recovery.rs` and `src/control/coordinator.rs`
- [X] T036 [US3] Implement degraded-state reporting and event queries in `src/core/state.rs` and `src/storage/events.rs`
- [X] T037 [US3] Implement status, event, and recovery guidance views in `src/cli/output.rs` and `src/cli/styles.rs`

**Checkpoint**: All user stories are independently functional

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Tighten final quality, keep the command surface disciplined, and align docs with the delivered MVP

- [X] T038 [P] Add essential Rustdoc comments for public invariants in `src/app.rs` and `src/control/coordinator.rs`
- [X] T039 [P] Add regression checks for JSON output stability in `tests/integration/test_cli_output.rs` and `tests/integration/test_rule_output.rs`
- [X] T040 Review and trim non-essential flags or copy from the command surface in `src/cli/commands.rs`
- [X] T041 Update verification steps and command examples in `specs/002-build-cli-mvp/quickstart.md`
- [X] T042 Finalize Cargo quality commands and dependency hygiene in `Cargo.toml`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1**: Starts immediately
- **Phase 2**: Depends on Phase 1 and blocks all story work
- **Phase 3**: Depends on Phase 2 and delivers the MVP
- **Phase 4**: Depends on Phase 3 because allow rules rely on the active protection lifecycle
- **Phase 5**: Depends on Phase 3 and may proceed after Phase 4 if team capacity allows, though event visibility benefits from completed rule persistence
- **Phase 6**: Depends on all desired user stories being complete

### User Story Dependencies

- **US1 (P1)**: No dependency on later stories; this is the recommended MVP slice
- **US2 (P2)**: Depends on US1 runtime activation, state persistence, and DNS evaluation
- **US3 (P3)**: Depends on US1 recovery and state infrastructure; it can be completed before or after US2, but event usefulness improves once US2 writes rule events

### Within Each User Story

- Validation tasks come before implementation
- Command parsing comes before orchestration wiring
- Domain/state changes come before output rendering
- Persistence updates come before end-to-end verification

## Parallel Opportunities

- `T003` and `T004` can run in parallel after `T001` and `T002`
- `T006`, `T007`, and `T008` can run in parallel in Phase 2
- All validation tasks marked `[P]` inside each user story can run in parallel
- `T026` and rule output test preparation can proceed in parallel once US1 is stable
- `T036` and `T037` can proceed in parallel once `T035` defines the recovery result shape

## Parallel Example: User Story 1

```bash
Task: "Add CLI contract tests for enable, disable, and status in tests/contract/test_cli_lifecycle.rs"
Task: "Add integration tests for activation, rollback, and idempotent lifecycle behavior in tests/integration/test_activation_flow.rs"
Task: "Add output and exit-code checks for lifecycle commands in tests/integration/test_cli_output.rs"
```

## Parallel Example: User Story 2

```bash
Task: "Add CLI contract tests for allow add, allow remove, and rules list in tests/contract/test_cli_rules.rs"
Task: "Add integration tests for allow-rule precedence, normalization, and duplicate handling in tests/integration/test_rule_management.rs"
Task: "Add output checks for concise rule summaries in tests/integration/test_rule_output.rs"
```

## Parallel Example: User Story 3

```bash
Task: "Add CLI contract tests for recover and events in tests/contract/test_cli_recovery.rs"
Task: "Add integration tests for interrupted-session recovery and degraded-state handling in tests/integration/test_recovery_flow.rs"
Task: "Add output checks for degraded status, events, and recovery guidance in tests/integration/test_status_events_output.rs"
```

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1
2. Complete Phase 2
3. Complete Phase 3
4. Validate `status`, `enable`, and `disable` end to end
5. Stop and review before expanding the command surface

### Incremental Delivery

1. Ship the lifecycle and safety core in US1
2. Add precise allow-rule management in US2
3. Add recovery and event confidence flows in US3
4. Finish with polish, JSON stability, and CLI trimming

### Parallel Team Strategy

1. One developer handles storage/core foundations while another sets up CLI presentation during Phases 1 and 2
2. After US1 stabilizes, one developer can own rule management while another prepares recovery/event validation
3. Merge only after each story passes its independent test criteria

## Notes

- Every task follows the required checklist format with checkbox, ID, and file path
- User story phases include `[US1]`, `[US2]`, and `[US3]` labels on all story-specific tasks
- The recommended MVP scope is Phase 3 only
- Keep the command surface limited to the contract in `specs/002-build-cli-mvp/contracts/cli-contract.md`
