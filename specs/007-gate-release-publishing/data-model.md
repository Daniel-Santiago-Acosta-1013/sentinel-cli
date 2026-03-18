# Data Model: Automatización Segura de Releases

## Overview

El feature no introduce una base de datos nueva, pero sí define un modelo
operativo claro para representar la autorización del release, el conjunto
canónico de artefactos y el estado observable en GitHub Release, npm y
Homebrew.

## Entities

### 1. ReleaseIntent

Representa el intento inicial de release disparado por un tag estable.

**Fields**

- `tag_name`: nombre del tag, por ejemplo `v0.2.0`.
- `tag_commit`: SHA exacto al que apunta el tag.
- `requested_at`: momento en que el workflow empieza a evaluar el intento.
- `requested_by`: actor que originó el tag o la ejecución.
- `tag_format_valid`: indica si el tag respeta el formato estable permitido.

**Validation rules**

- `tag_name` debe seguir `vMAJOR.MINOR.PATCH`.
- `tag_commit` debe resolverse de forma determinista antes de cualquier otra
  validación.
- Si `tag_format_valid` es falso, el intento debe pasar directo a `blocked`.

### 2. AuthorizedReleaseSource

Representa la fuente de verdad autorizada para una release.

**Fields**

- `main_head_commit`: SHA del HEAD vigente de `main`.
- `authorized_commit`: SHA autorizado para publicar.
- `project_version`: versión oficial leída del proyecto.
- `tag_version`: versión expresada por el tag.
- `authorized_at`: momento de la validación efectiva.
- `authorization_passed`: indica si el intento quedó autorizado.
- `authorization_reason`: razón explícita del resultado.

**Validation rules**

- `authorized_commit` debe ser exactamente igual a `main_head_commit`.
- `tag_version` debe ser exactamente igual a `project_version`.
- `authorization_passed` solo puede ser verdadero si ambas igualdades se
  cumplen a la vez.

### 3. ArtifactManifest

Representa el conjunto canónico de artefactos derivados de la release
autorizada.

**Fields**

- `version`: versión única asociada al release.
- `source_commit`: commit exacto desde el que se construyó.
- `generated_at`: momento de construcción.
- `artifacts`: lista de artefactos generados.
- `checksums`: mapa de checksums por artefacto.
- `manifest_id`: identificador del manifiesto de release.

**Validation rules**

- Todos los artefactos deben compartir la misma `version`.
- `source_commit` debe ser exactamente igual a `authorized_commit`.
- Cada artefacto debe tener checksum verificable antes de cualquier publicación.

### 4. ReleaseArtifact

Representa una unidad distribuible del release canónico.

**Fields**

- `name`: nombre del archivo o paquete.
- `artifact_type`: categoría (`binary-archive`, `npm-package`, `formula-input`,
  `manifest`).
- `version`: versión del artefacto.
- `checksum`: checksum asociado.
- `origin`: referencia al manifiesto canónico.
- `publishable_channels`: canales a los que puede alimentar.

**Validation rules**

- `version` debe coincidir con la versión del `ArtifactManifest`.
- Todo artefacto publicado debe poder trazarse de vuelta a su `origin`.
- Ningún canal puede usar un artefacto fuera del manifiesto autorizado.

### 5. ChannelPublicationState

Representa el estado observable de una versión en un canal de distribución.

**Fields**

- `channel`: nombre del canal (`github-release`, `npm`, `homebrew`).
- `version`: versión inspeccionada.
- `status`: estado (`absent`, `pending`, `materialized`, `incompatible`,
  `failed`).
- `observed_commit`: commit asociado cuando exista evidencia suficiente.
- `observed_artifact_ref`: referencia a artefacto o fórmula observada.
- `observed_at`: momento de la inspección.
- `details`: explicación resumida del estado.

**Validation rules**

- Un canal solo puede quedar `materialized` si la versión coincide con la
  release autorizada y la evidencia es compatible.
- `incompatible` debe bloquear cualquier re-publicación automática.
- `pending` nunca puede usarse para declarar éxito final.

### 6. ReleaseExecutionRecord

Representa la traza auditable completa de un intento de release.

**Fields**

- `run_id`: identificador único del intento.
- `intent`: referencia al `ReleaseIntent`.
- `authorized_source`: referencia a `AuthorizedReleaseSource`.
- `artifact_manifest`: referencia al `ArtifactManifest`, si existe.
- `channel_states`: lista de `ChannelPublicationState`.
- `overall_status`: clasificación global (`blocked`, `materialized`, `partial`,
  `completed`).
- `started_at`: momento de inicio.
- `finished_at`: momento de cierre.
- `next_safe_action`: siguiente paso recomendado para mantenimiento.

**Validation rules**

- `overall_status = completed` requiere `materialized` compatible en GitHub
  Release, npm y Homebrew.
- `overall_status = materialized` requiere que todos los canales ya estén
  correctamente presentes sin nueva publicación.
- `overall_status = partial` requiere al menos un canal materializado y otro no
  cerrado correctamente.
- `overall_status = blocked` requiere ausencia de publicación nueva o evidencia
  insuficiente para avanzar con seguridad.

## Relationships

- Un `ReleaseIntent` puede producir un `AuthorizedReleaseSource`.
- Un `AuthorizedReleaseSource` autorizado produce un `ArtifactManifest`.
- Un `ArtifactManifest` contiene varios `ReleaseArtifact`.
- Un `ReleaseExecutionRecord` agrega el `ReleaseIntent`, la fuente autorizada,
  el manifiesto y los estados por canal.
- Cada `ChannelPublicationState` debe poder trazarse a la misma `version` y al
  mismo `ArtifactManifest`.

## State Transitions

### Authorization

- `received` -> `blocked`
- `received` -> `authorized`

### Build and Publication

- `authorized` -> `artifacts_generated`
- `artifacts_generated` -> `github_release_materialized`
- `github_release_materialized` -> `npm_materialized`
- `npm_materialized` -> `homebrew_materialized`
- `homebrew_materialized` -> `completed`

### Recovery and Retry

- `authorized` -> `blocked` when consistency fails
- `github_release_materialized` -> `partial` when npm or Homebrew fail
- `npm_materialized` -> `partial` when Homebrew fails
- `partial` -> `materialized` when all channels are later confirmed correct
- `partial` -> `blocked` when inspection finds incompatibility
- `completed` -> `materialized` on future re-runs for the same version

## Invariants

- Ninguna release puede publicarse desde un commit distinto de `main@HEAD`.
- Ningún canal puede recibir una versión que no coincida con `Cargo.toml`.
- Todo estado final debe poder explicar qué commit se autorizó, qué versión se
  intentó y qué evidencia existe por canal.
- La misma versión nunca debe quedar simultáneamente `completed` e
  `incompatible`.
- Si no se puede verificar un hecho crítico, el sistema debe degradar a
  `blocked` o `partial`, nunca a `completed`.
