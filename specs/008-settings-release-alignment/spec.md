# Feature Specification: Ajustes de Bloqueo y Release Alineado

**Feature Branch**: `008-settings-release-alignment`  
**Created**: 2026-03-19  
**Status**: Draft  
**Input**: User description: "Necesito una especificación en español para Sentinel CLI que preserve la línea estética actual del producto y contemple estos cambios funcionales y de release: en el home se debe eliminar por completo la indicación `✓ Riesgo: Normal`, manteniendo el resto del diseño limpio y coherente; además se debe agregar una nueva opción principal llamada `Ajustes`, y dentro de ella una opción `Dominios bloqueados`, donde el usuario pueda ver los dominios que actualmente se están bloqueando y administrarlos de forma completa, incluyendo agregar, editar, eliminar y revisar el listado vigente, sin romper la experiencia visual ni el flujo menu-driven actual. También se debe mejorar la vista `Estado de Sentinel`, recortando la tabla actual para eliminar los campos `Riesgo`, `Resumen` y `Accion sugerida`, y en esa misma vista se debe agregar una nueva sección llamada `Actividad de bloqueo`, presentada como tabla, que incluya únicamente estas métricas: `Bloqueos desde la activación`, `Dominios únicos bloqueados`, `Último bloqueo` y `Top dominios bloqueados`. Por otro lado, se debe modificar el workflow de release actual para que la creación de versiones quede totalmente centralizada en GitHub Actions: ya no se crearán ni pushearán tags localmente, sino que el workflow recibirá como input la nueva versión, ejecutará primero un job previo al release que actualice automáticamente la versión en `Cargo.toml` y en todos los lugares del proyecto donde se muestre o dependa de la versión para garantizar alineamiento total, generará un commit con esos cambios, creará el tag correspondiente y luego ejecutará en un job separado el proceso de release y deploy, evitando inconsistencias entre tag, versión interna, npm y Homebrew. La especificación debe dejar claro que la automatización del versionado y del release debe ser precisa, trazable y consistente para evitar problemas de despliegue y desalineación entre canales."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Entrar a un home más limpio sin perder orientación (Priority: P1)

Como usuario de Sentinel CLI, quiero abrir el home sin la indicación `✓ Riesgo:
Normal` y seguir viendo un menú principal claro y coherente, para entender de
inmediato qué puedo hacer sin ruido visual innecesario.

**Why this priority**: El home es la entrada a toda la experiencia. Si la
pantalla inicial no queda limpia ni consistente con la estética actual, el resto
de la mejora pierde valor percibido.

**Independent Test**: Puede probarse iniciando la CLI y verificando que el home
ya no muestra la indicación eliminada, mantiene su línea visual actual y ofrece
la nueva opción principal `Ajustes` sin alterar el flujo guiado por menús.

**Acceptance Scenarios**:

1. **Given** que el usuario abre Sentinel CLI en una terminal interactiva,
   **When** aparece el home, **Then** la indicación `✓ Riesgo: Normal` no está
   presente y el resto del diseño principal se mantiene limpio y reconocible.
2. **Given** que el usuario está en el home, **When** revisa las opciones
   principales, **Then** encuentra `Ajustes` al mismo nivel de navegación que
   las demás entradas principales.
3. **Given** que el usuario vuelve al home desde otra vista, **When** se
   completa la navegación, **Then** el home reaparece sin residuos visuales y
   con el mismo orden y claridad de opciones esperados.

---

### User Story 2 - Administrar dominios bloqueados desde Ajustes (Priority: P1)

Como usuario, quiero entrar a `Ajustes` y administrar completamente los
dominios bloqueados, para poder revisar qué está bloqueando Sentinel y mantener
esa lista actualizada sin salir del flujo guiado por menús.

**Why this priority**: La nueva capacidad funcional principal es dar control
visible y completo sobre los dominios bloqueados. Sin esta historia, la nueva
sección `Ajustes` sería solo un contenedor vacío.

**Independent Test**: Puede probarse entrando a `Ajustes` > `Dominios
bloqueados` y confirmando que el usuario puede revisar el listado vigente,
agregar un dominio, editar uno existente y eliminarlo, siempre regresando a una
vista consistente del listado.

**Acceptance Scenarios**:

1. **Given** que el usuario entra a `Ajustes` y selecciona `Dominios
   bloqueados`, **When** se abre la vista, **Then** ve el listado vigente de
   dominios bloqueados o un estado vacío claro si todavía no existe ninguno.
2. **Given** que el usuario está en la vista `Dominios bloqueados`, **When**
   agrega un nuevo dominio válido, **Then** el dominio aparece en el listado
   vigente sin romper la navegación ni la estética de la vista.
3. **Given** que el usuario está revisando un dominio ya registrado, **When**
   elige editarlo o eliminarlo y confirma la acción, **Then** el listado se
   actualiza de forma visible y mantiene una ruta clara para continuar o volver.

---

### User Story 3 - Revisar estado y actividad real de bloqueo (Priority: P2)

Como usuario, quiero abrir `Estado de Sentinel` y ver una vista más enfocada,
sin columnas accesorias y con una sección específica de actividad de bloqueo,
para entender rápidamente qué está ocurriendo con la protección sin leer
información irrelevante.

**Why this priority**: Esta mejora refuerza comprensión y confianza operativa,
pero depende de que el home y la administración de dominios ya existan dentro
del flujo visual coherente.

**Independent Test**: Puede probarse entrando a `Estado de Sentinel` y
verificando que la tabla principal ya no muestra `Riesgo`, `Resumen` ni
`Accion sugerida`, y que además existe una sección `Actividad de bloqueo` con
exactamente las cuatro métricas pedidas.

**Acceptance Scenarios**:

1. **Given** que el usuario abre `Estado de Sentinel`, **When** revisa la tabla
   principal, **Then** no encuentra los campos `Riesgo`, `Resumen` ni `Accion
   sugerida`.
2. **Given** que el usuario permanece en `Estado de Sentinel`, **When** revisa
   la nueva sección `Actividad de bloqueo`, **Then** ve únicamente `Bloqueos
   desde la activación`, `Dominios únicos bloqueados`, `Último bloqueo` y `Top
   dominios bloqueados`.
3. **Given** que todavía no existe actividad de bloqueo suficiente, **When** el
   usuario abre esa sección, **Then** cada métrica muestra un estado vacío o
   sin datos de forma comprensible, sin inventar información.

---

### User Story 4 - Publicar versiones totalmente alineadas desde el flujo oficial (Priority: P1)

Como mantenedor de Sentinel CLI, quiero iniciar una nueva versión desde el
flujo oficial centralizado del repositorio, para que la actualización de
versiones, la creación del identificador oficial y la publicación en canales
ocurran en el orden correcto, con trazabilidad y sin depender de tags locales.

**Why this priority**: La desalineación entre versión interna, identificador de
release y canales de distribución puede romper despliegues y crear estados
ambiguos. Esta historia reduce ese riesgo operativo de forma directa.

**Independent Test**: Puede probarse solicitando una nueva versión desde el
flujo oficial y verificando que primero se alinean todas las referencias de
versión, luego se registra ese cambio de forma auditable, después se crea el
identificador oficial de la versión y solo entonces se ejecuta la publicación.

**Acceptance Scenarios**:

1. **Given** que el mantenedor inicia una nueva versión con un valor objetivo,
   **When** comienza el flujo oficial de release, **Then** el sistema actualiza
   primero la versión del producto y todas sus referencias visibles o
   dependientes antes de iniciar la publicación.
2. **Given** que la alineación de versión termina correctamente, **When** el
   flujo continúa, **Then** queda una evidencia auditable del cambio, se crea el
   identificador oficial de la versión y recién después avanza la etapa de
   release y despliegue.
3. **Given** que alguna referencia de versión no pudo alinearse o quedó
   inconsistente, **When** el flujo valida el estado previo a la publicación,
   **Then** el release se bloquea antes de desplegar en canales oficiales y
   expone la razón exacta de la inconsistencia.

### Edge Cases

- ¿Qué ocurre si el usuario entra a `Dominios bloqueados` y la lista actual está
  vacía?
- ¿Qué sucede si el usuario intenta agregar un dominio que ya está bloqueado o
  uno con formato inválido?
- ¿Cómo se comporta la vista si el usuario elimina el último dominio bloqueado?
- ¿Qué se muestra en `Actividad de bloqueo` cuando nunca se ha producido un
  bloqueo desde la activación?
- ¿Cómo se presenta `Top dominios bloqueados` cuando varios dominios comparten
  la misma frecuencia o cuando todavía no hay suficientes datos?
- ¿Qué sucede si el mantenedor solicita una versión que ya existe como
  identificador oficial o ya fue distribuida en uno o más canales?
- ¿Cómo responde el flujo oficial si logra alinear algunas referencias de
  versión pero no todas antes de intentar publicar?
- ¿Qué pasa si el cambio auditable de versión se registra, pero el identificador
  oficial no puede crearse o la publicación falla en uno de los canales?
- ¿Cómo se evita reportar éxito completo cuando la versión quedó desalineada
  entre el repositorio y los canales oficiales de distribución?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: El sistema MUST eliminar por completo la indicación `✓ Riesgo:
  Normal` del home de Sentinel CLI.
- **FR-002**: El sistema MUST conservar en el home la línea estética actual del
  producto y su claridad visual después de retirar esa indicación.
- **FR-003**: El sistema MUST incorporar una opción principal `Ajustes` en el
  home, visible dentro de la navegación principal existente.
- **FR-004**: El sistema MUST mantener el flujo interactivo principal como una
  experiencia guiada por menús, sin introducir rutas que rompan la navegación
  actual esperada por el usuario.
- **FR-005**: La opción `Ajustes` MUST incluir una entrada `Dominios bloqueados`
  accesible desde el mismo flujo guiado por menús.
- **FR-006**: La vista `Dominios bloqueados` MUST mostrar el listado vigente de
  dominios actualmente bloqueados de forma legible y coherente con la estética
  del producto.
- **FR-007**: La vista `Dominios bloqueados` MUST mostrar un estado vacío claro
  cuando no existan dominios bloqueados configurados.
- **FR-008**: Los usuarios MUST poder agregar un dominio bloqueado desde la
  vista `Dominios bloqueados`.
- **FR-009**: Los usuarios MUST poder editar un dominio bloqueado existente
  desde la misma sección.
- **FR-010**: Los usuarios MUST poder eliminar un dominio bloqueado existente
  desde la misma sección.
- **FR-011**: El sistema MUST actualizar el listado vigente inmediatamente
  después de una acción de alta, edición o eliminación confirmada.
- **FR-012**: El sistema MUST evitar que el usuario registre dominios duplicados
  o con formato inválido y MUST comunicar la causa de rechazo con un mensaje
  comprensible.
- **FR-013**: La gestión de `Dominios bloqueados` MUST conservar rutas claras
  para continuar administrando la lista o volver a niveles superiores del menú
  sin perder contexto.
- **FR-014**: La vista `Estado de Sentinel` MUST retirar de su tabla principal
  los campos `Riesgo`, `Resumen` y `Accion sugerida`.
- **FR-015**: La vista `Estado de Sentinel` MUST incorporar una sección separada
  llamada `Actividad de bloqueo`.
- **FR-016**: La sección `Actividad de bloqueo` MUST presentarse como tabla y
  MUST incluir únicamente estas métricas: `Bloqueos desde la activación`,
  `Dominios únicos bloqueados`, `Último bloqueo` y `Top dominios bloqueados`.
- **FR-017**: La sección `Actividad de bloqueo` MUST mostrar datos vigentes del
  estado de bloqueo o, cuando no existan, un estado sin datos explícito para
  cada métrica afectada.
- **FR-018**: `Top dominios bloqueados` MUST permitir al usuario identificar
  cuáles son los dominios con mayor recurrencia de bloqueo dentro del periodo
  visible actual.
- **FR-019**: `Último bloqueo` MUST permitir al usuario identificar el evento de
  bloqueo más reciente o confirmar claramente que aún no existe uno.
- **FR-020**: El sistema MUST mantener una experiencia visual y de navegación
  coherente entre home, `Ajustes`, `Dominios bloqueados` y `Estado de Sentinel`.
- **FR-021**: El flujo oficial de versionado y release MUST centralizar la
  creación de nuevas versiones en el proceso oficial del repositorio, iniciado
  mediante una versión objetivo explícita.
- **FR-022**: El flujo oficial MUST evitar que los mantenedores dependan de la
  creación o publicación manual de identificadores locales de versión para
  iniciar una release.
- **FR-023**: Antes de crear el identificador oficial de una nueva versión, el
  sistema MUST alinear la versión objetivo en el registro oficial del producto y
  en toda referencia del proyecto que la muestre o la utilice para distribución,
  visibilidad o validación.
- **FR-024**: El sistema MUST dejar una evidencia auditable y trazable de la
  actualización de versión realizada antes de la etapa de release.
- **FR-025**: El sistema MUST crear el identificador oficial de la nueva versión
  solo después de que la alineación previa de versiones haya finalizado sin
  inconsistencias.
- **FR-026**: El sistema MUST ejecutar la publicación y el despliegue en una
  etapa separada de la alineación y creación del identificador oficial.
- **FR-027**: El sistema MUST bloquear la publicación si detecta cualquier
  desalineación entre la versión objetivo, el estado versionado del proyecto, el
  identificador oficial creado o los canales oficiales de distribución.
- **FR-028**: El sistema MUST garantizar que una release completada deje la
  misma versión visible en el repositorio y en todos los canales oficiales
  soportados para esa versión.
- **FR-029**: El sistema MUST exponer un estado final trazable para cada intento
  de release, incluyendo como mínimo la versión solicitada, el resultado de la
  alineación previa, la confirmación del identificador oficial y el resultado de
  publicación por canal.
- **FR-030**: El sistema MUST fallar en modo seguro cuando no pueda completar la
  alineación de versiones, registrar la evidencia auditable, crear el
  identificador oficial o confirmar la consistencia final entre canales, y MUST
  evitar presentar ese intento como release exitosa.

### Key Entities *(include if feature involves data)*

- **Home Principal**: Representa la pantalla inicial de Sentinel CLI con sus
  opciones principales visibles y la estética base del producto.
- **Entrada de Ajustes**: Representa la opción principal que agrupa funciones de
  configuración y administración desde el mismo flujo guiado por menús.
- **Dominio Bloqueado**: Representa cada dominio que Sentinel mantiene dentro
  del listado activo de bloqueo y que el usuario puede revisar, agregar, editar
  o eliminar.
- **Listado de Dominios Bloqueados**: Representa la colección vigente de
  dominios sujetos a bloqueo visible para el usuario dentro de `Ajustes`.
- **Resumen de Actividad de Bloqueo**: Representa la información agregada que
  permite al usuario entender cuántos bloqueos ocurrieron, cuántos dominios
  fueron afectados, cuál fue el último evento y cuáles son los dominios más
  recurrentes.
- **Solicitud de Versión**: Representa la intención explícita del mantenedor de
  generar una nueva versión oficial del producto.
- **Registro Alineado de Versión**: Representa el estado del proyecto una vez
  que todas las referencias de versión requeridas quedaron sincronizadas con la
  versión objetivo.
- **Ejecución de Release**: Representa el intento trazable de crear el
  identificador oficial de versión y distribuirla por los canales oficiales.

## Assumptions & Scope Boundaries *(optional)*

- Se asume que `Ajustes` se incorpora como una opción principal adicional sin
  redefinir la arquitectura completa del menú principal existente.
- Se asume que la gestión de `Dominios bloqueados` cubre edición manual del
  listado vigente dentro de Sentinel CLI y no introduce en esta feature reglas
  automáticas nuevas para poblarlo.
- Se asume que la vista `Actividad de bloqueo` refleja la información de
  actividad disponible en el producto al momento de abrir `Estado de Sentinel`,
  sin ampliar en esta feature el alcance de la telemetría más allá de las cuatro
  métricas solicitadas.
- Quedan fuera de alcance cambios de branding mayores, nuevas secciones de
  configuración no pedidas y nuevos canales de distribución adicionales a los ya
  oficiales para Sentinel CLI.
- Se asume que la automatización centralizada de versiones y release debe ser la
  única ruta operativa aprobada para publicar nuevas versiones dentro del alcance
  de esta feature.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: En validación de aceptación, el 100% de los recorridos del home
  muestran la navegación principal sin la indicación `✓ Riesgo: Normal` y con la
  nueva opción `Ajustes` visible.
- **SC-002**: Al menos el 90% de los usuarios evaluados puede completar sin
  ayuda las tareas de revisar, agregar, editar y eliminar dominios bloqueados en
  una sola sesión de menú guiado.
- **SC-003**: Al menos el 95% de las revisiones de `Estado de Sentinel`
  identifica correctamente las cuatro métricas de `Actividad de bloqueo` y
  confirma la ausencia de `Riesgo`, `Resumen` y `Accion sugerida` en la tabla
  principal.
- **SC-004**: El 100% de las releases exitosas generadas con esta feature deja
  la misma versión confirmada en el estado versionado del proyecto, en el
  identificador oficial de la release y en todos los canales oficiales de
  distribución soportados.
- **SC-005**: El 100% de los intentos de release con desalineación de versión o
  fallo previo a publicación termina bloqueado o marcado como no exitoso, con
  una razón visible y trazable para el mantenedor.
