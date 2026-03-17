# Data Model: Rediseño de CLI por Vistas

## Overview

El feature no introduce una base de datos nueva, pero sí redefine el modelo
interactivo visible al usuario para soportar navegación por vistas
independientes, feedback de progreso y resultados finales claros.

## Entities

### 1. CliRoute

Representa una vista o destino navegable dentro de la experiencia interactiva.

**Fields**

- `id`: identificador estable de la vista.
- `title`: título visible de la vista.
- `kind`: categoría de la vista (`home`, `action`, `status`, `confirmation`,
  `result`, `exit`).
- `allows_back`: indica si permite volver a la vista anterior.
- `clear_on_enter`: indica si la terminal debe limpiarse al entrar.
- `show_logo`: indica si la vista incluye branding ASCII.
- `primary_action`: acción principal disponible en la vista.
- `secondary_actions`: acciones adicionales o de navegación.

**Validation rules**

- `id` debe ser único en toda la experiencia interactiva.
- Solo la vista `home` puede tener `show_logo = true`.
- Toda vista distinta de `home` debe definir una forma clara de volver o salir.

### 2. HomeMenuItem

Representa una opción seleccionable del home.

**Fields**

- `id`: identificador estable de la opción.
- `label`: texto principal mostrado al usuario.
- `description`: apoyo breve para entender la acción.
- `target_route`: ruta que se abre al seleccionar la opción.
- `availability`: condición visible de disponibilidad según el estado actual.
- `emphasis`: nivel visual para resaltar prioridad o riesgo.

**Validation rules**

- Todo `target_route` debe apuntar a una `CliRoute` válida.
- El home debe contener solo acciones principales y no duplicar navegación
  secundaria.

### 3. NavigationSession

Representa el estado de navegación durante una sesión interactiva.

**Fields**

- `current_route`: ruta activa.
- `previous_route`: ruta previa para navegación de regreso.
- `selected_index`: opción actualmente resaltada en la vista activa.
- `pending_confirmation`: acción sensible pendiente de confirmar.
- `pending_operation`: operación en curso, si existe.
- `last_result`: último resultado visible o resumido.
- `runtime_state`: estado operativo actual reutilizado desde Sentinel.
- `install_state`: estado actual de instalación reutilizado desde Sentinel.
- `script_mode`: indica si la sesión está siendo ejecutada por transcript de
  pruebas.

**Validation rules**

- `selected_index` debe mantenerse dentro del rango de opciones de la vista.
- Una `pending_confirmation` solo puede existir en rutas de confirmación.
- Una `pending_operation` activa bloquea transiciones no seguras hasta terminar
  o cancelarse de forma explícita.

### 4. OperationFeedback

Representa el progreso y resultado visible de una acción.

**Fields**

- `operation_id`: identificador de la acción en curso.
- `phase`: estado visible (`idle`, `running`, `success`, `warning`, `error`).
- `spinner_label`: texto corto mostrado mientras la operación sigue activa.
- `summary`: resultado corto para encabezado o bloque principal.
- `details`: explicación adicional del resultado.
- `next_step`: siguiente acción recomendada al usuario.
- `can_return_home`: indica si el usuario puede volver directamente al home.

**Validation rules**

- `spinner_label` solo debe mostrarse cuando `phase = running`.
- Todo resultado terminal (`success`, `warning`, `error`) debe definir
  `summary` y `next_step`.

### 5. ViewFrame

Representa la composición visible de una pantalla de CLI ya preparada para
render.

**Fields**

- `header`: encabezado principal.
- `body_sections`: bloques centrales de contenido.
- `status_badges`: señales breves de estado/riesgo.
- `menu_items`: opciones interactivas visibles.
- `footer_hint`: ayuda breve de navegación.
- `visual_cues`: conjunto de color, símbolos y énfasis aplicables.
- `preserve_result_until_exit`: indica si la vista debe mantener el resultado
  hasta que el usuario navegue manualmente.

**Validation rules**

- El frame debe poder renderizarse de forma legible tanto con estilo enriquecido
  como en modo plano.
- Una vista de resultado debe preservar el mensaje final hasta que el usuario
  decida regresar o salir.

## Relationships

- Un `HomeMenuItem` apunta a una `CliRoute`.
- Una `NavigationSession` mantiene una `CliRoute` activa y opcionalmente una
  `CliRoute` previa.
- Una `NavigationSession` puede contener un `OperationFeedback` activo.
- Una `CliRoute` se renderiza como un `ViewFrame`.

## State Transitions

### Navigation

- `home` -> `action/status/confirmation`
- `action` -> `result`
- `status` -> `home`
- `confirmation` -> `result` o `home`
- `result` -> `home` o `exit`
- `recovery-first route` -> `confirmation` -> `result` -> `home`

### Operation Feedback

- `idle` -> `running`
- `running` -> `success`
- `running` -> `warning`
- `running` -> `error`
- `success|warning|error` -> `idle` when user starts a new action

## Invariants

- El home siempre debe ser identificable como el punto principal de navegación.
- Ninguna transición de vista debe dejar texto residual que confunda el estado
  actual.
- Toda acción sensible debe pasar por confirmación o por una advertencia
  equivalente antes de cambiar la red.
- El flujo interactivo debe conservar la capacidad de mostrar estado y
  recuperación incluso cuando una operación previa haya fallado.
