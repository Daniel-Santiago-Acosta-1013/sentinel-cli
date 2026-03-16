# Tasks: CLI Guiada por Menús Segura

**Input**: Design documents from `/specs/004-menu-cli-network-safety/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/, quickstart.md

**Tests**: Esta feature exige pruebas de integración, contrato y snapshot para
validar UX de CLI, limpieza de pantalla e integridad de la configuración de red.

**Organization**: Las tareas están agrupadas por historia de usuario para que
cada historia pueda implementarse y validarse de forma independiente.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Puede ejecutarse en paralelo (archivos distintos, sin dependencia de tareas incompletas)
- **[Story]**: Historia de usuario asociada (`[US1]`, `[US2]`, `[US3]`)
- Todas las tareas incluyen rutas exactas de archivo

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Preparar dependencias y el esqueleto del nuevo flujo de CLI guiada

- [X] T001 Actualizar dependencias y cableado base de la CLI en `Cargo.toml`, `src/main.rs` y `src/cli/mod.rs`
- [X] T002 Crear el esqueleto de interacción guiada en `src/cli/copy.rs`, `src/cli/menu_state.rs`, `src/cli/renderer.rs` y `src/cli/terminal.rs`
- [X] T003 [P] Definir vocabulario estable en español para menús y estados en `src/cli/copy.rs`
- [X] T004 [P] Preparar helpers de transcript scriptado para la nueva CLI en `tests/support/mod.rs`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Infraestructura común que debe existir antes de cualquier historia

**⚠️ CRITICAL**: Ninguna historia de usuario debe comenzar antes de completar esta fase

- [X] T005 Refactorizar la entrada interactiva para despachar el nuevo runtime de CLI en `src/app.rs` y `src/cli/mod.rs`
- [X] T006 [P] Implementar primitivas de limpieza de pantalla, sondeo de teclado y detección de ancho en `src/cli/terminal.rs`
- [X] T007 [P] Normalizar etiquetas persistidas y metadatos de verificación para la UX en español en `src/storage/state.rs`
- [X] T008 [P] Extender la plataforma falsa para múltiples servicios y DNS personalizados en `src/platform/macos.rs` y `tests/support/mod.rs`
- [X] T009 [P] Añadir primitivas compartidas de comparación de snapshot y verificación de restauración en `src/control/snapshot.rs` y `src/control/coordinator.rs`
- [X] T010 Estabilizar el renderizado de transcript para contratos y snapshots en `src/app.rs` y `src/cli/renderer.rs`

**Checkpoint**: Base lista; US1 y US2 pueden avanzar, y US3 puede arrancar una vez estén disponibles las transiciones seguras de recuperación

---

## Phase 3: User Story 1 - Desactivar sin perder conectividad (Priority: P1) 🎯 MVP

**Goal**: Garantizar que desactivar o recuperar Sentinel restaure la
configuración original de red y deje una guía segura si la verificación falla

**Independent Test**: Activar protección con plataforma falsa, desactivarla y
comprobar que los DNS originales por servicio se restauran sin reinicio; forzar
una verificación fallida y comprobar que el estado queda degradado con
recuperación guiada

### Validation for User Story 1

- [X] T011 [P] [US1] Añadir contrato de restauración verificada en `tests/contract/recovery_contract.rs`
- [X] T012 [P] [US1] Añadir prueba de integración para activar y desactivar restaurando DNS originales en `tests/integration/recovery_flow.rs`
- [X] T013 [P] [US1] Añadir prueba de integración para fallos de verificación post-restauración en `tests/integration/safety_failures.rs`

### Implementation for User Story 1

- [X] T014 [P] [US1] Implementar comparación de snapshot restaurado y detección de mismatch en `src/control/snapshot.rs` y `src/control/recovery.rs`
- [X] T015 [P] [US1] Persistir resultados de verificación y siguientes acciones seguras en `src/storage/state.rs` y `src/storage/events.rs`
- [X] T016 [US1] Integrar la desactivación segura y las transiciones a estado degradado en `src/control/activation.rs` y `src/app.rs`
- [X] T017 [US1] Mostrar resultados de restauración verificada y guía de recuperación en `src/cli/renderer.rs` y `src/cli/menu_state.rs`

**Checkpoint**: US1 queda funcional como MVP y puede demostrarse sin depender de US2 o US3

---

## Phase 4: User Story 2 - Navegar con precisión usando teclado (Priority: P2)

**Goal**: Entregar una CLI guiada por menús, completamente en español, con
tablas legibles y refresco limpio de terminal

**Independent Test**: Abrir Sentinel en modo scriptado, navegar solo con
teclado, confirmar que cada vista se muestra en español, que el transcript usa
tablas para el estado y que no queda texto residual entre pantallas

### Validation for User Story 2

- [X] T018 [P] [US2] Añadir contrato de menús en español y tablas de estado en `tests/contract/interaction_contract.rs`
- [X] T019 [P] [US2] Añadir prueba de integración de navegación por teclado y refresco de pantalla en `tests/integration/interactive_activation.rs`
- [X] T020 [P] [US2] Añadir snapshots para inicio, estado y recuperación tabular en `tests/snapshot/home_and_activation.rs` y `tests/snapshot/recovery_and_status.rs`

### Implementation for User Story 2

- [X] T021 [P] [US2] Implementar la copia en español para menús, ayudas y confirmaciones en `src/cli/copy.rs` y `src/cli/menu_state.rs`
- [X] T022 [P] [US2] Implementar renderizado tabular y modo compacto por ancho de terminal en `src/cli/renderer.rs` y `src/cli/output.rs`
- [X] T023 [P] [US2] Implementar el comportamiento de limpieza o refresco entre vistas en `src/cli/terminal.rs` y `src/app.rs`
- [X] T024 [US2] Reemplazar el flujo interactivo basado en TUI por la nueva CLI guiada en `src/app.rs`, `src/cli/mod.rs` y `src/tui/mod.rs`

**Checkpoint**: US2 queda demostrable de forma independiente con navegación, tablas y textos en español

---

## Phase 5: User Story 3 - Recuperarse de estados incompletos o configuraciones previas (Priority: P3)

**Goal**: Detectar configuraciones previas y sesiones incompletas para dirigir
al usuario primero por una recuperación segura antes de nuevos cambios

**Independent Test**: Simular DNS personalizados en varios servicios, interrumpir
una sesión y validar que el siguiente arranque detecta el estado incompleto,
explica qué configuración preserva y conduce por recuperación antes de permitir
nuevas mutaciones de red

### Validation for User Story 3

- [X] T025 [P] [US3] Añadir contrato para arranque en estado degradado y guía de recuperación en `tests/contract/recovery_contract.rs`
- [X] T026 [P] [US3] Añadir prueba de integración para recuperación al iniciar tras sesión incompleta en `tests/integration/end_to_end_cli.rs`
- [X] T027 [P] [US3] Añadir prueba de integración para conservar DNS personalizados en múltiples servicios en `tests/integration/safety_failures.rs` y `tests/integration/recovery_flow.rs`

### Implementation for User Story 3

- [X] T028 [P] [US3] Implementar detección de DNS personalizados y metadatos preservados por servicio en `src/control/safety.rs` y `src/control/coordinator.rs`
- [X] T029 [P] [US3] Persistir marcadores de sesión incompleta y ruteo de recuperación al arranque en `src/storage/state.rs` y `src/app.rs`
- [X] T030 [US3] Implementar menús de recuperación inicial y explicación de configuración preservada en `src/cli/menu_state.rs`, `src/cli/renderer.rs` y `src/control/recovery.rs`

**Checkpoint**: US3 queda funcional con recuperación inicial guiada y preservación explícita de configuración previa

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Cerrar regresiones, limpiar deuda de transición y validar la documentación de la feature

- [X] T031 [P] Actualizar los entrypoints agregados de la suite para la nueva CLI en `tests/contract.rs`, `tests/integration.rs` y `tests/snapshot.rs`
- [X] T032 [P] Aislar o retirar código TUI obsoleto de la ruta interactiva en `src/tui/screens.rs`, `src/tui/app_state.rs` y `src/tui/theme.rs`
- [X] T033 Ajustar estilos finales y consistencia terminológica en español en `src/cli/styles.rs`, `src/cli/copy.rs` y `src/cli/output.rs`
- [X] T034 Validar y actualizar la documentación operativa en `specs/004-menu-cli-network-safety/quickstart.md` y `specs/004-menu-cli-network-safety/plan.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1 (Setup)**: Sin dependencias; puede iniciar de inmediato
- **Phase 2 (Foundational)**: Depende de Phase 1 y bloquea todas las historias
- **Phase 3 (US1)**: Depende de Phase 2
- **Phase 4 (US2)**: Depende de Phase 2
- **Phase 5 (US3)**: Depende de Phase 2 y se beneficia de que US1 ya haya fijado la semántica de restauración verificada
- **Phase 6 (Polish)**: Depende de las historias que se quieran entregar

### User Story Dependencies

- **US1 (P1)**: Sin dependencia funcional de otras historias; define el MVP seguro
- **US2 (P2)**: Sin dependencia funcional de otras historias; puede ejecutarse en paralelo con US1 después de Foundation
- **US3 (P3)**: Depende de las primitivas compartidas de Foundation y reutiliza los estados seguros establecidos por US1

### Dependency Graph

```text
Phase 1 -> Phase 2 -> US1 -> US3
                  \-> US2
US1 + US2 + US3 -> Phase 6
```

### Within Each User Story

- Las tareas de validación deben existir antes de cerrar la historia
- Persistencia y modelos antes de integración del flujo completo
- Controladores antes del renderizado final de la experiencia
- Cada historia debe poder probarse con sus propios criterios independientes

### Parallel Opportunities

- `T003` y `T004` pueden ejecutarse en paralelo dentro de Setup
- `T006`, `T007`, `T008` y `T009` pueden ejecutarse en paralelo dentro de Foundation
- Tras Foundation, `US1` y `US2` pueden avanzar en paralelo
- Dentro de `US1`, `T011`, `T012` y `T013` pueden hacerse en paralelo
- Dentro de `US2`, `T018`, `T019` y `T020` pueden hacerse en paralelo
- Dentro de `US3`, `T025`, `T026` y `T027` pueden hacerse en paralelo

---

## Parallel Example: User Story 1

```bash
Task: "Añadir contrato de restauración verificada en tests/contract/recovery_contract.rs"
Task: "Añadir prueba de integración para activar y desactivar restaurando DNS originales en tests/integration/recovery_flow.rs"
Task: "Añadir prueba de integración para fallos de verificación post-restauración en tests/integration/safety_failures.rs"
```

---

## Parallel Example: User Story 2

```bash
Task: "Añadir contrato de menús en español y tablas de estado en tests/contract/interaction_contract.rs"
Task: "Añadir prueba de integración de navegación por teclado y refresco de pantalla en tests/integration/interactive_activation.rs"
Task: "Añadir snapshots para inicio, estado y recuperación tabular en tests/snapshot/home_and_activation.rs y tests/snapshot/recovery_and_status.rs"
```

---

## Parallel Example: User Story 3

```bash
Task: "Añadir contrato para arranque en estado degradado y guía de recuperación en tests/contract/recovery_contract.rs"
Task: "Añadir prueba de integración para recuperación al iniciar tras sesión incompleta en tests/integration/end_to_end_cli.rs"
Task: "Añadir prueba de integración para conservar DNS personalizados en múltiples servicios en tests/integration/safety_failures.rs y tests/integration/recovery_flow.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Completar Phase 1: Setup
2. Completar Phase 2: Foundational
3. Completar Phase 3: US1
4. Validar restauración segura y estado degradado con la suite de US1
5. Detenerse y demostrar el MVP antes de tocar UX avanzada

### Incremental Delivery

1. Setup + Foundation dejan lista la arquitectura común
2. Entregar US1 para eliminar el riesgo principal de dañar la red
3. Entregar US2 para sustituir completamente la experiencia TUI por la CLI limpia en español
4. Entregar US3 para endurecer el arranque con recuperación y configuraciones previas
5. Cerrar con Phase 6 para limpieza y regresión transversal

### Parallel Team Strategy

1. Una persona toma `T005` y `T010` mientras otras resuelven `T006` a `T009`
2. Tras Foundation:
   - Persona A: US1
   - Persona B: US2
3. US3 arranca cuando ya estén estables las transiciones de restauración verificadas

---

## Notes

- Todas las tareas siguen el formato obligatorio de checklist con ID y rutas
- Las tareas `[P]` están diseñadas para tocar archivos diferentes o dependencias ya estabilizadas
- US1 es el alcance recomendado para el primer corte demostrable
- No mezclar nuevas capacidades de bloqueo con esta implementación; el foco es UX de CLI y seguridad de red
