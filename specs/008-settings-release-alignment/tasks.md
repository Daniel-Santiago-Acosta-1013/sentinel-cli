# Tasks: Ajustes de Bloqueo y Release Alineado

**Input**: Design documents from `/Users/santiagoacosta/Documents/personal-projects/sentinel-cli/specs/008-settings-release-alignment/`
**Prerequisites**: `plan.md` and `spec.md` required; `research.md`, `data-model.md`, `contracts/`, and `quickstart.md` used for task derivation

**Tests**: This feature explicitly requires contract, integration, snapshot, CLI UX, release workflow, and version-alignment validation coverage.

**Organization**: Tasks are grouped by user story so each story can be implemented and tested independently.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel with other tasks in the same phase when they touch different files and do not depend on incomplete work
- **[Story]**: Maps the task to a user story from `spec.md` (`[US1]`, `[US2]`, `[US3]`, `[US4]`)
- Every task includes an exact file path and is written so an implementation agent can execute it directly

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Create the scaffolding required for the new settings flow, managed blocked-domain persistence, and centralized version-alignment workflow.

- [X] T001 Update shared module and test registrations for the feature in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/src/storage/mod.rs and /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/tests/contract.rs and /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/tests/integration.rs and /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/tests/snapshot.rs
- [X] T002 [P] Extend route and action scaffolding for `Ajustes` and `Dominios bloqueados` in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/src/cli/navigation.rs
- [X] T003 [P] Create the managed blocked-domain store scaffold in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/src/storage/blocked_domains.rs
- [X] T004 [P] Create the centralized version-alignment script scaffold in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/scripts/release/update_versions.sh
- [X] T005 [P] Prepare workflow-dispatch inputs and staged job outputs for centralized release execution in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/.github/workflows/release.yml

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Build the shared persistence, validation, activity-tracking, and release-alignment primitives required by all user stories.

**⚠️ CRITICAL**: No user story work should start until this phase is complete.

- [X] T006 Implement shared domain normalization and validation helpers for managed blocked domains in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/src/app.rs and /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/src/core/rules.rs
- [X] T007 [P] Wire managed blocked-domain paths, load/save behavior, and seeding from the current local blocklist into /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/src/app.rs and /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/src/storage/blocked_domains.rs
- [X] T008 [P] Extend blocking rule resolution to consume the managed blocked-domain catalog instead of the fixed bundled-only list in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/src/core/rules.rs and /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/src/engine/dns.rs
- [X] T009 [P] Extend the event store with structured block-activity records and aggregation helpers in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/src/storage/events.rs
- [X] T010 [P] Define the reusable version-surface inventory and alignment helpers in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/scripts/release/common.sh and /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/scripts/release/update_versions.sh
- [X] T011 Define shared CLI copy, empty states, and operator guidance for settings, blocked-domain CRUD, activity tables, and release blocking in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/src/cli/copy.rs

**Checkpoint**: Foundation ready. User stories can now be implemented in priority order with isolated validation.

---

## Phase 3: User Story 1 - Entrar a un home más limpio sin perder orientación (Priority: P1) 🎯 MVP

**Goal**: Deliver a cleaner home that removes the risk badge, preserves the current visual line, and exposes `Ajustes` as a first-class top-level entry.

**Independent Test**: In a terminal or scripted transcript, start Sentinel and verify that the home no longer shows `✓ Riesgo: Normal`, still feels visually consistent, and shows `Ajustes` as a principal option with predictable back navigation.

### Validation for User Story 1

- [X] T012 [P] [US1] Update the interaction contract for a clean home and visible `Ajustes` entry in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/tests/contract/interaction_contract.rs
- [X] T013 [P] [US1] Refresh the home snapshot to remove the risk badge and include `Ajustes` in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/tests/snapshot/home_and_activation.rs

### Implementation for User Story 1

- [X] T014 [P] [US1] Remove the home risk badge and add the `Ajustes` menu entry in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/src/cli/views.rs
- [X] T015 [P] [US1] Add home and settings-shell labels, descriptions, and footer hints in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/src/cli/copy.rs
- [X] T016 [US1] Wire the home menu ordering and settings-shell route into session state in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/src/cli/menu_state.rs
- [X] T017 [US1] Implement route transitions between `Home` and the `Ajustes` shell in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/src/app.rs and /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/src/cli/navigation.rs

**Checkpoint**: User Story 1 is complete when Sentinel opens into the new clean home and the top-level `Ajustes` branch is visible and reachable without implementing blocked-domain CRUD yet.

---

## Phase 4: User Story 2 - Administrar dominios bloqueados desde Ajustes (Priority: P1)

**Goal**: Let users list, add, edit, and delete blocked domains from `Ajustes` while preserving the existing menu-driven UX and keeping the managed catalog consistent.

**Independent Test**: Navigate to `Ajustes > Dominios bloqueados`, verify the current list or empty state, add a valid domain, edit it, delete it, and confirm the list updates immediately without breaking navigation or corrupting the managed catalog.

### Validation for User Story 2

- [X] T018 [P] [US2] Extend interaction contract coverage for blocked-domain listing and CRUD flows in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/tests/contract/interaction_contract.rs
- [X] T019 [P] [US2] Add scripted end-to-end coverage for `Ajustes > Dominios bloqueados` CRUD journeys in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/tests/integration/end_to_end_cli.rs
- [X] T020 [P] [US2] Add snapshot coverage for blocked-domain list, editor state, and empty state in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/tests/snapshot/settings_and_blocklist.rs

### Implementation for User Story 2

- [X] T021 [P] [US2] Implement blocked-domain catalog load/save, dedupe, and edit operations in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/src/storage/blocked_domains.rs
- [X] T022 [P] [US2] Add blocked-domain list and editor renderers for the settings branch in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/src/cli/views.rs and /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/src/cli/output.rs
- [X] T023 [P] [US2] Add menu-state selection rules and action descriptors for blocked-domain CRUD in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/src/cli/menu_state.rs
- [X] T024 [US2] Implement add, edit, and delete handlers with validation and atomic persistence in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/src/app.rs
- [X] T025 [US2] Apply the managed blocked-domain catalog to the effective blocking rules used by the runtime in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/src/core/rules.rs and /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/src/engine/dns.rs

**Checkpoint**: User Story 2 is complete when users can fully manage the active blocked-domain list from inside Sentinel and those changes are reflected in the effective block catalog.

---

## Phase 5: User Story 4 - Publicar versiones totalmente alineadas desde el flujo oficial (Priority: P1)

**Goal**: Centralize version creation in GitHub Actions so the workflow receives a target version, aligns all version surfaces, creates an auditable commit and tag, and only then performs release/deploy from that exact state.

**Independent Test**: Trigger the official workflow with a stable version input and verify that it updates every required version surface, produces a commit, creates the matching tag, and then runs release/deploy in a separate job that blocks if any surface or channel is misaligned.

### Validation for User Story 4

- [X] T026 [P] [US4] Extend release contract coverage for workflow-dispatch inputs and pre-release version alignment in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/tests/contract/release_automation_contract.rs
- [X] T027 [P] [US4] Extend release integration coverage for version commit, tag creation, and two-job release execution in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/tests/integration/release_pipeline.rs

### Implementation for User Story 4

- [X] T028 [P] [US4] Implement version-surface discovery, update, and validation logic in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/scripts/release/update_versions.sh
- [X] T029 [P] [US4] Extend shared release helpers to resolve aligned versions, generated commits, and created tags in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/scripts/release/common.sh and /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/scripts/release/resolve_version.sh
- [X] T030 [P] [US4] Update release fixtures for workflow-dispatch inputs and aligned-surface assertions in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/tests/support/release_fixtures.rs
- [X] T031 [US4] Replace tag-push triggering with workflow-dispatch version input and a dedicated versioning job in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/.github/workflows/release.yml
- [X] T032 [US4] Implement the auditable version-commit and tag-creation path before release/deploy in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/.github/workflows/release.yml and /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/scripts/release/update_versions.sh
- [X] T033 [US4] Update build and publication scripts to consume the CI-created version/tag state and block on any alignment mismatch in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/scripts/release/build_release_artifacts.sh and /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/scripts/release/publish_github_release.sh and /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/scripts/release/publish_npm.sh and /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/scripts/release/publish_homebrew.sh and /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/scripts/release/inspect_release_state.sh and /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/scripts/release/summarize_release.sh

**Checkpoint**: User Story 4 is complete when the official CI workflow is the only supported path to create a new version and it guarantees alignment between repo state, tag, artifacts, npm, and Homebrew.

---

## Phase 6: User Story 3 - Revisar estado y actividad real de bloqueo (Priority: P2)

**Goal**: Simplify `Estado de Sentinel` by removing low-signal columns and add a dedicated `Actividad de bloqueo` table driven by persisted block events.

**Independent Test**: Open `Estado de Sentinel` after generating block activity and verify that the main table omits `Riesgo`, `Resumen`, and `Accion sugerida`, while the new activity table shows only the four required metrics and handles no-data states clearly.

### Validation for User Story 3

- [X] T034 [P] [US3] Extend interaction contract coverage for the trimmed status table and `Actividad de bloqueo` section in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/tests/contract/interaction_contract.rs
- [X] T035 [P] [US3] Refresh status snapshot coverage for the simplified table and new activity section in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/tests/snapshot/recovery_and_status.rs
- [X] T036 [P] [US3] Add scripted integration coverage for block-activity metrics and no-data behavior in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/tests/integration/interactive_activation.rs

### Implementation for User Story 3

- [X] T037 [P] [US3] Implement block-activity aggregation and top-domain ranking from persisted events in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/src/storage/events.rs
- [X] T038 [P] [US3] Remove `Riesgo`, `Resumen`, and `Accion sugerida` from the main status table in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/src/cli/output.rs
- [X] T039 [P] [US3] Add the `Actividad de bloqueo` table renderer and empty-state handling in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/src/cli/output.rs and /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/src/cli/views.rs
- [X] T040 [P] [US3] Emit structured block-activity events from DNS blocking paths in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/src/engine/dns.rs and /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/src/engine/runtime.rs
- [X] T041 [US3] Wire activity snapshot loading and status-route presentation into /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/src/app.rs and /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/src/cli/menu_state.rs

**Checkpoint**: User Story 3 is complete when `Estado de Sentinel` is more focused and the user can inspect real blocking activity directly from the status flow.

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Finish documentation, regression alignment, and full-feature validation across CLI UX and centralized release.

- [X] T042 [P] Update the feature validation guide with final CLI and release verification steps in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/specs/008-settings-release-alignment/quickstart.md
- [X] T043 [P] Refresh maintainer-facing documentation for centralized versioning and settings navigation in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/README.md and /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/packaging/npm/README.md
- [X] T044 Review and trim cross-story copy, errors, and next-step guidance in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/src/cli/copy.rs and /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/scripts/release/summarize_release.sh
- [X] T045 [P] Refresh implementation notes, release contract details, and deferred-scope references in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/specs/008-settings-release-alignment/plan.md and /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/specs/008-settings-release-alignment/contracts/release-versioning-workflow.md
- [X] T046 Run the full validation checklist from /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/specs/008-settings-release-alignment/quickstart.md using /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/tests/contract.rs and /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/tests/integration.rs and /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/tests/snapshot.rs

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1: Setup**: No dependencies. Start immediately.
- **Phase 2: Foundational**: Depends on Phase 1. Blocks every user story.
- **Phase 3: User Story 1**: Depends on Phase 2. Recommended MVP starting point for visible CLI value.
- **Phase 4: User Story 2**: Depends on Phase 2 and integrates most cleanly after US1 exposes the `Ajustes` branch.
- **Phase 5: User Story 4**: Depends on Phase 2 and can proceed independently of the CLI stories once shared release primitives are ready.
- **Phase 6: User Story 3**: Depends on Phase 2 and benefits from US2 because activity visibility is most useful after blocked-domain management is in place.
- **Phase 7: Polish**: Depends on all targeted user stories being complete.

### User Story Dependency Graph

- **US1**: Start after Foundational. No dependency on other stories.
- **US2**: Start after Foundational, but should follow US1 because it extends the new `Ajustes` navigation branch.
- **US4**: Start after Foundational. No dependency on CLI stories.
- **US3**: Start after Foundational, but should follow US2 because it presents activity around the managed blocked-domain experience.

Recommended completion order:

1. **US1**
2. **US2**
3. **US4**
4. **US3**

### Within Each User Story

- Validation tasks should be updated before the corresponding implementation is considered done.
- Store and event primitives should exist before app handlers and view wiring are finalized.
- View renderers should exist before session wiring and transcript assertions are finalized.
- The version-alignment script should exist before the workflow and publish scripts consume its outputs.
- Each story should pass its independent test before moving to the next priority increment.

## Parallel Opportunities

- **Setup**: `T002`, `T003`, `T004`, and `T005` can run in parallel after `T001`.
- **Foundational**: `T007`, `T008`, `T009`, and `T010` can run in parallel after `T006`; `T011` then consolidates user-facing copy.
- **US1**: `T012`, `T013`, `T014`, and `T015` can run in parallel; `T016-T017` then wire session behavior.
- **US2**: `T018`, `T019`, `T020`, `T021`, `T022`, and `T023` can run in parallel; `T024-T025` then finalize behavior and runtime usage.
- **US4**: `T026`, `T027`, `T028`, `T029`, and `T030` can run in parallel; `T031-T033` then integrate the workflow and aligned publication path.
- **US3**: `T034`, `T035`, `T036`, `T037`, `T038`, `T039`, and `T040` can run in parallel; `T041` then wires the final status presentation.
- **Polish**: `T042`, `T043`, and `T045` can run in parallel before `T044` and `T046`.

## Parallel Example: User Story 1

```bash
Task T012: Update the interaction contract for a clean home in tests/contract/interaction_contract.rs
Task T013: Refresh the home snapshot in tests/snapshot/home_and_activation.rs
Task T014: Remove the home risk badge and add Ajustes in src/cli/views.rs
Task T015: Add home and settings-shell copy in src/cli/copy.rs
```

## Parallel Example: User Story 2

```bash
Task T018: Extend blocked-domain interaction coverage in tests/contract/interaction_contract.rs
Task T019: Add end-to-end CRUD coverage in tests/integration/end_to_end_cli.rs
Task T020: Add blocked-domain snapshots in tests/snapshot/settings_and_blocklist.rs
Task T021: Implement blocked-domain catalog operations in src/storage/blocked_domains.rs
Task T022: Add blocked-domain renderers in src/cli/views.rs and src/cli/output.rs
Task T023: Add CRUD menu-state actions in src/cli/menu_state.rs
```

## Parallel Example: User Story 4

```bash
Task T026: Extend release contract coverage in tests/contract/release_automation_contract.rs
Task T027: Extend release integration coverage in tests/integration/release_pipeline.rs
Task T028: Implement version-surface discovery in scripts/release/update_versions.sh
Task T029: Extend shared release helpers in scripts/release/common.sh and scripts/release/resolve_version.sh
Task T030: Update workflow-dispatch release fixtures in tests/support/release_fixtures.rs
```

## Parallel Example: User Story 3

```bash
Task T034: Extend status interaction coverage in tests/contract/interaction_contract.rs
Task T035: Refresh status snapshots in tests/snapshot/recovery_and_status.rs
Task T036: Add status activity integration coverage in tests/integration/interactive_activation.rs
Task T037: Implement activity aggregation in src/storage/events.rs
Task T038: Trim the main status table in src/cli/output.rs
Task T040: Emit structured block-activity events in src/engine/dns.rs and src/engine/runtime.rs
```

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1.
2. Complete Phase 2.
3. Complete Phase 3 (US1).
4. Validate the independent home-entry test for US1.
5. Demo the cleaner home before continuing.

### Incremental Delivery

1. Build the persistence, validation, and release-alignment foundation once.
2. Ship the cleaner home and `Ajustes` shell (US1).
3. Add full blocked-domain management (US2).
4. In parallel or next, centralize version creation and release in CI (US4).
5. Add the focused status/activity view (US3).
6. Finish with documentation and full regression validation.

### Suggested MVP Scope

- **MVP**: Phase 1 + Phase 2 + Phase 3 (US1 only)
- **Next increment**: Phase 4 (US2)
- **Parallel maintainer increment**: Phase 5 (US4)
- **Final product increment**: Phase 6 + Phase 7 (US3 and polish)

## Notes

- All tasks use the required checklist format.
- Tasks marked `[P]` touch different files or can be completed as non-blocking work after their prerequisites.
- Each user story retains explicit validation tasks and an independent test criterion.
- The task order intentionally preserves the menu-driven CLI UX while letting the release workflow evolve independently after the shared foundation is ready.
