# Tasks: Interactive Sentinel CLI

**Input**: Design documents from `/specs/003-interactive-cli-installer/`
**Prerequisites**: `plan.md`, `spec.md`, `research.md`, `data-model.md`, `contracts/`

**Tests**: Safety, installer, integration, and terminal snapshot tests are required for this feature because it changes live network behavior and the end-user interaction model.

**Organization**: Tasks are grouped by user story so each story remains independently implementable and testable once the foundational phase is complete.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel when it touches a different file and has no unmet dependency
- **[Story]**: Maps the task to a specific user story (`US1`, `US2`, `US3`)
- Every task includes an exact file path

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Establish the crate, module boundaries, and bundled assets for the interactive product.

- [X] T001 Update crate metadata and feature dependencies in Cargo.toml
- [X] T002 Create the interactive-only binary entrypoint in src/main.rs
- [X] T003 [P] Define top-level application orchestration in src/app.rs
- [X] T004 [P] Declare the TUI module surface in src/tui/mod.rs
- [X] T005 [P] Declare the control module surface in src/control/mod.rs
- [X] T006 [P] Declare the blocking module surface in src/blocking/mod.rs
- [X] T007 [P] Declare the platform module surface in src/platform/mod.rs
- [X] T008 [P] Declare the storage module surface in src/storage/mod.rs
- [X] T009 [P] Declare the installer module surface in src/install/mod.rs
- [X] T010 [P] Seed the bundled ad-domain asset in assets/blocklists/default-domains.txt

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Build the safety-critical primitives that every user story depends on before any user-facing flow is completed.

**⚠️ CRITICAL**: No user story work can be considered done until this phase is complete.

- [X] T011 Implement application directory resolution and config defaults in src/storage/config.rs
- [X] T012 [P] Implement persistent runtime state schema in src/storage/state.rs
- [X] T013 [P] Implement the recovery and health event journal in src/storage/events.rs
- [X] T014 [P] Implement install metadata persistence in src/storage/install.rs
- [X] T015 [P] Implement macOS network inspection, snapshot, and restore primitives in src/platform/macos.rs
- [X] T016 [P] Implement bundled blocklist loading and integrity validation in src/blocking/blocklist.rs
- [X] T017 [P] Implement DNS resolution and precise block matching in src/blocking/resolver.rs
- [X] T018 Implement the local blocking runtime lifecycle and health model in src/blocking/runtime.rs
- [X] T019 Implement safety check orchestration and rollback gates in src/control/safety.rs
- [X] T020 Implement shared recovery primitives and snapshot validation in src/control/recovery.rs

**Checkpoint**: Foundation ready. Interactive UX, installer, and recovery stories can proceed.

---

## Phase 3: User Story 1 - Activar protección desde una experiencia guiada (Priority: P1) 🎯 MVP

**Goal**: Deliver a polished interactive Sentinel session that guides the user through safety checks and activation without flags.

**Independent Test**: Launch `sentinel`, navigate only with the keyboard, run safety checks, activate protection, and deactivate it again while preserving a valid network state.

### Validation for User Story 1

- [X] T021 [P] [US1] Add contract coverage for interactive entry, explicit confirmations, and blocked unsafe actions in tests/contract/interaction_contract.rs
- [X] T022 [P] [US1] Add an integration test for the guided activation and disable journey in tests/integration/interactive_activation.rs
- [X] T023 [P] [US1] Add terminal snapshot coverage for the home, safety, and active-state screens in tests/snapshot/home_and_activation.rs

### Implementation for User Story 1

- [X] T024 [P] [US1] Implement guided session state and action selection in src/tui/app_state.rs
- [X] T025 [P] [US1] Implement keyboard navigation and confirmation handling in src/tui/input.rs
- [X] T026 [P] [US1] Implement terminal theme tokens and layout primitives in src/tui/theme.rs
- [X] T027 [US1] Implement the home, safety, and activation screens in src/tui/screens.rs
- [X] T028 [US1] Implement activation and deactivation control flow in src/control/activation.rs
- [X] T029 [US1] Wire the interactive event loop to safety checks and runtime actions in src/app.rs
- [X] T030 [US1] Persist activation outcomes and session-visible status summaries in src/storage/state.rs

**Checkpoint**: User Story 1 is independently functional as the MVP guided activation flow.

---

## Phase 4: User Story 2 - Instalar, actualizar o reinstalar Sentinel con un solo script (Priority: P2)

**Goal**: Deliver one official shell script that installs Sentinel into the `PATH` and safely decides whether to install, update, or reinstall.

**Independent Test**: Run the installer on a clean machine, then rerun it over an existing install and verify the executable remains callable as `sentinel` after install, update, or reinstall decisions.

### Validation for User Story 2

- [X] T031 [P] [US2] Add contract coverage for install, update, and reinstall semantics in tests/contract/install_script_contract.rs
- [X] T032 [P] [US2] Add an integration test for first-time installation into the `PATH` in tests/integration/install_fresh.rs
- [X] T033 [P] [US2] Add an integration test for update and reinstall decision paths in tests/integration/install_existing.rs

### Implementation for User Story 2

- [X] T034 [P] [US2] Implement version discovery and target comparison logic in src/install/version.rs
- [X] T035 [US2] Implement install state inspection and action selection in src/storage/install.rs
- [X] T036 [US2] Implement the official install, update, and reinstall flow in scripts/install-sentinel.sh
- [X] T037 [US2] Surface install and update state inside the interactive status flow in src/tui/screens.rs

**Checkpoint**: User Story 2 is independently functional with a single lifecycle script and visible install state.

---

## Phase 5: User Story 3 - Confiar en el estado y la seguridad del bloqueo (Priority: P3)

**Goal**: Deliver clear status, degraded-health handling, and guided recovery so the user can trust Sentinel on a real device.

**Independent Test**: Put Sentinel into an active state, trigger a recoverable safety problem, verify the UI surfaces the risk clearly, and restore a valid network state from the interactive recovery flow.

### Validation for User Story 3

- [X] T038 [P] [US3] Add contract coverage for degraded health and recovery-first guidance in tests/contract/recovery_contract.rs
- [X] T039 [P] [US3] Add an integration test for failed safety checks and safe-stop behavior in tests/integration/safety_failures.rs
- [X] T040 [P] [US3] Add an integration test for recovery snapshot restoration after interrupted changes in tests/integration/recovery_flow.rs
- [X] T041 [P] [US3] Add terminal snapshot coverage for degraded and recovery screens in tests/snapshot/recovery_and_status.rs

### Implementation for User Story 3

- [X] T042 [US3] Persist safety check history and recommended actions in src/control/safety.rs
- [X] T043 [US3] Implement guided recovery confirmation and restore flow in src/control/recovery.rs
- [X] T044 [US3] Expand status, degraded, and recovery screens in src/tui/screens.rs
- [X] T045 [US3] Record recovery events and health timeline entries in src/storage/events.rs

**Checkpoint**: User Story 3 is independently functional with health visibility and guided recovery.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Final hardening, clarity, and end-to-end validation across all stories.

- [X] T046 [P] Tighten terminal error presentation and fallback exit messaging in src/app.rs
- [X] T047 [P] Add essential Rustdoc comments to user-critical control flows in src/control/activation.rs
- [X] T048 [P] Review and trim non-essential interactive actions from the final menus in src/tui/screens.rs
- [X] T049 [P] Update operator validation steps and installer guidance in specs/003-interactive-cli-installer/quickstart.md
- [X] T050 Add end-to-end regression coverage for install plus interactive startup in tests/integration/end_to_end_cli.rs

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1: Setup** has no dependencies and starts immediately.
- **Phase 2: Foundational** depends on Phase 1 and blocks every user story.
- **Phase 3: US1** depends on Phase 2 and delivers the MVP interactive activation flow.
- **Phase 4: US2** depends on Phase 2 and can proceed after the foundations are ready; it integrates with the shared interactive surfaces built for US1.
- **Phase 5: US3** depends on Phase 2 and uses the runtime, safety, and UI foundations already established.
- **Phase 6: Polish** depends on the completion of the user stories included in the release.

### User Story Dependencies

- **US1 (P1)**: No dependency on other user stories once the foundational phase is complete.
- **US2 (P2)**: No blocking dependency on US1 for the installer flow, but it reuses shared UI surfaces when exposing install state.
- **US3 (P3)**: No blocking dependency on US2; it builds directly on the shared runtime, safety, and recovery primitives from Phase 2.

### Within Each User Story

- Validation tasks must exist before the story is considered complete.
- Session and data models come before orchestration that depends on them.
- Runtime or installer control flows come before the screens that report their final state.
- The story must pass its independent test before moving to the next release increment.

### Parallel Opportunities

- Setup tasks `T003` through `T010` can run in parallel after `T001` and `T002`.
- Foundational tasks `T012` through `T017` can run in parallel once the module skeleton exists.
- US1 tasks `T021`, `T022`, and `T023` can run in parallel, as can `T024`, `T025`, and `T026`.
- US2 tasks `T031`, `T032`, and `T033` can run in parallel, followed by `T034` and `T035`.
- US3 tasks `T038` through `T041` can run in parallel before the recovery implementation sequence.
- Polish tasks `T046` through `T049` can run in parallel after the chosen release stories are complete.

---

## Parallel Example: User Story 1

```bash
Task: "Add contract coverage for interactive entry, explicit confirmations, and blocked unsafe actions in tests/contract/interaction_contract.rs"
Task: "Add an integration test for the guided activation and disable journey in tests/integration/interactive_activation.rs"
Task: "Add terminal snapshot coverage for the home, safety, and active-state screens in tests/snapshot/home_and_activation.rs"

Task: "Implement guided session state and action selection in src/tui/app_state.rs"
Task: "Implement keyboard navigation and confirmation handling in src/tui/input.rs"
Task: "Implement terminal theme tokens and layout primitives in src/tui/theme.rs"
```

---

## Parallel Example: User Story 2

```bash
Task: "Add contract coverage for install, update, and reinstall semantics in tests/contract/install_script_contract.rs"
Task: "Add an integration test for first-time installation into the `PATH` in tests/integration/install_fresh.rs"
Task: "Add an integration test for update and reinstall decision paths in tests/integration/install_existing.rs"

Task: "Implement version discovery and target comparison logic in src/install/version.rs"
Task: "Implement install state inspection and action selection in src/storage/install.rs"
```

---

## Parallel Example: User Story 3

```bash
Task: "Add contract coverage for degraded health and recovery-first guidance in tests/contract/recovery_contract.rs"
Task: "Add an integration test for failed safety checks and safe-stop behavior in tests/integration/safety_failures.rs"
Task: "Add an integration test for recovery snapshot restoration after interrupted changes in tests/integration/recovery_flow.rs"
Task: "Add terminal snapshot coverage for degraded and recovery screens in tests/snapshot/recovery_and_status.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup.
2. Complete Phase 2: Foundational.
3. Complete Phase 3: User Story 1.
4. Validate the guided activation flow and network safety behavior before expanding scope.

### Incremental Delivery

1. Deliver the interactive activation MVP from US1.
2. Add the installer lifecycle flow from US2.
3. Add degraded-state visibility and recovery from US3.
4. Finish with cross-cutting polish and end-to-end validation.

### Parallel Team Strategy

1. One developer completes crate and module setup while another curates the bundled blocklist asset.
2. After Phase 2, the installer flow and recovery flow can progress alongside the interactive UI refinements.
3. Keep tasks that modify the same file sequential even if their surrounding phase has other `[P]` work available.

---

## Notes

- `[P]` tasks touch different files and can be executed concurrently when dependencies are met.
- Story labels maintain traceability from spec to plan to implementation.
- Safety and recovery tests are part of the deliverable, not optional cleanup.
- The release scope stays intentionally narrow: guided interaction, safe blocking, visible status, recovery, and one installer script.
