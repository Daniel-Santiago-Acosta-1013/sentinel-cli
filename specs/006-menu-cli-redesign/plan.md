# Implementation Plan: Rediseño de CLI por Vistas

**Branch**: `006-menu-cli-redesign` | **Date**: 2026-03-16 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/006-menu-cli-redesign/spec.md`

## Summary

Reestructurar la experiencia interactiva de Sentinel para que opere como una
CLI clásica, guiada y menu-driven sobre la base actual en Rust: home limpio con
logo ASCII, navegación por vistas independientes, limpieza de pantalla entre
transiciones, presentación visual con ANSI/Unicode/spinners y eliminación
completa del código TUI. El cambio preserva el command surface existente y
reusa la lógica de seguridad, activación, estado, instalación y recuperación ya
presente en `control/`, `storage/` y `blocking/`.

## Technical Context

**Language/Version**: Rust 1.90.0 (edition 2024)  
**Primary Dependencies**: `crossterm`, `comfy-table`, `tokio`, `serde`,
`serde_json`, `toml`, `miette`, `thiserror`, `tracing`,
`tracing-subscriber`, `directories`, `uuid`, `chrono`, `hickory-proto`;
`ratatui` is currently present and targeted for removal in this feature  
**Storage**: Local filesystem state under application support directories
using `TOML` for configuration and `JSON`/`JSONL` for runtime state, install
metadata, snapshots, and events  
**Testing**: `cargo test`, existing contract/integration/snapshot suites,
plus `cargo clippy` for static validation  
**Target Platform**: Interactive terminal sessions on macOS with local network
control and a scripted non-interactive transcript mode for tests  
**Project Type**: Single-binary Rust CLI application  
**Performance Goals**: Home and view transitions should feel immediate in a
terminal session, and operations lasting more than 1 second must expose visible
progress feedback before completion  
**Constraints**: Preserve network safety and rollback behavior, avoid adding
new user-facing commands or flags, eliminate full-screen reactive/TUI behavior,
keep output legible in terminals with limited styling support, and avoid
functional regressions in existing core flows  
**Scale/Scope**: One interactive CLI entrypoint, one home view, 5-7 dedicated
action/status/result views, removal of the `src/tui` layer, and updates across
interactive contract, snapshot, and integration coverage

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **Minimal Command Surface**: PASS. The feature keeps the existing `sentinel`
  entrypoint and interactive actions; no new top-level commands or flags are
  required. Any internal restructuring stays behind the current surface.
- **Exceptional Terminal Experience**: PASS. The redesign directly targets
  cleaner output, explicit next steps, restrained styling, visible results, and
  clearer navigation between views.
- **Safety Before Blocking**: PASS. Activation, deactivation, status, and
  recovery remain backed by the current safety controllers and snapshot logic;
  the plan preserves confirmations, recovery-first behavior, and rollback
  messaging for network-affecting flows.
- **Focused, Modular Quality**: PASS. The migration concentrates interactive UX
  inside `src/cli`, removes the separate `src/tui` stack, and keeps network and
  persistence logic in their existing modules.
- **Deliberate Delivery**: PASS. Scope is limited to interactive CLI UX,
  presentation architecture, and related tests. Non-essential theming,
  personalization, mouse input, and automation redesign are explicitly
  deferred.

**Post-Design Re-check**: PASS. The proposed design still preserves minimal
surface area, clean terminal output, existing safety boundaries, modular
ownership, and deferral of non-essential work.

## Project Structure

### Documentation (this feature)

```text
specs/006-menu-cli-redesign/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── interactive-cli.md
└── tasks.md
```

### Source Code (repository root)

```text
src/
├── app.rs
├── blocking/
├── cli/
│   ├── commands.rs
│   ├── copy.rs
│   ├── logo.rs
│   ├── menu_state.rs
│   ├── navigation.rs
│   ├── output.rs
│   ├── renderer.rs
│   ├── spinner.rs
│   ├── styles.rs
│   └── terminal.rs
│   └── views.rs
├── control/
├── core/
├── engine/
├── install/
├── platform/
└── storage/

tests/
├── contract/
├── integration/
├── snapshot/
└── support/
```

**Structure Decision**: Mantener el proyecto como un único binario CLI en Rust.
La reestructuración concentra navegación, render, branding y feedback visual en
`src/cli` mediante `navigation.rs`, `views.rs`, `logo.rs`, `spinner.rs` y la
capa existente de renderer/terminal. `control/`, `storage/`, `blocking/`,
`engine/` e `install/` siguen como capas de lógica. La antigua carpeta
`src/tui` ya no forma parte de la arquitectura interactiva.

## Current UX/UI Problem

- La experiencia interactiva actual mezcla un flujo guiado con una mecánica de
  actualización reactiva sobre la misma superficie, lo que hace que la pantalla
  se sienta más cercana a una TUI que a una CLI clásica.
- El home comparte demasiada responsabilidad con vistas secundarias y no actúa
  como punto de entrada limpio y estable para la navegación.
- El usuario cambia de acción pero la percepción visual sigue siendo la misma
  pantalla, lo que reduce la claridad contextual y complica saber dónde está.
- La capa visual actual casi no aprovecha estilo, jerarquía ni feedback de
  progreso, por lo que la herramienta no transmite una experiencia de CLI
  profesional.
- La coexistencia de `src/cli` y `src/tui` deja una arquitectura de
  presentación dividida y potencialmente contradictoria para el producto.

## Restructuring Objective

Hacer que Sentinel se sienta como una CLI profesional, limpia, guiada y
coherente. El home debe ser minimalista y servir como hub principal; cada
acción debe abrir una vista independiente con su propio contexto, limpiar la
pantalla al entrar y mostrar resultados de forma separada. La estética debe
mejorar la legibilidad y la confianza, no introducir complejidad ni confusión.

## UX/UI Principles

- **Home-first minimalism**: El home muestra branding, acciones principales y
  guía mínima; evita tablas o detalles secundarios por defecto.
- **One intent per view**: Cada vista resuelve una intención concreta:
  seleccionar, confirmar, consultar estado, ejecutar una acción o leer un
  resultado.
- **Explicit navigation**: Toda transición debe dejar claro cómo avanzar,
  regresar o salir.
- **Result-oriented copy**: Cada vista debe comunicar estado actual, resultado,
  riesgo y siguiente paso sin texto sobrante.
- **Restrained CLI polish**: ANSI, símbolos y spinners se usan como soporte de
  comprensión, no como decoración agresiva.
- **Terminal-safe degradation**: La experiencia debe seguir siendo legible si el
  terminal no soporta color, Unicode amplio o animación fluida.
- **Safety stays visible**: Las acciones que tocan red conservan confirmación,
  advertencias y caminos de recuperación claros.

## Functional Scope

**In scope**

- Redefinir el home como punto principal de navegación.
- Separar las acciones actuales en vistas independientes.
- Limpiar pantalla en cada transición entre vistas y resultados.
- Incorporar el logo ASCII provisto por el usuario al home.
- Añadir una capa visual CLI con énfasis ANSI, símbolos Unicode y spinners.
- Reorganizar la arquitectura interactiva para eliminar el comportamiento tipo
  TUI y retirar `src/tui`.
- Actualizar pruebas y snapshots para validar el nuevo flujo.

**Out of scope**

- Añadir nuevos comandos o flags al surface público.
- Rediseñar la lógica de negocio de seguridad, activación o recuperación.
- Personalización de temas, atajos o layouts por usuario.
- Soporte de mouse, layouts complejos o interacción multi-panel.
- Cambios a los modos internos más allá de lo necesario para conservar tests y
  compatibilidad de transcript.

## Rust Modules and Layers Impacted

- **`src/app.rs`**: seguirá siendo el orquestador principal, pero debe pasar de
  un render loop reactivo a un flujo explícito de rutas/vistas y resultados.
- **`src/cli/menu_state.rs`**: debe evolucionar desde una sesión centrada en
  selección dentro de una sola pantalla a un estado de navegación por vistas,
  historial/back y operaciones en curso.
- **`src/cli/renderer.rs`**: debe dividirse hacia render por vista, con
  construcción de frames separados para home, confirmación, estado, seguridad,
  recuperación y resultados.
- **`src/cli/terminal.rs`**: seguirá controlando limpieza de pantalla, lectura
  de teclado y degradación compatible con terminal; debe soportar feedback de
  progreso sin volver al patrón TUI.
- **`src/cli/copy.rs`**: debe redefinir títulos, hints y microcopy para un
  flujo por vistas mucho más limpio y orientado a siguiente paso.
- **`src/cli/styles.rs`**: pasa de ser un stub a una capa real de ANSI,
  símbolos y fallbacks visuales.
- **`src/cli/output.rs`**: debe concentrarse en bloques reutilizables para
  tablas/resúmenes dentro de vistas separadas.
- **`src/tui/*`**: debe eliminarse por completo junto con su dependencia
  asociada una vez alcanzada la paridad.
- **`Cargo.toml`**: debe eliminar la dependencia TUI y reflejar solo la
  arquitectura CLI resultante.
- **`tests/contract/*`**, **`tests/snapshot/*`** y **`tests/integration/*`**:
  deben actualizarse para validar home limpio, transiciones por vista, feedback
  visual y ausencia de regresiones en seguridad.

## Migration Strategy to Independent CLI Views

### Phase 0 - Align behavior and target UX

- Confirmar el mapa de acciones actuales y su destino de vista.
- Definir el problema de UX actual, el objetivo final y los principios visuales
  del CLI.
- Decidir la estrategia de degradación para color, Unicode y spinner.

### Phase 1 - Introduce route-based CLI presentation

- Reemplazar el modelo centrado en `ViewId` + pantalla única por un modelo de
  rutas/vistas explícitas con contexto de navegación y salida clara.
- Redefinir el home como vista separada, con logo ASCII y acciones principales.
- Separar confirmaciones, estados y resultados en pantallas dedicadas.

### Phase 2 - Layer visual polish without changing business logic

- Implementar estilos ANSI y símbolos para títulos, estados y severidad.
- Añadir spinner/feedback de progreso para operaciones perceptiblemente largas.
- Mantener el contenido textual claro y compatible con fallback plano.

### Phase 3 - Remove TUI residue and restore test confidence

- Eliminar `src/tui` y la dependencia `ratatui`.
- Actualizar contratos, snapshots e integraciones al flujo por vistas.
- Verificar que activación, desactivación, estado y recuperación siguen
  respetando el comportamiento seguro esperado.

## Acceptance and Validation Plan

- El home abre limpio, muestra el logo ASCII y solo presenta navegación
  principal y ayuda mínima.
- Cada opción principal del home abre una vista distinta o un resultado
  independiente y limpia la pantalla al entrar.
- El usuario puede regresar desde vistas secundarias sin reiniciar Sentinel.
- Los flujos de estado, seguridad, activación, desactivación e instalación
  siguen disponibles sin regresión funcional crítica.
- Las operaciones largas muestran progreso visible y terminan con una vista de
  resultado clara.
- La experiencia sigue siendo legible cuando ANSI, Unicode o spinner no están
  disponibles.
- No queda código TUI ni dependencia asociada tras la migración.
- La validación mínima de implementación debe incluir `cargo test` y
  `cargo clippy`.

## Risks and Important Considerations

- **Riesgo de regresión en flujos sensibles**: el rediseño toca el camino
  interactivo principal de acciones que afectan red; las confirmaciones y
  resultados no deben debilitar garantías de seguridad ya existentes.
- **Riesgo de snapshots frágiles**: el cambio visual alterará gran parte de los
  tests de snapshot y transcript; conviene reescribirlos en torno a contratos
  de vista, no a texto incidental.
- **Compatibilidad de terminal**: ANSI, Unicode y spinners pueden verse
  distinto según entorno; se requiere fallback limpio.
- **Acoplamiento entre navegación y lógica**: si el flujo de vistas se mezcla
  con controladores de negocio, la eliminación de TUI no reducirá complejidad.
- **Percepción de “demasiado diseño”**: la estética debe mantenerse sobria para
  no contradecir la constitución del proyecto.

## Ordered Implementation Breakdown

1. Documentar el mapa actual de acciones, vistas y contratos que deben
   preservarse.
2. Diseñar el modelo de navegación objetivo: home, vistas de acción,
   confirmaciones y resultados.
3. Integrar el logo ASCII y simplificar el contenido visible del home.
4. Reestructurar `menu_state` para soportar rutas explícitas, back y contexto de
   operación.
5. Dividir el renderer en componentes por vista y definir el contrato de frame
   para cada una.
6. Implementar la capa de estilos ANSI, símbolos y fallbacks legibles.
7. Añadir feedback de progreso para operaciones largas sin volver al render
   reactivo persistente.
8. Adaptar `terminal.rs` para limpiar pantalla y manejar transiciones de vista
   de forma coherente.
9. Reescribir copy y ayudas para un flujo más breve, guiado y profesional.
10. Eliminar `src/tui` y retirar `ratatui` del proyecto.
11. Actualizar contratos, snapshots e integraciones a la nueva navegación.
12. Ejecutar validaciones finales de comportamiento, seguridad y legibilidad en
    terminal.

## Complexity Tracking

No constitution violations or exceptional complexity are required for this
feature. The plan deliberately removes architectural overlap instead of adding
new layers.
