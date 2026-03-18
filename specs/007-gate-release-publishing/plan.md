# Implementation Plan: Automatización Segura de Releases

**Branch**: `007-gate-release-publishing` | **Date**: 2026-03-17 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/007-gate-release-publishing/spec.md`

## Summary

Automatizar releases estables de Sentinel CLI mediante GitHub Actions a partir
de tags `vMAJOR.MINOR.PATCH`, con una compuerta estricta que solo autoriza la
publicación cuando el tag resuelve exactamente al HEAD vigente de `main`. El
plan usa `Cargo.toml` como versión oficial del proyecto, genera un conjunto
canónico de artefactos con manifiesto verificable, y publica npm y Homebrew
solo desde ese mismo conjunto ya autorizado. El flujo debe clasificar cada
intento como bloqueado, materializado, parcial o completado para sostener
reproducibilidad, trazabilidad e idempotencia operativa.

## Technical Context

**Language/Version**: Rust 1.90.0 (edition 2024) para el binario y la versión
oficial del proyecto, más POSIX shell para automatización de release en CI  
**Primary Dependencies**: Dependencias actuales del binario en `Cargo.toml`,
`git`, `cargo`, `tar`, `shasum`, `curl`, `npm` CLI, GitHub Actions y un
repositorio de tap de Homebrew controlado por mantenimiento  
**Storage**: Archivos del repositorio (`Cargo.toml`, plantillas de packaging,
scripts de release), artefactos generados con checksums, metadatos de GitHub
Release y resúmenes persistidos por ejecución de CI  
**Testing**: `cargo test`, `cargo clippy`, pruebas de contrato e integración
basadas en fixtures para validación de tags, estados de canales y reintentos de
release  
**Target Platform**: GitHub-hosted runners para CI/CD, mantenedores operando
desde Git/GitHub, consumidores finales en npm y Homebrew  
**Project Type**: Aplicación CLI en Rust con automatización de distribución en
el mismo repositorio  
**Performance Goals**: Todo tag no autorizado debe quedar bloqueado en la
primera etapa en menos de 2 minutos; toda ejecución debe dejar estado final
visible en menos de 15 minutos  
**Constraints**: Sin nuevos comandos o flags públicos; publicación permitida
solo desde el HEAD vigente de `main`; una sola fuente de verdad por release;
fallo en modo seguro ante incertidumbre; reintentos idempotentes; rechazo de
prereleases y de canales distintos de npm y Homebrew  
**Scale/Scope**: Un repositorio, una línea de releases estables, dos canales de
distribución, un workflow principal de release, varios scripts internos de
verificación/publicación y cobertura automatizada para bloqueo, consistencia,
fallo parcial y post-verificación

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **Minimal Command Surface**: PASS. La feature no agrega comandos, flags ni
  configuraciones para usuarios de Sentinel; toda la complejidad queda en CI/CD
  y en assets de mantenimiento.
- **Exceptional Terminal Experience**: PASS. Aunque el cambio no altera la UX
  diaria del CLI, sí exige salidas de error y resúmenes de release claros para
  mantenedores, con motivos explícitos de bloqueo y estado por canal.
- **Safety Before Blocking**: PASS. La automatización no toca la configuración
  de red del usuario final, pero sí maneja una operación crítica de
  distribución; por eso se impone validación previa estricta, fallo seguro,
  bloqueo de estados inciertos y recuperación guiada ante fallos parciales.
- **Focused, Modular Quality**: PASS. El diseño separa workflow, scripts de
  release, packaging y pruebas de release sin mezclar esta lógica con módulos de
  runtime como `blocking/`, `control/` o `engine/`.
- **Deliberate Delivery**: PASS. El alcance se limita a releases estables desde
  `main@HEAD`, consistencia de versión, publicación a npm/Homebrew, trazabilidad
  y recuperación segura. Se difieren firma criptográfica avanzada, prereleases,
  más canales de distribución y automatizaciones de marketing.

**Post-Design Re-check**: PASS. La Fase 1 mantiene superficie pública mínima,
define errores y estados finales legibles para mantenimiento, opera en modo
seguro ante incertidumbre, conserva módulos claros entre runtime y release, y
deja fuera trabajo no esencial como prereleases, release notes generativas o
soporte de canales adicionales.

## Project Structure

### Documentation (this feature)

```text
specs/007-gate-release-publishing/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── release-automation-contract.md
└── tasks.md
```

### Source Code (repository root)

```text
.github/
└── workflows/
    └── release.yml

packaging/
├── npm/
│   ├── package.json
│   ├── README.md
│   └── bin/
└── homebrew/
    └── sentinel.rb.tpl

scripts/
└── release/
    ├── authorize_release.sh
    ├── resolve_version.sh
    ├── build_release_artifacts.sh
    ├── publish_github_release.sh
    ├── publish_npm.sh
    ├── publish_homebrew.sh
    ├── inspect_release_state.sh
    └── summarize_release.sh

src/
├── install/
│   └── version.rs
└── storage/

tests/
├── contract/
│   └── release_automation_contract.rs
├── integration/
│   └── release_pipeline.rs
└── support/
    └── release_fixtures.rs
```

**Structure Decision**: Mantener Sentinel como un único binario CLI en Rust y
agregar la lógica de release como una capa separada de mantenimiento. La versión
oficial sigue viviendo en `Cargo.toml` y `src/install/version.rs`; GitHub
Actions orquesta el flujo; `scripts/release/` concentra autorización,
construcción, publicación e inspección; `packaging/` contiene los insumos de
distribución para npm y Homebrew; `tests/` valida reglas e invariantes sin
mezclar esta automatización con la lógica operativa del producto.

## Current Delivery Gap

- El repositorio no tiene todavía una automatización oficial para publicar
  releases estables desde tags.
- No existe una compuerta explícita que garantice que un tag solo pueda
  publicarse si coincide exactamente con `main@HEAD`.
- No hay hoy una definición formal de fuente de verdad para versión, artefactos
  y canales de distribución.
- npm y Homebrew aún no tienen un contrato de consistencia compartido ni una
  estrategia documentada de recuperación ante fallos parciales.
- La trazabilidad final del release no está modelada como estado global auditable
  por intento.

## Release Automation Objective

Hacer que cada release estable de Sentinel sea un evento determinista y
auditable: el mantenedor crea un tag válido, el pipeline comprueba que ese tag
representa exactamente el último commit autorizado de `main`, valida la versión
oficial del proyecto, genera artefactos reproducibles con checksums, publica
solo desde ese conjunto canónico y deja un estado final inequívoco con detalle
por canal, motivo de bloqueo o evidencia de materialización previa.

## Scope Boundaries

**In scope**

- Workflow principal de release activado por tags estables.
- Verificación estricta `tag commit == HEAD actual de main`.
- Validación de consistencia entre tag, `Cargo.toml`, artefactos, npm y
  Homebrew.
- Generación de artefactos canónicos y manifiesto verificable.
- Publicación de npm y Homebrew desde el mismo conjunto autorizado.
- Clasificación y reporte de estados `blocked`, `materialized`, `partial` y
  `completed`.
- Reintentos seguros y bloqueo de re-publicaciones inconsistentes.
- Post-verificación del estado final en ambos canales.
- Cobertura automatizada para autorización, consistencia, idempotencia y
  fallos parciales.

**Out of scope**

- Prereleases, canales beta o versiones con sufijos.
- Publicación a Homebrew Core, apt, Scoop u otros registries.
- Firma criptográfica avanzada de artefactos más allá de checksums y
  verificabilidad del manifiesto.
- Generación automática de changelogs o notas de marketing.
- Cambios al runtime funcional de bloqueo de DNS, recuperación o UX interactiva
  de Sentinel fuera de lo necesario para exponer la versión oficial.

## Components and Files Likely Impacted

- **`.github/workflows/release.yml`**: workflow único responsable de validar,
  construir, publicar y resumir el estado del release.
- **`scripts/release/authorize_release.sh`**: resuelve SHA del tag, HEAD de
  `main` y decide si la release está autorizada o debe bloquearse.
- **`scripts/release/resolve_version.sh`**: extrae versión oficial del proyecto
  y valida que coincide exactamente con el tag.
- **`scripts/release/build_release_artifacts.sh`**: construye el conjunto
  canónico de artefactos y genera manifiesto con checksums.
- **`scripts/release/publish_github_release.sh`**: materializa el release
  canónico como referencia auditable antes de publicar canales externos.
- **`scripts/release/publish_npm.sh`**: publica la versión o confirma
  materialización previa si ya coincide con la release autorizada.
- **`scripts/release/publish_homebrew.sh`**: renderiza y sincroniza la fórmula
  desde el manifiesto canónico hacia el tap oficial.
- **`scripts/release/inspect_release_state.sh`**: consulta el estado observable
  de GitHub Release, npm y Homebrew para detectar parcialidad o desalineación.
- **`scripts/release/summarize_release.sh`**: emite el estado final y la razón
  global del intento para mantenimiento y auditoría.
- **`packaging/npm/package.json`**: define el paquete publicado en npm y su
  versión alineada con el release autorizado.
- **`packaging/homebrew/sentinel.rb.tpl`**: plantilla de fórmula que se rellena
  con versión, URL y checksum del artefacto canónico.
- **`Cargo.toml`**: sigue siendo la declaración oficial de versión del proyecto.
- **`src/install/version.rs`**: sigue siendo la referencia de versión expuesta
  por el binario y debe mantenerse coherente con el release.
- **`tests/contract/*`**: validan reglas de autorización, consistencia de
  versión y clasificación de estados.
- **`tests/integration/*`**: validan flujos de release completo, bloqueo,
  reintento y estado parcial usando fixtures.

## Phase Strategy

### Phase 0 - Research and Decision Closure

- Confirmar que la fuente de verdad del release será:
  - commit autorizado igual a `main@HEAD`
  - tag estable `vMAJOR.MINOR.PATCH` que apunta a ese commit
  - versión declarada en `Cargo.toml`
  - manifiesto de artefactos generado desde ese commit
- Definir la estrategia de autorización en dos momentos:
  - al inicio del workflow
  - inmediatamente antes de la primera publicación externa
- Determinar el orden de materialización:
  - construir artefactos
  - publicar release canónico en GitHub
  - publicar npm
  - publicar Homebrew
  - verificar estado final
- Cerrar la política de reintentos:
  - materialized si todo ya existe y coincide
  - blocked si hay evidencia incompatible
  - partial si algún canal quedó a medio camino
- Documentar el tap oficial de Homebrew como destino externo controlado por
  mantenimiento y no como fuente de verdad.

### Phase 1 - Design and Contracts

- Modelar entidades de release, artefactos y estados por canal.
- Definir el contrato visible del pipeline para autorización, consistencia,
  bloqueo y post-verificación.
- Diseñar el layout modular de `.github/workflows/`, `scripts/release/` y
  `packaging/`.
- Preparar quickstart y criterios de aceptación orientados a mantenimiento.
- Actualizar el contexto del agente con la nueva capa de CI/CD y distribución.

### Phase 2 - Implementation Planning

- Desglosar tareas por:
  - workflow y gating
  - resolución de versión y generación de artefactos
  - publicación en GitHub/npm/Homebrew
  - inspección de estado y resúmenes
  - pruebas de contrato e integración
- Asegurar que cada tarea preserve:
  - bloqueo seguro
  - idempotencia
  - trazabilidad
  - verificabilidad posterior al despliegue

## Validation Strategy

### Required Validations Before Any Publication

- El tag existe, es estable y respeta `vMAJOR.MINOR.PATCH`.
- El commit del tag es exactamente igual al HEAD vigente de `main`.
- La versión del tag y la versión de `Cargo.toml` coinciden exactamente.
- La versión visible del binario no contradice la versión oficial del proyecto.
- Los artefactos se generan desde el commit autorizado y quedan acompañados por
  checksums y manifiesto.
- No existe evidencia previa incompatible en GitHub Release, npm o Homebrew para
  esa misma versión.

### Validations During Publication

- npm recibe solo la versión autorizada y desde el mismo conjunto de artefactos
  ya aprobado.
- Homebrew se actualiza con fórmula derivada del mismo manifiesto canónico.
- Cualquier fallo después de materializar un canal detiene el avance a éxito
  global y fuerza clasificación parcial.

### Post-Publication Verifications

- GitHub Release exhibe la versión, el commit y los checksums esperados.
- npm muestra la versión autorizada y no una variación distinta.
- Homebrew referencia la misma versión y el mismo artefacto canónico.
- El resumen final permite identificar commit, tag, versión, estado por canal y
  siguiente acción segura.

## Release State Model

- **Blocked**: la autorización o consistencia falló antes de publicar.
- **Materialized**: la versión ya estaba correctamente presente y coincide con
  la release autorizada; no se publica de nuevo.
- **Partial**: al menos un canal quedó materializado y otro no, o la
  post-verificación no logró cerrar consistencia completa.
- **Completed**: GitHub Release, npm y Homebrew quedaron alineados con la misma
  versión y el mismo conjunto canónico de artefactos.

## Testing Coverage Plan

### Contract Coverage

- Tag válido en `main@HEAD` autoriza la release.
- Tag válido en commit viejo de `main` bloquea la release.
- Tag válido fuera de `main@HEAD` bloquea la release.
- Tag y `Cargo.toml` desalineados bloquean la release.
- Estado ya materializado en ambos canales produce resultado `materialized`.
- Evidencia incompatible entre canales produce `blocked` o `partial` según el
  punto exacto del hallazgo.

### Integration Coverage

- Pipeline feliz completo desde tag autorizado hasta verificación final.
- Fallo después de GitHub Release y antes de npm.
- Fallo después de npm y antes de Homebrew.
- Reintento seguro de un estado parcial.
- Reejecución de una versión ya completada sin duplicar publicación.
- Imposibilidad de resolver `main@HEAD` o estado real de un canal.

### Regression Coverage

- `src/install/version.rs` y `Cargo.toml` siguen devolviendo la versión oficial
  usada por el pipeline.
- Los scripts de release no introducen dependencias innecesarias en el binario.
- El repositorio mantiene `cargo test` y `cargo clippy` como validaciones base.

## Operational Risks and Mitigations

- **Race con avance de `main`**: Mitigar revalidando `main@HEAD` justo antes de
  cualquier publicación externa.
- **Versión ya publicada parcialmente**: Mitigar inspeccionando estado previo y
  bloqueando repeticiones incompatibles.
- **Diferencia entre artefactos y metadatos de canal**: Mitigar usando un solo
  manifiesto canónico y checksums obligatorios.
- **Fallo del destino Homebrew externo**: Mitigar clasificando `partial`,
  preservando evidencia del canal ya materializado y evitando marcar éxito.
- **Secrets o permisos incompletos en CI**: Mitigar fallando antes de publicar y
  dejando razón explícita en el resumen final.
- **Reposición manual no auditada**: Mitigar haciendo que el workflow consulte
  el estado real de los canales y nunca asuma éxito por intención previa.

## Verification Criteria

- Una release solo llega a `completed` si el tag y `main@HEAD` son exactamente
  el mismo commit y todos los canales coinciden con la versión autorizada.
- Ningún intento con tag viejo, rama incorrecta o estado incierto publica
  externamente.
- Todo intento deja resumen final auditable con datos suficientes para
  reconstruir la decisión.
- Un reintento de la misma versión nunca duplica una publicación correcta ni
  pisa evidencia incompatible existente.
- La evidencia visible de npm y Homebrew puede trazarse de vuelta al mismo
  conjunto canónico de artefactos.

## Implementation Notes

- El workflow quedó centralizado en `.github/workflows/release.yml`.
- Los scripts ejecutables viven en `scripts/release/` y comparten utilidades en
  `scripts/release/common.sh`.
- El manifiesto canónico implementado es `release-manifest.env`, acompañado por
  `SHASUMS256.txt` y el archivo `.tar.gz` del binario.
- La validación automatizada actual cubre contratos y flujos de integración para
  autorización estricta, publicación canónica, fallos parciales, reintentos y
  visibilidad final.

## Complexity Tracking

No constitution violations requiring justification.
