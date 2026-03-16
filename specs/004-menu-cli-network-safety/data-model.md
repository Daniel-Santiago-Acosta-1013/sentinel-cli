# Data Model: CLI Guiada por Menús Segura

## MenuSession

**Purpose**: Representa la sesión interactiva actual de la CLI y la posición
del usuario dentro del flujo guiado.

**Fields**:
- `view_id`: vista activa (`inicio | seguridad | estado | recuperacion | confirmacion | salida`)
- `selected_index`: opción actualmente seleccionada en el menú visible
- `available_actions`: acciones visibles en la vista actual
- `status_summary`: resumen corto de protección y red para el encabezado
- `last_message`: último mensaje de resultado o guía
- `pending_confirmation`: acción sensible pendiente de confirmación
- `transcript_mode`: indica si la sesión está corriendo en modo scriptado para pruebas

**Validation Rules**:
- `selected_index` MUST apuntar a una acción válida de `available_actions`.
- `pending_confirmation` MUST existir solo para acciones que mutan red o
  restauran estado.
- `view_id=confirmacion` MUST incluir una acción pendiente antes de renderizarse.

## TerminalViewModel

**Purpose**: Representa el contenido listo para renderizar en una pantalla
limpia de terminal.

**Fields**:
- `title`: título visible de la pantalla
- `intro_text`: explicación breve del contexto actual
- `status_table`: filas tabulares de estado cuando aplica
- `action_list`: lista de acciones navegables
- `footer_hint`: ayuda breve de teclado en español
- `clear_before_render`: indica si debe limpiarse/refrescarse la terminal antes de imprimir
- `width_mode`: modo de presentación (`normal | compacta`)

**Validation Rules**:
- `status_table` MUST contener encabezados en español si la vista expone estado.
- `clear_before_render` MUST ser verdadero en cambios de vista y resultados
  finales para evitar texto acumulado.
- `width_mode=compacta` MUST omitir columnas no esenciales cuando el ancho de la
  terminal sea insuficiente.

## NetworkConfigurationSnapshot

**Purpose**: Captura la configuración original de red del usuario antes de una
operación sensible y permite restaurarla con verificación posterior.

**Fields**:
- `snapshot_id`: identificador único
- `captured_at`: fecha y hora de captura
- `services`: servicios de red incluidos en la captura
- `dns_servers_by_service`: servidores DNS originales por servicio
- `restorable`: indica si la captura es válida para restauración
- `source_reason`: motivo de captura (`activacion | recuperacion | validacion`)

**Validation Rules**:
- `services` MUST no estar vacío antes de permitir cambios de red.
- Cada servicio en `services` MUST tener una entrada correspondiente en
  `dns_servers_by_service`, incluso si la lista de DNS original está vacía.
- `restorable` MUST ser verdadero antes de activar protección.

## SafetyCheckResult

**Purpose**: Resume si Sentinel puede cambiar la red del equipo sin comprometer
la conectividad.

**Fields**:
- `check_id`: identificador del chequeo
- `status`: `pass | warn | fail`
- `issues`: observaciones visibles para el usuario
- `connectivity_ready`: indica si la conectividad mínima es segura
- `recovery_ready`: indica si existe ruta de rollback
- `recommended_action`: siguiente paso sugerido en español
- `detected_custom_dns`: indica si el usuario ya tenía DNS personalizados
- `verification_target`: snapshot o estado que debe poder restaurarse

**Validation Rules**:
- `status=fail` MUST bloquear activación o desactivación sensible.
- `recovery_ready` MUST ser verdadero antes de permitir activación.
- `recommended_action` MUST estar en español y mapearse a una acción visible o
  a una salida segura.

## ProtectionWorkflowState

**Purpose**: Representa el estado operativo persistido de Sentinel respecto a la
protección y recuperación.

**Fields**:
- `mode`: `inactive | active | degraded | recovering`
- `runtime_pid`: proceso de runtime local cuando aplica
- `runtime_addr`: dirección local usada por el runtime cuando aplica
- `active_snapshot_id`: snapshot asociado a la protección activa
- `risk_level`: `normal | warning | critical`
- `status_summary`: resumen persistido mostrado al usuario
- `last_transition_at`: fecha y hora de la última transición
- `last_verification_result`: resultado de la verificación posterior a restauración

**State Transitions**:
- `inactive -> active`: solo después de chequeos de seguridad exitosos y
  snapshot restorable.
- `active -> inactive`: solo después de restauración y verificación del estado
  original.
- `active -> degraded`: cuando la restauración o verificación falla.
- `degraded -> recovering`: cuando el usuario inicia recuperación guiada.
- `recovering -> inactive`: cuando la recuperación restaura y verifica estado válido.

**Validation Rules**:
- `mode=active` MUST tener `active_snapshot_id`.
- `mode=degraded` MUST acompañarse de un mensaje claro y una acción de
  recuperación disponible.
- `last_verification_result` MUST registrar si el estado restaurado coincide o
  no con el snapshot esperado.

## StatusSummary

**Purpose**: Modelo estable de la información tabular visible al usuario para
comparar protección, red y seguridad.

**Fields**:
- `section`: nombre lógico de la tabla (`proteccion | red | seguridad | recuperacion`)
- `rows`: pares `campo -> valor`
- `severity`: nivel visual de la sección (`normal | warning | critical`)
- `compactable`: indica si la tabla puede reducir columnas en terminales angostas

**Validation Rules**:
- Cada `field` MUST estar en español y ser estable para contratos y snapshots.
- `rows` MUST incluir la siguiente acción recomendada en vistas críticas.
- `compactable=true` MUST conservar los campos esenciales de seguridad y estado
  aunque se reduzca el ancho.
