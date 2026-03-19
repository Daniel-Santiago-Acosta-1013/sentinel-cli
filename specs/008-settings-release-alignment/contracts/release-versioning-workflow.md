# Release Versioning Workflow Contract: Sentinel CLI

## Purpose

Definir el contrato operativo del flujo oficial de versionado y release
centralizado en GitHub Actions para Sentinel CLI.

## Entry Contract

- La nueva versión debe entrar al flujo oficial como input explícito del
  workflow.
- La release ya no depende de tags creados o empujados manualmente desde el
  entorno local.
- El workflow oficial es la única ruta soportada para crear una nueva versión
  dentro del alcance de esta feature.

## Version Alignment Contract

- El primer job del workflow debe ejecutar un único script de alineación que
  actualice la nueva versión en `Cargo.toml`, `packaging/npm/package.json` y
  `packaging/homebrew/sentinel.rb.tpl`.
- Ese job debe validar que todas las superficies requeridas quedaron alineadas
  antes de continuar.
- Si alguna superficie falla o queda inconsistente, el workflow debe detenerse
  antes de crear tag o desplegar artefactos.

## Auditability Contract

- El job de versionado debe dejar un commit auditable con los cambios de
  versión.
- El tag oficial de la nueva versión debe crearse desde ese estado ya validado.
- La evidencia del cambio debe permitir reconstruir qué versión se solicitó,
  qué superficies se tocaron, cuál fue el commit fuente del release y qué tag
  quedó asociado a ese commit.

## Release and Deploy Contract

- El job de release/deploy debe ejecutarse separado del job de versionado.
- Ese job debe consumir la versión y el tag creados por el job previo, no
  resolver una versión alternativa por su cuenta.
- Los artefactos, npm y Homebrew deben derivarse del mismo estado versionado y
  del mismo tag oficial.
- Si algún canal queda desalineado o falla, el resultado global no puede
  declararse exitoso.

## Consistency Contract

- La versión final visible en el repositorio, el tag, los artefactos, npm y
  Homebrew debe ser exactamente la misma.
- El workflow debe bloquear cualquier intento que no pueda probar esa
  coincidencia.
- La ausencia de evidencia suficiente nunca puede presentarse como release
  completada.

## Final Visibility Contract

- Toda ejecución debe cerrar con un resumen que muestre, como mínimo:
  - versión solicitada
  - resultado de alineación previa
  - commit auditable generado
  - tag creado
  - resultado por canal
  - siguiente acción segura si hubo fallo o parcialidad
