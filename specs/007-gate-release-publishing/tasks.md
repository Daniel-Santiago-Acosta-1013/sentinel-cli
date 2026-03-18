# Tasks: Automatización Segura de Releases

**Input**: Design documents from `/Users/santiagoacosta/Documents/personal-projects/sentinel-cli/specs/007-gate-release-publishing/`
**Prerequisites**: `plan.md` and `spec.md` required; `research.md`, `data-model.md`, `contracts/`, and `quickstart.md` used for task derivation

**Tests**: This feature explicitly requires contract, integration, CI/CD gating, dual-channel consistency, retry safety, and post-release verification coverage.

**Organization**: Tasks are grouped by user story so each release behavior can be implemented and validated independently.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel with other tasks in the same phase when they touch different files and do not depend on incomplete work
- **[Story]**: Maps the task to a user story from `spec.md` (`[US1]`, `[US2]`, `[US3]`, `[US4]`)
- Every task includes an exact file path and is written so an implementation agent can execute it directly

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Create the release automation scaffolding, packaging skeletons, and test support files required by the feature plan.

- [X] T001 Create the release workflow scaffold with staged jobs and outputs in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/.github/workflows/release.yml
- [X] T002 [P] Create shared shell helpers for release scripts in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/scripts/release/common.sh
- [X] T003 [P] Create reusable release fixtures and mocked channel-state helpers in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/tests/support/release_fixtures.rs
- [X] T004 [P] Create the npm packaging skeleton for Sentinel releases in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/packaging/npm/package.json and /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/packaging/npm/README.md
- [X] T005 [P] Create the Homebrew formula template scaffold for canonical artifact publication in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/packaging/homebrew/sentinel.rb.tpl

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Build the shared release-state, inspection, and test registration infrastructure that every user story depends on.

**⚠️ CRITICAL**: No user story work should start until this phase is complete.

- [X] T006 Register the release contract suite in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/tests/contract.rs
- [X] T007 [P] Register the release integration suite in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/tests/integration.rs
- [X] T008 [P] Align the project version helper with CI release expectations in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/src/install/version.rs
- [X] T009 [P] Implement the shared release-state inspection baseline for GitHub Release, npm, and Homebrew in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/scripts/release/inspect_release_state.sh
- [X] T010 Define shared environment validation, artifact directory layout, and failure-exit conventions in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/scripts/release/common.sh

**Checkpoint**: Foundation ready. User stories can now be implemented in priority order with isolated validation.

---

## Phase 3: User Story 1 - Autorizar releases solo desde el HEAD vigente de main (Priority: P1) 🎯 MVP

**Goal**: Ensure that a release can only proceed when the triggering tag resolves exactly to the current HEAD of `main` and that any other state is blocked before publication.

**Independent Test**: Trigger the workflow with one tag pointing to `main@HEAD`, one tag pointing to an older `main` commit, and one tag pointing outside the current `main` HEAD; verify that only the first case advances past authorization and the other two stop before artifact generation or external publication.

### Validation for User Story 1

- [X] T011 [P] [US1] Add contract coverage for stable tag parsing, tag-to-commit resolution, and `main@HEAD` equality in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/tests/contract/release_automation_contract.rs
- [X] T012 [P] [US1] Add integration coverage for authorized and blocked tag scenarios in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/tests/integration/release_pipeline.rs

### Implementation for User Story 1

- [X] T013 [P] [US1] Implement stable tag validation and exact SHA resolution for the triggering ref in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/scripts/release/authorize_release.sh
- [X] T014 [P] [US1] Implement project-version extraction and exact tag/version comparison in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/scripts/release/resolve_version.sh
- [X] T015 [US1] Wire the authorization gate, blocked exit path, and zero-publication guarantee into /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/.github/workflows/release.yml
- [X] T016 [US1] Emit explicit blocked summaries with tag, commit, main HEAD, and reason fields in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/scripts/release/summarize_release.sh

**Checkpoint**: User Story 1 is complete when invalid tags are rejected before any external side effect and valid tags are proven to originate from the exact current `main` HEAD.

---

## Phase 4: User Story 2 - Publicar una versión consistente en todos los canales (Priority: P1)

**Goal**: Build one canonical release artifact set from the authorized source and publish the same version consistently to GitHub Release, npm, and Homebrew.

**Independent Test**: Run the workflow from an authorized tag whose version matches `Cargo.toml`; verify that the build produces a canonical manifest with checksums, GitHub Release is created first, and npm and Homebrew expose the exact same authorized version and artifact lineage.

### Validation for User Story 2

- [X] T017 [P] [US2] Extend contract coverage for source-of-truth rules, artifact-manifest consistency, and dual-channel version alignment in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/tests/contract/release_automation_contract.rs
- [X] T018 [P] [US2] Add integration coverage for successful canonical build and publication across GitHub Release, npm, and Homebrew in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/tests/integration/release_pipeline.rs

### Implementation for User Story 2

- [X] T019 [P] [US2] Build the canonical release manifest, artifact metadata, and checksums in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/scripts/release/build_release_artifacts.sh
- [X] T020 [P] [US2] Materialize the canonical GitHub Release with artifacts and checksums in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/scripts/release/publish_github_release.sh
- [X] T021 [P] [US2] Prepare npm package metadata and binary mapping for the authorized release version in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/packaging/npm/package.json
- [X] T022 [P] [US2] Publish npm packages only from the canonical artifact set in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/scripts/release/publish_npm.sh
- [X] T023 [P] [US2] Render the Homebrew formula from the canonical manifest with version and checksum fields in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/packaging/homebrew/sentinel.rb.tpl
- [X] T024 [P] [US2] Publish Homebrew updates only from the canonical artifact set in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/scripts/release/publish_homebrew.sh
- [X] T025 [US2] Connect canonical build, GitHub Release publication, npm publication, and Homebrew publication stages in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/.github/workflows/release.yml

**Checkpoint**: User Story 2 is complete when one authorized release produces one manifest-backed version that is visibly identical across GitHub Release, npm, and Homebrew.

---

## Phase 5: User Story 3 - Recuperar control ante fallos parciales y reintentos (Priority: P2)

**Goal**: Classify partial and already-materialized releases safely, prevent inconsistent re-publication, and make retries idempotent.

**Independent Test**: Force a run where one channel is published and a later channel fails, then re-run the same version; verify that the workflow detects the existing external state, marks the attempt as `partial` or `materialized` as appropriate, and never republishes blindly over incompatible evidence.

### Validation for User Story 3

- [X] T026 [P] [US3] Extend contract coverage for `materialized`, `partial`, and incompatible channel-state handling in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/tests/contract/release_automation_contract.rs
- [X] T027 [P] [US3] Add integration coverage for partial publication and safe retry scenarios in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/tests/integration/release_pipeline.rs

### Implementation for User Story 3

- [X] T028 [P] [US3] Extend channel-state inspection to classify absent, materialized, incompatible, and failed states in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/scripts/release/inspect_release_state.sh
- [X] T029 [P] [US3] Implement npm retry safety for existing, materialized, and incompatible versions in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/scripts/release/publish_npm.sh
- [X] T030 [P] [US3] Implement Homebrew retry safety for partial and incompatible states in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/scripts/release/publish_homebrew.sh
- [X] T031 [US3] Update workflow branching so retries classify runs as `materialized` or `partial` instead of republishing blindly in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/.github/workflows/release.yml
- [X] T032 [US3] Persist next-safe-action guidance for partial and blocked reruns in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/scripts/release/summarize_release.sh

**Checkpoint**: User Story 3 is complete when failed or repeated runs preserve external consistency and maintainers can retry safely without creating duplicate or conflicting publications.

---

## Phase 6: User Story 4 - Auditar el estado final y la trazabilidad de la release (Priority: P2)

**Goal**: Expose a final, operator-friendly release record that shows the authorized commit, target version, channel states, and the next safe action.

**Independent Test**: Complete one successful run, one blocked run, and one partial run; verify that each leaves a visible final summary containing the evaluated tag, authorized commit, version, per-channel state, global classification, and next-safe-action guidance without requiring manual reconstruction from raw logs.

### Validation for User Story 4

- [X] T033 [P] [US4] Extend contract coverage for final summary fields, reproducibility evidence, and per-channel visibility in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/tests/contract/release_automation_contract.rs
- [X] T034 [P] [US4] Add integration coverage for post-publication verification and operator-facing summaries in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/tests/integration/release_pipeline.rs

### Implementation for User Story 4

- [X] T035 [P] [US4] Enrich inspected release evidence with commit, artifact, checksum, and channel details in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/scripts/release/inspect_release_state.sh
- [X] T036 [P] [US4] Generate final release summaries with global status and next-safe-action fields in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/scripts/release/summarize_release.sh
- [X] T037 [US4] Publish post-deploy verification and run-summary outputs from the workflow in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/.github/workflows/release.yml
- [X] T038 [US4] Document the maintainer verification flow and expected final states in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/specs/007-gate-release-publishing/quickstart.md

**Checkpoint**: User Story 4 is complete when every release attempt closes with an auditable state record that explains exactly what happened and what to do next.

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Final cleanup, documentation alignment, and full validation across all release stories.

- [X] T039 [P] Refresh the release contract to match the implemented scripts, states, and evidence model in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/specs/007-gate-release-publishing/contracts/release-automation-contract.md
- [X] T040 [P] Refresh implementation notes, impacted files, and deferred scope after task execution in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/specs/007-gate-release-publishing/plan.md
- [X] T041 Review workflow copy, failure reasons, and command surface for non-essential complexity in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/.github/workflows/release.yml and /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/scripts/release/summarize_release.sh
- [X] T042 Run the full release verification checklist and record any follow-up fixes in /Users/santiagoacosta/Documents/personal-projects/sentinel-cli/specs/007-gate-release-publishing/quickstart.md

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1: Setup**: No dependencies. Start immediately.
- **Phase 2: Foundational**: Depends on Phase 1. Blocks every user story.
- **Phase 3: User Story 1**: Depends on Phase 2. This is the recommended MVP starting point because it establishes the strict release gate.
- **Phase 4: User Story 2**: Depends on Phase 2 and integrates most safely after US1 establishes the authorized release source.
- **Phase 5: User Story 3**: Depends on Phase 2 and should build on the publication paths completed in US2.
- **Phase 6: User Story 4**: Depends on Phase 2 and is most valuable once the authorization and publication flows from US1-US3 exist.
- **Phase 7: Polish**: Depends on all targeted user stories being complete.

### User Story Dependency Graph

- **US1**: Start after Foundational. No dependency on later stories.
- **US2**: Start after Foundational, but should follow US1 because publication must consume the authorized source defined there.
- **US3**: Start after Foundational, but should follow US2 because retry safety depends on real publication states.
- **US4**: Start after Foundational, but should follow US1-US3 so the final summary covers blocked, successful, and partial outcomes end to end.

Recommended completion order:

1. **US1**
2. **US2**
3. **US3**
4. **US4**

### Within Each User Story

- Validation tasks should be updated before the corresponding implementation is considered done.
- Authorization and version checks should exist before any publication logic is wired.
- Canonical artifact generation should exist before channel publication tasks are finalized.
- Channel inspection and retry branching should exist before the story is considered safe for reruns.
- Final summary generation and post-verification should be confirmed before a story is marked complete.

## Parallel Opportunities

- **Setup**: `T002`, `T003`, `T004`, and `T005` can run in parallel after `T001`.
- **Foundational**: `T007`, `T008`, and `T009` can run in parallel after `T006`; `T010` then consolidates the shared conventions.
- **US1**: `T011`, `T012`, `T013`, and `T014` can run in parallel; `T015-T016` then wire workflow gating and operator summaries.
- **US2**: `T017`, `T018`, `T019`, `T020`, `T021`, `T022`, `T023`, and `T024` can run in parallel; `T025` then integrates the canonical publication path.
- **US3**: `T026`, `T027`, `T028`, `T029`, and `T030` can run in parallel; `T031-T032` then finalize branching and safe rerun guidance.
- **US4**: `T033`, `T034`, `T035`, and `T036` can run in parallel; `T037-T038` then publish the final visibility path.
- **Polish**: `T039` and `T040` can run in parallel before `T041-T042`.

## Parallel Example: User Story 1

```bash
Task T011: Add contract coverage for tag parsing and main HEAD equality in tests/contract/release_automation_contract.rs
Task T012: Add integration coverage for allowed and blocked tags in tests/integration/release_pipeline.rs
Task T013: Implement tag validation and SHA resolution in scripts/release/authorize_release.sh
Task T014: Implement version extraction and tag/version comparison in scripts/release/resolve_version.sh
```

## Parallel Example: User Story 2

```bash
Task T017: Extend contract coverage for source-of-truth and manifest consistency in tests/contract/release_automation_contract.rs
Task T018: Add integration coverage for canonical release publication in tests/integration/release_pipeline.rs
Task T019: Build canonical artifact manifests in scripts/release/build_release_artifacts.sh
Task T020: Materialize the GitHub Release in scripts/release/publish_github_release.sh
Task T021: Prepare npm packaging metadata in packaging/npm/package.json
Task T023: Render the Homebrew formula template in packaging/homebrew/sentinel.rb.tpl
```

## Parallel Example: User Story 3

```bash
Task T026: Extend contract coverage for materialized and partial states in tests/contract/release_automation_contract.rs
Task T027: Add integration coverage for safe retry scenarios in tests/integration/release_pipeline.rs
Task T028: Extend channel-state inspection in scripts/release/inspect_release_state.sh
Task T029: Implement npm retry safety in scripts/release/publish_npm.sh
Task T030: Implement Homebrew retry safety in scripts/release/publish_homebrew.sh
```

## Parallel Example: User Story 4

```bash
Task T033: Extend contract coverage for final summary fields in tests/contract/release_automation_contract.rs
Task T034: Add integration coverage for post-release verification in tests/integration/release_pipeline.rs
Task T035: Enrich release evidence inspection in scripts/release/inspect_release_state.sh
Task T036: Generate final release summaries in scripts/release/summarize_release.sh
```

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1.
2. Complete Phase 2.
3. Complete Phase 3 (US1).
4. Validate the independent authorization gate test for US1.
5. Demo the blocked-versus-authorized release behavior before continuing.

### Incremental Delivery

1. Build the release scaffolding and shared inspection foundation once.
2. Ship the strict authorization gate (US1) as the first critical safeguard.
3. Add canonical artifact generation and dual-channel publication (US2).
4. Add safe retry and partial-failure handling (US3).
5. Add final auditability and operator visibility (US4).
6. Finish with documentation refresh and full quickstart validation.

### Suggested MVP Scope

- **MVP**: Phase 1 + Phase 2 + Phase 3 (US1 only)
- **Next increment**: Phase 4 (US2)
- **Third increment**: Phase 5 (US3)
- **Final increment**: Phase 6 + Phase 7 (US4 and polish)

## Notes

- All tasks use the required checklist format.
- Tasks marked `[P]` touch different files or can be executed as non-blocking work after their prerequisites.
- Each user story retains its own validation tasks and independent test criteria.
- The plan intentionally makes authorization and consistency checks land before any external publication work.
