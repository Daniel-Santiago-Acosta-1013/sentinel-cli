# Tasks: Rediseño de CLI por Vistas

**Input**: Design documents from `/Users/santiagoacosta/Documents/personal-projects/sentinel-cli/specs/006-menu-cli-redesign/`
**Prerequisites**: `plan.md` and `spec.md` required; `research.md`, `data-model.md`, `contracts/`, and `quickstart.md` used for task derivation

**Tests**: This feature explicitly requires contract, integration, snapshot, CLI UX, and safety regression coverage.

**Organization**: Tasks are grouped by user story so each story can be implemented and validated independently.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel with other tasks in the same phase when they touch different files and do not depend on incomplete work
- **[Story]**: Maps the task to a user story from `spec.md` (`[US1]`, `[US2]`, `[US3]`)
- Every task includes an exact file path and is written so an implementation agent can execute it directly

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Create the module entrypoints and scaffolding for the CLI-only redesign.

- [X] T001 Update module exports for the CLI redesign in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/src/cli/mod.rs
- [X] T002 [P] Create the route and transition scaffolding for CLI views in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/src/cli/navigation.rs
- [X] T003 [P] Create the dedicated view-frame scaffolding for home, action, status, confirmation, and result screens in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/src/cli/views.rs
- [X] T004 [P] Create the progress feedback scaffolding for long-running operations in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/src/cli/spinner.rs

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Build the shared navigation, terminal, and transcript infrastructure that every story depends on.

**⚠️ CRITICAL**: No user story work should start until this phase is complete.

- [X] T005 Replace the current single-screen session model with route-aware navigation state in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/src/cli/menu_state.rs
- [X] T006 Refactor the interactive app loop to drive explicit route transitions instead of a single reactive screen in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/src/app.rs
- [X] T007 [P] Extend terminal control for screen clearing, terminal capability detection, and result persistence in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/src/cli/terminal.rs
- [X] T008 [P] Refactor reusable table and section helpers for dedicated CLI views in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/src/cli/output.rs
- [X] T009 [P] Expand shared route titles, navigation hints, and result copy for all redesigned views in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/src/cli/copy.rs
- [X] T010 [P] Update scripted interaction helpers for route-based transcripts and progress assertions in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/tests/support/mod.rs

**Checkpoint**: Foundation ready. User stories can now be implemented in priority order with isolated validation.

---

## Phase 3: User Story 1 - Entrar a un home limpio y orientado a acciones (Priority: P1) 🎯 MVP

**Goal**: Deliver a minimal, branded, easy-to-read home that acts as the main navigation hub and never shows residual text from prior views.

**Independent Test**: Run the scripted home flow and verify that startup lands on a clean home with the embedded ASCII logo, a short action list, and no leftover content from any previous view.

### Validation for User Story 1

- [X] T011 [P] [US1] Update home interaction assertions for logo, minimal menu, and clean startup in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/tests/contract/interaction_contract.rs
- [X] T012 [P] [US1] Refresh the home snapshot expectations for a focused landing view in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/tests/snapshot/home_and_activation.rs

### Implementation for User Story 1

- [X] T013 [P] [US1] Embed the approved ASCII logo and expose it for the CLI home in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/src/cli/logo.rs
- [X] T014 [P] [US1] Implement the dedicated home frame with minimal sections and action menu layout in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/src/cli/views.rs
- [X] T015 [US1] Wire the home route, action list, and clean entry state in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/src/cli/menu_state.rs
- [X] T016 [US1] Connect interactive startup to the redesigned home view in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/src/app.rs
- [X] T017 [US1] Apply restrained branding and home-specific navigation hints in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/src/cli/copy.rs

**Checkpoint**: User Story 1 is complete when Sentinel opens directly into the new home and that experience can be validated without implementing the rest of the navigation redesign.

---

## Phase 4: User Story 2 - Navegar por vistas independientes como en un CLI tradicional (Priority: P1)

**Goal**: Make every home action open a distinct view or result screen, with explicit back/continue paths and screen clearing between transitions.

**Independent Test**: Run scripted navigation through home, status, safety, recovery, confirmation, and exit flows, and verify that each action opens its own view, the terminal is cleared on transition, and back navigation returns to a recognizable location.

### Validation for User Story 2

- [X] T018 [P] [US2] Extend navigation contract coverage for per-action routes, back behavior, and confirmation screens in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/tests/contract/interaction_contract.rs
- [X] T019 [P] [US2] Add end-to-end navigation coverage for moving between dedicated views in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/tests/integration/interactive_activation.rs
- [X] T020 [P] [US2] Refresh snapshot coverage for status, safety, recovery, and result screens in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/tests/snapshot/recovery_and_status.rs

### Implementation for User Story 2

- [X] T021 [P] [US2] Map every home action to its own route and back-stack behavior in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/src/cli/navigation.rs
- [X] T022 [P] [US2] Implement dedicated frames for safety, status, installation, confirmation, recovery, and final-result views in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/src/cli/views.rs
- [X] T023 [US2] Refactor action handlers to return route-specific results instead of mutating a single shared screen in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/src/app.rs
- [X] T024 [US2] Isolate confirmation copy, exit copy, and per-view next-step guidance in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/src/cli/copy.rs
- [X] T025 [US2] Enforce screen clearing on every route transition while preserving final messages until manual navigation in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/src/cli/terminal.rs

**Checkpoint**: User Story 2 is complete when every main action behaves like a separate CLI view and the user can move forward or backward without losing orientation.

---

## Phase 5: User Story 3 - Ejecutar acciones con una estética CLI coherente sin perder funcionalidad (Priority: P2)

**Goal**: Layer professional CLI polish onto the new view-based flow using ANSI styling, Unicode symbols, spinners, and fallback-safe rendering while removing the legacy TUI stack.

**Independent Test**: Run long or sensitive scripted flows and verify that progress becomes visible, results are visually distinguishable, the output remains readable without advanced terminal features, and no TUI code path remains in the product.

### Validation for User Story 3

- [X] T026 [P] [US3] Extend recovery and long-running flow assertions for progress cues and readable fallback output in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/tests/contract/recovery_contract.rs
- [X] T027 [P] [US3] Add integration coverage for spinner/progress behavior and final guidance during slow operations in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/tests/integration/recovery_flow.rs
- [X] T028 [P] [US3] Refresh snapshot coverage for styled badges, result emphasis, and fallback-safe output in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/tests/snapshot/recovery_and_status.rs

### Implementation for User Story 3

- [X] T029 [P] [US3] Implement ANSI styling, Unicode symbols, and plain-text fallbacks in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/src/cli/styles.rs
- [X] T030 [P] [US3] Implement spinner lifecycle and transcript-safe progress rendering in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/src/cli/spinner.rs
- [X] T031 [US3] Apply styled headers, badges, and progress blocks across the dedicated CLI views in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/src/cli/views.rs
- [X] T032 [US3] Remove the legacy TUI module tree under /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/src/tui/ and delete the `ratatui` dependency from /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/Cargo.toml
- [X] T033 [US3] Remove any remaining TUI-only interactive path so the pure CLI flow is the sole interactive mode in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/src/app.rs

**Checkpoint**: User Story 3 is complete when the CLI feels polished and modern, degrades safely, and no full-screen TUI implementation remains.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Final cleanup, documentation alignment, and regression validation across all stories.

- [X] T034 [P] Refresh the implementation validation guide for the redesigned CLI in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/specs/006-menu-cli-redesign/quickstart.md
- [X] T035 [P] Update delivery and acceptance notes after implementation details settle in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/specs/006-menu-cli-redesign/plan.md
- [X] T036 Review and trim any remaining non-essential terminal noise across shared copy in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/src/cli/copy.rs
- [X] T037 Verify test module registrations remain correct for the redesigned CLI suites in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/tests/contract.rs
- [X] T038 Verify test module registrations remain correct for the redesigned CLI suites in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/tests/integration.rs
- [X] T039 Verify test module registrations remain correct for the redesigned CLI suites in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/tests/snapshot.rs
- [X] T040 Run the full regression checklist and record any follow-up fixes from /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/specs/006-menu-cli-redesign/quickstart.md

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1: Setup**: No dependencies. Start immediately.
- **Phase 2: Foundational**: Depends on Phase 1. Blocks every user story.
- **Phase 3: User Story 1**: Depends on Phase 2. Recommended MVP starting point.
- **Phase 4: User Story 2**: Depends on Phase 2 and benefits from the home shell delivered in Phase 3.
- **Phase 5: User Story 3**: Depends on Phase 2 and should be layered after the dedicated views from Phases 3-4 exist.
- **Phase 6: Polish**: Depends on all targeted user stories being complete.

### User Story Dependency Graph

- **US1**: Start after Foundational. No dependency on later stories.
- **US2**: Start after Foundational. Integrates most cleanly after US1 establishes the final home entry point.
- **US3**: Start after Foundational, but should be applied after US1 and US2 define the final view surfaces.

Recommended completion order:

1. **US1**
2. **US2**
3. **US3**

### Within Each User Story

- Validation tasks should be updated before the corresponding implementation is considered done.
- Shared models/state changes should land before route handlers and final rendering changes.
- View rendering should be in place before app wiring is finalized.
- Terminal behavior and copy changes should be verified against each story’s independent test before moving on.

## Parallel Opportunities

- **Setup**: `T002`, `T003`, and `T004` can run in parallel after `T001`.
- **Foundational**: `T007`, `T008`, `T009`, and `T010` can run in parallel once `T005` and `T006` define the shared direction.
- **US1**: `T011`, `T012`, `T013`, and `T014` can run in parallel; `T015-T017` then integrate the results.
- **US2**: `T018`, `T019`, `T020`, and `T021` can run in parallel; `T022-T025` then wire and stabilize the flow.
- **US3**: `T026`, `T027`, `T028`, `T029`, and `T030` can run in parallel; `T031-T033` then finalize polish and TUI removal.
- **Polish**: `T034`, `T035`, `T037`, `T038`, and `T039` can run in parallel before `T040`.

## Parallel Example: User Story 1

```bash
Task T011: Update home interaction assertions in tests/contract/interaction_contract.rs
Task T012: Refresh the home snapshot in tests/snapshot/home_and_activation.rs
Task T013: Embed the approved ASCII logo in src/cli/logo.rs
Task T014: Implement the dedicated home frame in src/cli/views.rs
```

## Parallel Example: User Story 2

```bash
Task T018: Extend navigation contract coverage in tests/contract/interaction_contract.rs
Task T019: Add end-to-end navigation coverage in tests/integration/interactive_activation.rs
Task T020: Refresh dedicated view snapshots in tests/snapshot/recovery_and_status.rs
Task T021: Map home actions to route transitions in src/cli/navigation.rs
```

## Parallel Example: User Story 3

```bash
Task T026: Extend recovery progress assertions in tests/contract/recovery_contract.rs
Task T027: Add spinner/progress integration coverage in tests/integration/recovery_flow.rs
Task T029: Implement ANSI styling and fallbacks in src/cli/styles.rs
Task T030: Implement spinner lifecycle in src/cli/spinner.rs
```

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1.
2. Complete Phase 2.
3. Complete Phase 3 (US1).
4. Validate the independent home-entry test for US1.
5. Demo the new home before continuing with the rest of the navigation redesign.

### Incremental Delivery

1. Build the route-based foundation once.
2. Ship the new home (US1) as the first visible improvement.
3. Add independent views and back navigation (US2).
4. Layer styling, spinners, and TUI removal last (US3).
5. Finish with regression cleanup and documentation alignment.

### Suggested MVP Scope

- **MVP**: Phase 1 + Phase 2 + Phase 3 (US1 only)
- **Next increment**: Phase 4 (US2)
- **Final increment**: Phase 5 + Phase 6 (US3 and polish)

## Notes

- All tasks use the required checklist format.
- Tasks marked `[P]` touch different files or can be executed as non-blocking work after their prerequisites.
- Each user story retains its own validation tasks and independent test criteria.
- The plan intentionally delays TUI removal until the pure CLI views are already functional.
