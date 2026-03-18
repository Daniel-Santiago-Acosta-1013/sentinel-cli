# Feature Specification: Automatización Segura de Releases

**Feature Branch**: `007-gate-release-publishing`  
**Created**: 2026-03-17  
**Status**: Draft  
**Input**: User description: "Redáctame una especificación completa en español para una nueva feature de CI/CD de Sentinel CLI que automatice releases y distribución con GitHub Actions, donde al crear un tag de versión válido se genere y publique esa versión en npm y Homebrew, pero con una regla estricta: la publicación solo puede ejecutarse si el tag apunta exactamente al commit HEAD actual de la rama main; si el tag apunta a un commit viejo de main, a otra rama, o a cualquier estado que no sea el último cambio autorizado de main, el release debe bloquearse de forma explícita. La especificación debe estar orientada al mantenedor del proyecto y enfocarse en trazabilidad, consistencia de versión, seguridad operativa y prevención de releases incorrectas, definiendo escenarios, criterios de aceptación, edge cases y requisitos funcionales claros para validar consistencia entre el tag, la versión del proyecto, los artefactos generados, npm y Homebrew, así como el comportamiento ante fallos parciales, reintentos, re-publicación de versiones existentes, desalineación entre canales y visibilidad del estado final del release. También debe dejar inequívocamente establecida la fuente de verdad de la release, las garantías de reproducibilidad y que la automatización siempre publica desde el estado más reciente y preciso de main, no simplemente desde cualquier tag existente."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Autorizar releases solo desde el HEAD vigente de main (Priority: P1)

Como mantenedor de Sentinel CLI, quiero que una release solo pueda arrancar si el
tag de versión apunta exactamente al commit que en ese momento es el HEAD actual
de `main`, para evitar publicar versiones desde commits viejos, ramas
incorrectas o estados no autorizados.

**Why this priority**: Esta es la barrera principal de seguridad operativa. Si
la autorización del release falla, todo el resto del flujo pierde confiabilidad.

**Independent Test**: Puede probarse creando tags de versión sobre el HEAD
vigente de `main`, sobre un commit anterior de `main`, y sobre commits ajenos a
`main`, verificando que solo el primer caso avanza y los demás se bloquean antes
de cualquier publicación externa.

**Acceptance Scenarios**:

1. **Given** que existe un tag de versión válido apuntando exactamente al HEAD
   actual de `main`, **When** la automatización evalúa la autorización del
   release, **Then** el release se marca como autorizado y puede avanzar a las
   validaciones de consistencia.
2. **Given** que existe un tag de versión válido apuntando a un commit antiguo
   que pertenece al historial de `main` pero no a su HEAD actual, **When** la
   automatización intenta iniciar el release, **Then** el release se bloquea de
   forma explícita antes de generar artefactos o publicar en canales externos.
3. **Given** que existe un tag de versión válido apuntando a un commit fuera del
   HEAD actual de `main`, **When** la automatización intenta iniciar el release,
   **Then** el release se bloquea con una razón visible que indique que el tag
   no representa el último cambio autorizado de `main`.

---

### User Story 2 - Publicar una versión consistente en todos los canales (Priority: P1)

Como mantenedor, quiero que la versión autorizada sea exactamente la misma en el
tag, en la declaración de versión del proyecto, en los artefactos generados y
en los registros publicados en npm y Homebrew, para evitar releases ambiguas o
inconsistentes entre canales.

**Why this priority**: El valor del release depende de que exista una sola
verdad de versión y que todos los artefactos distribuidos representen el mismo
estado exacto del producto.

**Independent Test**: Puede probarse con una versión alineada en todos los
componentes y con casos donde el tag, la declaración de versión o los
artefactos no coinciden, verificando que solo el caso alineado termina en
publicación completa.

**Acceptance Scenarios**:

1. **Given** que el tag autorizado y la declaración de versión del proyecto
   expresan la misma versión, **When** se generan los artefactos de release,
   **Then** todos los artefactos resultantes reportan esa misma versión sin
   discrepancias.
2. **Given** que el tag autorizado indica una versión distinta a la declarada en
   el proyecto, **When** la automatización valida la consistencia previa al
   release, **Then** el flujo se bloquea antes de publicar cualquier canal.
3. **Given** que la publicación se completa para npm y Homebrew, **When** el
   mantenedor revisa el estado final del release, **Then** puede confirmar que
   ambos canales exhiben la misma versión autorizada y corresponden al mismo
   conjunto de artefactos.

---

### User Story 3 - Recuperar control ante fallos parciales y reintentos (Priority: P2)

Como mantenedor, quiero que el sistema trate fallos parciales, reintentos y
versiones ya existentes de forma segura e idempotente, para no duplicar
publicaciones ni perder trazabilidad durante la recuperación.

**Why this priority**: Los problemas operativos no se pueden evitar por
completo; la seguridad del release depende de que los reintentos no empeoren un
estado ya inconsistente.

**Independent Test**: Puede probarse forzando un fallo después de publicar en un
canal pero antes del segundo, reintentando luego el mismo release, y ejecutando
un intento sobre una versión ya publicada, verificando que el sistema informa el
estado real y evita publicar de nuevo de forma incorrecta.

**Acceptance Scenarios**:

1. **Given** que la versión autorizada se publica en un canal y falla antes de
   completar el segundo, **When** finaliza la ejecución, **Then** el release no
   se marca como exitoso y muestra un estado parcial con detalle por canal.
2. **Given** que existe un estado parcial previo para la misma versión
   autorizada, **When** el mantenedor reintenta el release, **Then** el sistema
   conserva la trazabilidad del intento anterior y solo permite acciones seguras
   compatibles con el estado ya alcanzado.
3. **Given** que la versión ya existe íntegramente en los canales esperados y
   coincide con la release autorizada, **When** se reejecuta el proceso para esa
   misma versión, **Then** el sistema no duplica la publicación y reporta que la
   versión ya está materializada.

---

### User Story 4 - Auditar el estado final y la trazabilidad de la release (Priority: P2)

Como mantenedor, quiero ver un estado final inequívoco del release, incluyendo
qué commit se autorizó, qué versión se intentó publicar, qué pasó en cada canal
y cuál es la siguiente acción segura, para poder auditar y operar sin depender
de interpretación manual del pipeline.

**Why this priority**: Sin visibilidad final clara no hay cierre operativo ni
capacidad real de auditoría, especialmente cuando hay bloqueos o desalineación
entre canales.

**Independent Test**: Puede probarse ejecutando un release exitoso, uno
bloqueado y uno parcial, verificando que en todos los casos el mantenedor puede
identificar desde el resultado final el commit autorizado, el tag evaluado, la
versión y el estado de npm y Homebrew.

**Acceptance Scenarios**:

1. **Given** que un release termina con éxito completo, **When** el mantenedor
   consulta el resultado final, **Then** ve el commit autorizado, el tag, la
   versión y la confirmación de publicación satisfactoria en ambos canales.
2. **Given** que un release se bloquea por una regla de autorización o
   consistencia, **When** el mantenedor revisa el resultado final, **Then** ve
   la razón exacta del bloqueo y la confirmación de que no hubo publicación
   externa.
3. **Given** que un release termina con desalineación o fallo parcial, **When**
   el mantenedor revisa el resultado final, **Then** puede identificar qué
   canal quedó afectado, cuál fue el último estado confirmado y cuál es la
   siguiente acción segura.

### Edge Cases

- El tag de versión era válido cuando se creó, pero `main` avanzó antes de que
  iniciara la publicación; el release debe bloquearse porque el tag ya no apunta
  al HEAD vigente.
- El tag apunta a un commit que pertenece al historial de `main`, pero no al
  último commit autorizado; el release debe bloquearse aunque el commit sea
  alcanzable desde `main`.
- El tag apunta a un commit de otra rama que luego se mergeó parcialmente o fue
  cherry-picked; el release debe evaluar el commit exacto, no solo similitud de
  contenido.
- El tag tiene formato de versión válido, pero la declaración de versión del
  proyecto no coincide; el release debe bloquearse antes de generar artefactos.
- Los artefactos generados reportan una versión distinta a la autorizada o no
  pueden asociarse inequívocamente al commit autorizado; el release debe
  bloquearse.
- npm acepta la publicación pero Homebrew falla o queda pendiente; el estado
  final debe ser parcial, no exitoso.
- Uno de los canales ya contiene esa versión, pero el otro no, o ambos la
  contienen con evidencia distinta; el sistema debe marcar desalineación y no
  declarar éxito completo.
- Se reintenta una versión ya publicada correctamente en ambos canales; el
  sistema debe reconocerlo como materializado y evitar re-publicación.
- No es posible verificar el HEAD actual de `main`, resolver el commit del tag o
  comprobar el estado final en alguno de los canales; el release debe fallar en
  modo seguro y no asumir éxito.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: El sistema MUST considerar autorizable una release solo cuando
  exista un tag de versión válido que apunte exactamente al HEAD actual de la
  rama `main` en el momento de la validación.
- **FR-002**: El sistema MUST tratar como tags válidos para esta feature solo
  releases estables con formato de versión semántica `vMAJOR.MINOR.PATCH`.
- **FR-003**: El sistema MUST bloquear explícitamente cualquier release cuyo tag
  apunte a un commit antiguo de `main`, a un commit fuera de `main`, o a
  cualquier commit que no sea el HEAD vigente de `main`.
- **FR-004**: El sistema MUST volver a validar la relación entre tag y HEAD de
  `main` al iniciar la publicación efectiva, de modo que un tag creado sobre un
  HEAD previo no pueda publicarse si `main` avanzó antes de ese momento.
- **FR-005**: El sistema MUST establecer como fuente de verdad de una release la
  combinación formada por el commit autorizado en `main`, el tag de versión
  válido que apunta a ese commit y la declaración de versión del proyecto
  contenida en ese mismo commit.
- **FR-006**: El sistema MUST validar que la versión expresada por el tag
  coincida exactamente con la declaración de versión del proyecto antes de
  generar artefactos o publicar en cualquier canal.
- **FR-007**: El sistema MUST validar que todos los artefactos generados para la
  release representen exactamente la versión autorizada y puedan vincularse al
  commit autorizado de forma verificable.
- **FR-008**: El sistema MUST garantizar que npm y Homebrew reciban
  distribuciones derivadas del mismo commit autorizado y del mismo conjunto de
  artefactos aprobados.
- **FR-009**: El sistema MUST bloquear la publicación si detecta cualquier
  discrepancia entre el tag, la declaración de versión del proyecto, los
  artefactos generados o la información preparada para npm y Homebrew.
- **FR-010**: El sistema MUST impedir cualquier publicación externa cuando la
  release quede en estado bloqueado durante las validaciones previas.
- **FR-011**: El sistema MUST registrar para cada intento de release, como
  mínimo, el tag evaluado, el commit resuelto, la versión objetivo, la hora de
  inicio, la hora de fin y el resultado por etapa.
- **FR-012**: El sistema MUST exponer un estado final inequívoco para cada
  intento con una de estas clasificaciones: bloqueado, materializado, parcial o
  completado.
- **FR-013**: El sistema MUST exponer el resultado por canal de distribución de
  manera separada, permitiendo distinguir claramente el estado de npm y el de
  Homebrew.
- **FR-014**: El sistema MUST tratar los reintentos de la misma versión
  autorizada de forma idempotente, evitando duplicar publicaciones ya
  completadas o sobrescribir un estado confirmado.
- **FR-015**: El sistema MUST reconocer cuando una versión ya existe
  íntegramente en npm y Homebrew y coincide con la release autorizada, marcando
  el resultado como materializado en lugar de intentar una nueva publicación.
- **FR-016**: El sistema MUST detectar y señalar como desalineación cualquier
  caso en el que una versión exista en un canal pero no en el otro, o en el que
  los canales exhiban evidencia incompatible para una misma versión.
- **FR-017**: El sistema MUST detener el avance automático hacia éxito completo
  cuando exista un fallo parcial o desalineación entre canales, incluso si uno
  de los canales ya fue publicado con éxito.
- **FR-018**: El sistema MUST preservar la trazabilidad de fallos parciales,
  incluyendo qué pasos sí ocurrieron, cuáles no ocurrieron y cuál es la
  siguiente acción segura disponible para el mantenedor.
- **FR-019**: El sistema MUST bloquear la re-publicación de una versión que ya
  exista en un canal con evidencia incompatible respecto a la release autorizada
  hasta que el mantenedor resuelva la discrepancia.
- **FR-020**: El sistema MUST fallar en modo seguro cuando no pueda comprobar la
  relación exacta entre el tag y `main`, determinar la versión autorizada o
  confirmar el estado real de npm y Homebrew.
- **FR-021**: El sistema MUST permitir que un mantenedor determine, a partir del
  resultado final visible, si la release fue publicada desde el último cambio
  autorizado de `main` sin necesidad de inspeccionar manualmente la historia del
  repositorio.
- **FR-022**: El sistema MUST incluir en el estado final visible el commit
  autorizado, el tag evaluado, la versión objetivo, el estado por canal y la
  razón del resultado global.
- **FR-023**: El sistema MUST garantizar reproducibilidad operativa suficiente
  para reconstruir qué release se intentó, desde qué fuente de verdad se derivó
  y qué artefactos fueron considerados válidos para la distribución.
- **FR-024**: El sistema MUST rechazar la automatización de prereleases,
  versiones con sufijos no estables o publicaciones a canales distintos de npm y
  Homebrew dentro del alcance de esta feature.

### Key Entities *(include if feature involves data)*

- **Release Candidate**: Representa una intención de release evaluada a partir
  de un tag de versión, un commit resuelto y una declaración de versión del
  proyecto.
- **Authorized Release Source**: Representa la fuente de verdad compuesta por el
  HEAD vigente de `main`, el tag válido asociado y la versión declarada en ese
  mismo estado.
- **Version Tag**: Representa el identificador versionado que dispara la
  automatización y que debe coincidir exactamente con la versión autorizada.
- **Project Version Record**: Representa la declaración oficial de versión
  incluida en el proyecto y utilizada para comprobar consistencia antes de
  publicar.
- **Release Artifact Set**: Representa el conjunto de distribuciones generadas a
  partir de la release autorizada y verificadas antes de salir a los canales.
- **Channel Publication State**: Representa el estado observable de una versión
  en cada canal de distribución, incluyendo ausencia, materialización,
  incompatibilidad o publicación parcial.
- **Release Execution Record**: Representa la traza visible de un intento de
  release con sus tiempos, validaciones, decisiones de bloqueo y resultados por
  etapa.

## Assumptions & Scope Boundaries *(optional)*

- Esta feature cubre solo releases estables oficiales de Sentinel CLI
  distribuidas en npm y Homebrew.
- Se asume que `main` es la rama protegida que representa el último estado
  autorizado para publicación.
- Se asume que el proyecto mantiene una única declaración oficial de versión por
  commit y que esa declaración puede compararse de forma determinista con el
  tag.
- La eliminación o reversión de versiones ya publicadas queda fuera de alcance;
  esta feature cubre detección, bloqueo, trazabilidad y recuperación segura.
- La definición concreta del pipeline, sus jobs y su infraestructura de
  ejecución se resolverá en planificación; esta especificación solo define el
  comportamiento esperado y sus garantías operativas.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: El 100% de los intentos de release cuyo tag no apunte al HEAD
  vigente de `main` quedan bloqueados antes de cualquier publicación externa.
- **SC-002**: El 100% de las releases completadas exhiben la misma versión en el
  tag autorizado, la declaración de versión del proyecto, los artefactos
  generados y los dos canales de distribución.
- **SC-003**: El 100% de los intentos de release terminan con un estado final
  visible para el mantenedor en un máximo de 15 minutos desde el inicio del
  intento o desde la primera condición de bloqueo definitiva.
- **SC-004**: En el 100% de los fallos parciales o desalineaciones, el
  mantenedor puede identificar en menos de 5 minutos el commit autorizado, la
  versión afectada, el canal comprometido y la siguiente acción segura.
- **SC-005**: Al menos el 95% de las releases autorizadas desde el HEAD vigente
  de `main` se completan sin intervención manual adicional.
