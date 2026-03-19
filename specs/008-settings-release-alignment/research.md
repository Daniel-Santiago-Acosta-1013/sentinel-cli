# Research: Ajustes de Bloqueo y Release Alineado

## Decision 1: El listado activo de dominios bloqueados pasa a ser una fuente local administrable, inicializada desde la base actual

**Decision**: Sentinel debe mantener un listado local administrable de dominios
bloqueados como fuente efectiva usada por la CLI y por el runtime. Ese listado
se inicializa desde la base actual solo cuando no exista un archivo gestionado,
y a partir de entonces las operaciones de agregar, editar y eliminar actúan
sobre ese estado local persistido.

**Rationale**: El requisito pide administración completa de los dominios
actualmente bloqueados. Un overlay parcial o una vista solo de lectura sobre la
lista built-in no daría control total al usuario. Convertir la copia local en
fuente efectiva preserva la experiencia pedida y evita que cambios manuales se
pierdan al reiniciar.

**Alternatives considered**:

- Mantener una lista built-in inmutable y agregar solo un overlay de entradas
  adicionales. Rechazado porque no permitiría editar o eliminar dominios ya
  bloqueados.
- Mostrar el listado vigente pero editarlo fuera de Sentinel. Rechazado porque
  rompe el objetivo de administración completa dentro del flujo menu-driven.
- Guardar dominios administrables dentro de `config.toml` como un campo más.
  Rechazado porque mezcla configuración general con un recurso operativo de alto
  volumen y evolución distinta.

## Decision 2: La actividad de bloqueo se deriva de eventos estructurados persistidos en `events.jsonl`

**Decision**: La tabla `Actividad de bloqueo` debe construirse a partir de
eventos estructurados de bloqueo persistidos en el store de eventos existente,
agregados en tiempo de lectura para exponer las cuatro métricas pedidas sin
introducir una base de datos nueva.

**Rationale**: Sentinel ya tiene una traza JSONL operativa. Extender esa traza
con un tipo de evento específico de bloqueo permite obtener `Bloqueos desde la
activación`, `Dominios únicos bloqueados`, `Último bloqueo` y `Top dominios
bloqueados` con trazabilidad y sin abrir una segunda fuente de verdad.

**Alternatives considered**:

- Guardar un contador agregado aparte en `state.json`. Rechazado porque pierde
  detalle para recomputar top dominios y complica auditoría.
- Calcular métricas solo en memoria durante la ejecución activa. Rechazado
  porque no sobrevive reinicios ni alimenta una vista consistente.
- Crear una base de datos o almacén dedicado para actividad. Rechazado por
  complejidad innecesaria para el alcance actual.

## Decision 3: `Ajustes` se integra extendiendo el modelo actual de rutas y vistas, sin crear una segunda experiencia de interacción

**Decision**: La nueva navegación debe integrarse ampliando `Route`,
`MenuActionId`, `MenuSession` y las vistas existentes dentro de `src/cli`, de
modo que `Ajustes` y `Dominios bloqueados` sigan exactamente el flujo actual de
pantallas limpias y opciones seleccionables.

**Rationale**: El producto ya tiene una base menu-driven coherente. Extender ese
modelo reduce riesgo, preserva copy y navegación existentes, y evita bifurcar la
UX con modales o flujos especiales.

**Alternatives considered**:

- Implementar `Dominios bloqueados` como un comando separado. Rechazado por
  violar la constitución de superficie mínima.
- Resolver CRUD con prompts ad hoc fuera del sistema de rutas. Rechazado porque
  rompería la consistencia visual y de navegación.
- Crear una subaplicación o vista pseudo-TUI independiente. Rechazado porque
  contradice la línea estética y arquitectónica ya definida.

## Decision 4: El release se divide en dos jobs: versionado auditable y release/deploy

**Decision**: El workflow oficial debe empezar por un job de versionado que
recibe la nueva versión, actualiza todas las superficies, valida alineación,
genera un commit auditable y crea el tag; solo después un segundo job ejecuta
build, release y deploy usando ese estado ya fijado.

**Rationale**: Separar versionado de publicación reduce estados ambiguos. El tag
deja de ser un prerrequisito manual y pasa a ser una consecuencia del estado
versionado aprobado, lo que elimina la fuente principal de desalineación entre
repo y canales.

**Alternatives considered**:

- Mantener el trigger por `push` de tags. Rechazado porque conserva la
  dependencia de tags locales y el riesgo de desalineación manual.
- Hacer versionado y deploy dentro de un solo job. Rechazado porque mezcla
  preparación y distribución sin una frontera auditable clara.
- Crear commit sin crear tag en CI. Rechazado porque el tag sigue siendo parte
  del contrato final de release y debe nacer del mismo estado aprobado.

## Decision 5: La alineación de versión debe centralizarse en un único script de inventario explícito

**Decision**: El job previo de versionado debe delegar la actualización y
validación de superficies dependientes a un único script de alineación con un
inventario explícito de archivos sensibles a versión.

**Rationale**: La versión hoy afecta `Cargo.toml`, plantillas de packaging,
scripts y fixtures de pruebas. Centralizar el cambio en un solo script permite
auditar qué se actualizó, detectar omisiones y evitar que cada job o script
reimplemente la misma lógica.

**Alternatives considered**:

- Actualizar archivos inline dentro del workflow YAML. Rechazado por
  mantenibilidad baja y escasa testabilidad.
- Confiar únicamente en `Cargo.toml` y placeholders al momento de publicar.
  Rechazado porque deja superficies internas y fixtures potencialmente
  desalineadas.
- Actualizar manualmente archivos secundarios. Rechazado porque el usuario pidió
  centralización precisa y consistente.

## Decision 6: La validación de consistencia final debe abarcar repositorio, artefactos y canales

**Decision**: El release no debe declararse exitoso solo porque el job de
publicación terminó; debe comprobar que la versión en el repositorio, el tag
creado, los artefactos generados y los estados observables de npm/Homebrew
coinciden exactamente.

**Rationale**: El problema explícito a resolver es la desalineación entre
canales. Si el pipeline no compara esos estados al final, puede seguir
reportando éxito falso después de una publicación parcial o inconsistente.

**Alternatives considered**:

- Dar por bueno cualquier job que complete sin error técnico. Rechazado porque
  no prueba consistencia real.
- Verificar solo el tag y los artefactos locales. Rechazado porque no cubre el
  estado final de npm y Homebrew.
- Verificar canales pero no el commit/estado versionado del repositorio.
  Rechazado porque deja abierta la posibilidad de publicar desde una base no
  auditable.
