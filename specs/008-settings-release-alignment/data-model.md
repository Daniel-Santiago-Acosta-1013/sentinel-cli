# Data Model: Ajustes de Bloqueo y Release Alineado

## Overview

El feature introduce dos grupos de entidades: las necesarias para administrar el
listado activo de dominios bloqueados y resumir actividad de bloqueo en la CLI,
y las necesarias para centralizar y auditar el versionado/release desde el
workflow oficial.

## Entities

### 1. SettingsRoute

Representa la rama de navegación interactiva asociada a `Ajustes`.

**Fields**

- `id`: identificador estable de la ruta.
- `title`: texto visible de la vista.
- `parent_route`: ruta superior desde la cual se accede.
- `menu_items`: acciones visibles en la vista.
- `allows_back`: indica si permite volver al nivel anterior.

**Validation rules**

- `id` debe ser único dentro del árbol de navegación.
- Toda ruta bajo `Ajustes` debe poder volver a `Home` o a su vista padre.

### 2. BlockedDomainEntry

Representa un dominio bloqueado que forma parte del listado activo gestionable.

**Fields**

- `id`: identificador estable de la entrada.
- `domain`: dominio normalizado.
- `source`: origen de la entrada (`seeded`, `user_added`, `user_edited`).
- `enabled`: indica si participa en el listado efectivo.
- `created_at`: momento de alta inicial.
- `updated_at`: momento de la última modificación.

**Validation rules**

- `domain` debe guardarse en minúsculas, sin punto final y con formato de
  dominio válido.
- No puede existir más de una entrada activa con el mismo `domain`.
- Toda edición debe actualizar `updated_at`.

### 3. BlockedDomainCatalog

Representa el conjunto persistido de dominios bloqueados usado por Sentinel.

**Fields**

- `entries`: lista ordenada de `BlockedDomainEntry`.
- `seed_version`: referencia a la base desde la que se inicializó el catálogo.
- `last_synced_at`: momento de la última carga o reescritura válida.
- `integrity_ok`: indica si el catálogo pasó validación estructural.

**Validation rules**

- `entries` debe permanecer deduplicada por dominio normalizado.
- Si `integrity_ok` es falso, la CLI no debe aplicar cambios silenciosamente.
- El catálogo efectivo debe poder regenerarse en orden estable para la UI y el
  runtime.

### 4. BlockActivityRecord

Representa un evento estructurado de bloqueo de dominio.

**Fields**

- `event_id`: identificador único.
- `blocked_domain`: dominio que disparó el bloqueo.
- `blocked_at`: momento del evento.
- `source`: origen del bloqueo (`runtime_dns`, equivalente operativo).
- `result`: estado del manejo (`blocked`, `skipped`, `error`).

**Validation rules**

- Solo eventos con `result = blocked` alimentan la tabla de actividad.
- `blocked_domain` debe estar normalizado con la misma regla del catálogo.
- `blocked_at` debe ser monotónico respecto al orden persistido en la traza.

### 5. BlockActivitySnapshot

Representa el resumen visible en `Estado de Sentinel`.

**Fields**

- `blocked_since_activation`: número total de bloqueos desde la activación.
- `unique_blocked_domains`: cantidad de dominios únicos bloqueados.
- `last_blocked_at`: timestamp del último bloqueo, si existe.
- `top_blocked_domains`: lista ordenada de dominios con mayor recurrencia.

**Validation rules**

- `blocked_since_activation` debe ser mayor o igual a
  `unique_blocked_domains`.
- `top_blocked_domains` debe ordenarse por frecuencia descendente y usar un
  desempate estable.
- Si no existen eventos, todas las métricas deben representar estado vacío sin
  inventar datos.

### 6. VersionAlignmentRequest

Representa la solicitud explícita de una nueva versión dentro del workflow
oficial.

**Fields**

- `requested_version`: versión objetivo estable.
- `requested_by`: actor o ejecución que la inició.
- `requested_at`: momento de inicio.
- `surfaces_to_update`: inventario de superficies dependientes de versión.

**Validation rules**

- `requested_version` debe seguir el formato estable permitido por el proyecto.
- `surfaces_to_update` no puede estar vacío.

### 7. VersionSurface

Representa un archivo o superficie del proyecto cuyo contenido depende de la
versión oficial.

**Fields**

- `path`: ubicación de la superficie.
- `surface_type`: categoría (`manifest`, `template`, `script`, `fixture`,
  `doc`, `runtime_version`).
- `current_version`: versión detectada antes del cambio.
- `target_version`: versión esperada después del cambio.
- `status`: estado de alineación (`pending`, `updated`, `validated`, `failed`).

**Validation rules**

- Toda superficie obligatoria debe terminar en `validated` antes de crear el
  tag.
- Si una superficie queda en `failed`, el workflow no puede continuar al job de
  release.

### 8. CentralizedReleaseExecution

Representa la ejecución completa del flujo oficial de versionado y release.

**Fields**

- `run_id`: identificador único de ejecución.
- `alignment_request`: referencia a `VersionAlignmentRequest`.
- `version_commit`: commit generado por el job previo.
- `release_tag`: tag creado desde CI.
- `artifact_version`: versión de artefactos generados.
- `channel_states`: estado observable por canal.
- `overall_status`: resultado global (`blocked`, `partial`, `completed`).
- `next_safe_action`: siguiente paso recomendado.

**Validation rules**

- `release_tag` debe corresponder exactamente a `requested_version`.
- `artifact_version` debe coincidir con la versión alineada en todas las
  superficies obligatorias.
- `overall_status = completed` requiere coincidencia entre repositorio, tag,
  artefactos, npm y Homebrew.

## Relationships

- Una `SettingsRoute` puede contener una vista de `BlockedDomainCatalog`.
- Un `BlockedDomainCatalog` contiene varias `BlockedDomainEntry`.
- Un `BlockActivitySnapshot` se deriva de múltiples `BlockActivityRecord`.
- Un `VersionAlignmentRequest` contiene varias `VersionSurface`.
- Una `CentralizedReleaseExecution` depende de un `VersionAlignmentRequest`
  validado y agrega el estado final por canal.

## State Transitions

### Blocked Domain Management

- `seeded` -> `user_edited`
- `seeded` -> `disabled/removed`
- `user_added` -> `user_edited`
- `user_added` -> `disabled/removed`

### Blocking Activity

- `no_data` -> `has_events`
- `has_events` -> `has_more_events`

### Version Alignment and Release

- `requested` -> `aligning_surfaces`
- `aligning_surfaces` -> `validated`
- `aligning_surfaces` -> `blocked`
- `validated` -> `version_commit_created`
- `version_commit_created` -> `tag_created`
- `tag_created` -> `release_running`
- `release_running` -> `completed`
- `release_running` -> `partial`
- `release_running` -> `blocked`

## Invariants

- El home nunca vuelve a mostrar la insignia `✓ Riesgo: Normal`.
- El listado activo de dominios bloqueados debe ser único, legible y editable
  desde la CLI sin duplicados válidos.
- La tabla `Actividad de bloqueo` solo muestra las cuatro métricas pedidas.
- Ninguna release centralizada puede crear tag o publicar si alguna superficie
  requerida de versión no quedó validada.
- Ninguna ejecución puede declararse completa si el estado final entre
  repositorio, artefactos, npm y Homebrew no coincide exactamente.
