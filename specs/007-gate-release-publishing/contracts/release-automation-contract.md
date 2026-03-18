# Release Automation Contract: Sentinel CLI

## Purpose

Definir el contrato operativo visible para la automatización de releases
estables de Sentinel CLI desde GitHub Actions hacia npm y Homebrew.

## Entry Contract

- El flujo de release solo se activa por tags estables con formato
  `vMAJOR.MINOR.PATCH`.
- La automatización debe evaluar siempre el commit exacto del tag y el HEAD
  actual de `main`.
- Si el tag no apunta exactamente a `main@HEAD`, el workflow debe bloquear la
  release antes de generar artefactos o publicar externamente.

## Source of Truth Contract

- La fuente de verdad de una release está compuesta por:
  - el HEAD vigente de `main`
  - el tag estable que apunta a ese mismo commit
  - la versión oficial declarada en `Cargo.toml`
  - el manifiesto canónico `release-manifest.env` generado desde ese commit
- Ningún canal externo puede convertirse en fuente de verdad alternativa.
- npm y Homebrew deben derivar del mismo manifiesto canónico ya autorizado.

## Version Consistency Contract

- La versión del tag debe coincidir exactamente con la versión del proyecto.
- El binario, los artefactos, npm y Homebrew deben exhibir esa misma versión.
- Cualquier discrepancia detectada antes de publicar debe bloquear la release.
- Cualquier discrepancia detectada después de publicar algún canal debe impedir
  declarar éxito completo y debe quedar reflejada como `partial` o `blocked`.

## Publication Contract

- La automatización debe materializar primero un release canónico auditable con
  artefactos y checksums verificables.
- Los scripts de publicación deben aceptar un manifiesto explícito mediante
  `RELEASE_MANIFEST_PATH` para evitar publicar desde estado implícito o
  reconstruido fuera del commit autorizado.
- npm solo puede publicarse desde la release canónica ya generada.
- Homebrew solo puede publicarse desde la misma evidencia canónica utilizada por
  npm.
- Si un canal falla, la automatización debe detener el avance automático a
  `completed`.

## Retry and Idempotency Contract

- Antes de reintentar, el workflow debe inspeccionar el estado real de GitHub
  Release, npm y Homebrew.
- Si la versión ya existe correctamente y coincide con la release autorizada, el
  resultado debe ser `materialized` y no una nueva publicación.
- Si existe evidencia incompatible en cualquier canal, la automatización debe
  bloquear la re-publicación hasta intervención del mantenedor.
- Un reintento nunca puede sobrescribir o duplicar una versión ya materializada
  correctamente.

## Final Visibility Contract

- Toda ejecución debe cerrar con un estado global: `blocked`, `materialized`,
  `partial` o `completed`.
- El resumen final debe exponer:
  - tag evaluado
  - commit autorizado
  - versión objetivo
  - estado por canal
  - razón global del resultado
  - siguiente acción segura
- La ausencia de evidencia suficiente nunca puede presentarse como éxito.

## Verification Contract

- La automatización debe comprobar el estado final observable en GitHub Release,
  npm y Homebrew antes de declarar `completed`.
- La inspección del estado final debe poder operar tanto con estados reales de
  canal como con un backend de estado controlado por `RELEASE_STATE_DIR` para
  validar el flujo de manera reproducible en pruebas.
- La release solo puede considerarse reproducible si sus artefactos y checksums
  pueden trazarse al mismo commit autorizado.
- Todo flujo de release debe permitir auditoría posterior sin necesidad de
  reconstruir manualmente decisiones implícitas del pipeline.
