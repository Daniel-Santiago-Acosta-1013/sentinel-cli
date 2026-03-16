# Implementation Plan: CLI Guiada por Menús Segura

**Branch**: `004-menu-cli-network-safety` | **Date**: 2026-03-16 | **Spec**: [spec.md](/Users/santiagoacosta/Documents/personal-projects/sentinel-cli/specs/004-menu-cli-network-safety/spec.md)
**Input**: Feature specification from `/specs/004-menu-cli-network-safety/spec.md`

## Summary

Reemplazar la experiencia principal basada en `ratatui` y pantalla alterna por
una CLI guiada por menús, en español, con refresco limpio de terminal y tablas
de estado legibles. La implementación priorizará dos resultados: navegación
precisa con teclado sin acumulación de texto y protección estricta de la
configuración original de red mediante snapshots validados, verificación
post-restauración y pruebas de integración/contrato que cubran activación,
desactivación, recuperación e interrupciones.

## Technical Context

**Language/Version**: Rust 1.90.0 (edition 2024)  
**Primary Dependencies**: `crossterm`, `comfy-table`, `tokio`, `serde`, `serde_json`, `toml`, `miette`, `thiserror`, `tracing`, `tracing-subscriber`, `directories`, `uuid`, `chrono`, `hickory-proto`  
**Storage**: Archivos locales bajo application support (`TOML` para configuración y `JSON`/`JSONL` para estado, snapshots, instalación y eventos)  
**Testing**: `cargo test`, pruebas de integración con plataforma falsa, pruebas de contrato para transcripts y seguridad de red, pruebas snapshot para tablas y vistas en español  
**Target Platform**: macOS 14+ para operación real; plataforma falsa controlada para validación automatizada  
**Project Type**: CLI interactiva a nivel sistema con flujos guiados y recuperación de red  
**Performance Goals**: navegación y refresco de vista perceptiblemente instantáneos en terminal interactiva, chequeos de seguridad completados en menos de 5 segundos en sistemas sanos, verificación post-restauración completada antes de cerrar el flujo sensible  
**Constraints**: toda la experiencia visible debe estar en español, los estados deben mostrarse con tablas legibles, la terminal debe limpiarse o refrescarse entre vistas, la herramienta no debe requerir reinicio del equipo para recuperar red tras una desactivación correcta, y cada cambio de red debe tener snapshot recuperable y validación posterior  
**Scale/Scope**: una sola herramienta local para un solo equipo y un solo usuario por sesión; el alcance de este ciclo cubre navegación, presentación, seguridad de red y validación, no nuevas capacidades de bloqueo ni automatización adicional

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### Phase 0 Gate

- `PASS`: La superficie pública sigue mínima; el flujo central continúa siendo
  una sola CLI guiada sin agregar nuevos comandos de usuario final.
- `PASS`: La experiencia de terminal mejora directamente la legibilidad con
  textos en español, tablas de estado y limpieza explícita de pantalla.
- `PASS`: Toda operación que toca la red queda sujeta a captura de estado,
  verificación de restauración y recuperación guiada.
- `PASS`: La propuesta preserva módulos pequeños al separar navegación,
  renderizado, traducciones, control de seguridad y persistencia de snapshots.
- `PASS`: Se difiere cualquier trabajo no esencial como temas avanzados,
  configuración extensa de atajos, internacionalización adicional y cambios al
  modelo de bloqueo.

### Post-Design Re-check

- `PASS`: El diseño mantiene una única experiencia principal y reutiliza la
  infraestructura existente de control y almacenamiento en lugar de ampliar la
  superficie de comandos.
- `PASS`: Los contratos definen salida estable en español, tablas de estado y
  condiciones de seguridad para que la UX y la red sean verificables.
- `PASS`: El modelo de datos y quickstart exigen verificación posterior a la
  restauración y cobertura automatizada de integridad de snapshots.
- `PASS`: No hay violaciones que justifiquen complejidad adicional en esta fase.

## Project Structure

### Documentation (this feature)

```text
specs/004-menu-cli-network-safety/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   ├── interaction-contract.md
│   └── network-safety-contract.md
└── tasks.md
```

### Source Code (repository root)

```text
src/
├── main.rs
├── app.rs
├── cli/
│   ├── mod.rs
│   ├── commands.rs
│   ├── output.rs
│   ├── styles.rs
│   ├── copy.rs
│   ├── menu_state.rs
│   ├── renderer.rs
│   └── terminal.rs
├── control/
│   ├── activation.rs
│   ├── coordinator.rs
│   ├── recovery.rs
│   ├── safety.rs
│   └── snapshot.rs
├── blocking/
│   ├── blocklist.rs
│   ├── resolver.rs
│   └── runtime.rs
├── platform/
│   └── macos.rs
├── storage/
│   ├── config.rs
│   ├── events.rs
│   ├── install.rs
│   └── state.rs
└── tui/
    ├── app_state.rs
    ├── input.rs
    ├── screens.rs
    └── theme.rs

tests/
├── contract/
│   ├── interaction_contract.rs
│   └── recovery_contract.rs
├── integration/
│   ├── interactive_activation.rs
│   ├── recovery_flow.rs
│   └── safety_failures.rs
├── snapshot/
│   ├── home_and_activation.rs
│   └── recovery_and_status.rs
└── support/
    └── mod.rs
```

**Structure Decision**: Migrar el flujo principal a `src/cli/` y tratar
`src/tui/` como implementación a retirar o adaptar durante la transición. La
interacción guiada, el renderizado en español, la limpieza de terminal y las
tablas vivirán en módulos de CLI dedicados, mientras que `control/`,
`platform/` y `storage/` seguirán concentrando la lógica de seguridad,
restauración y persistencia de snapshots.

## Complexity Tracking

No se anticipan violaciones de constitución. El plan evita introducir GUI,
daemon adicional, configuración avanzada de perfiles o nuevos comandos públicos.
