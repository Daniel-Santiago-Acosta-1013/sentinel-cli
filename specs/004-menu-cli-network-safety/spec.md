# Feature Specification: CLI Guiada por Menús Segura

**Feature Branch**: `004-menu-cli-network-safety`  
**Created**: 2026-03-16  
**Status**: Draft  
**Input**: User description: "neceisto cambiar el tipo de CLI por que la actual parece una TUI necesito una especificacion que mejore esto ademas, necesito que sea Menu-driven, usable por las fechas del teclado de manera precisa, debe de mantener la configuracion de red original del equipo del usuario puesto que al desactivar la herramineta esta me deja defectos en la conexion al internet no funciona la red y estoy obligado a apagar el equipo y volverlo a encernder para que fucnione la red"

## Clarifications

### Session 2026-03-16

- Q: ¿Qué nivel visual debe tener la experiencia CLI? → A: Debe ser lo más limpia y estética posible.
- Q: ¿Cómo deben mostrarse los estados de la herramienta? → A: Mediante tablas claras y legibles.
- Q: ¿En qué idioma debe presentarse la herramienta? → A: Toda la experiencia debe estar en español.
- Q: ¿Qué debe contemplar la validación para proteger el equipo del usuario? → A: Tests que verifiquen la integridad de la configuración original de red en activación, desactivación y recuperación.
- Q: ¿Cómo debe comportarse la terminal al navegar entre pantallas? → A: La pantalla debe limpiarse o refrescarse para evitar acumulación de texto.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Desactivar sin perder conectividad (Priority: P1)

Como usuario local, quiero activar o desactivar la protección desde un menú guiado
sin comprometer la configuración original de red del equipo para no quedarme sin
internet al salir de la herramienta.

**Why this priority**: El problema actual deja al usuario sin red y obliga a
reiniciar el equipo. La preservación y restauración confiable de la red es el
valor más crítico del cambio.

**Independent Test**: Puede probarse activando la protección en un equipo con
conectividad funcional, desactivándola después y verificando que el equipo
recupera su configuración original y sigue navegando sin reinicio.

**Acceptance Scenarios**:

1. **Given** que el equipo tiene una configuración de red funcional antes de
   activar la herramienta, **When** el usuario activa la protección y luego la
   desactiva desde el menú, **Then** la configuración original de red se
   restaura y la conectividad sigue disponible sin reiniciar el equipo.
2. **Given** que ocurre un fallo durante la desactivación, **When** la
   herramienta detecta que la red no quedó en un estado válido, **Then** inicia
   un flujo guiado de restauración antes de dar por terminada la operación.

---

### User Story 2 - Navegar con precisión usando teclado (Priority: P2)

Como usuario, quiero una CLI guiada por menús que pueda operarse con precisión
usando solo el teclado, en español y con una presentación limpia para no
depender de una interfaz tipo TUI confusa ni de comandos memorizados.

**Why this priority**: El cambio de experiencia es parte central del pedido. Si
la herramienta no puede usarse con navegación clara por teclado, el rediseño no
cumple su objetivo principal.

**Independent Test**: Puede probarse abriendo la herramienta desde terminal,
recorriendo el menú principal y sus opciones con teclado, y completando una
tarea principal sin escribir flags.

**Acceptance Scenarios**:

1. **Given** que el usuario abre la herramienta, **When** aparece el menú
   principal, **Then** puede recorrer las opciones visibles con el teclado,
   identificar cuál está seleccionada, leer las acciones en español y confirmar
   su elección sin usar flags.
2. **Given** que el usuario presiona una tecla no válida o intenta moverse más
   allá de los límites del menú, **When** la herramienta recibe esa entrada,
   **Then** mantiene un estado predecible, conserva el contexto actual y muestra
   una guía breve de navegación.
3. **Given** que el usuario abre una vista de estado o cambia entre pantallas,
   **When** la herramienta actualiza la interfaz, **Then** limpia o refresca la
   terminal y muestra el estado de protección y red en una tabla legible sin
   texto acumulado de vistas anteriores.

---

### User Story 3 - Recuperarse de estados incompletos o configuraciones previas (Priority: P3)

Como usuario, quiero que la herramienta detecte configuraciones de red previas e
interrupciones de sesiones anteriores para recuperar un estado estable antes de
hacer nuevos cambios.

**Why this priority**: Muchos defectos de red ocurren cuando una herramienta
sobrescribe configuraciones existentes o deja cambios a medio camino. La
recuperación guiada reduce ese riesgo.

**Independent Test**: Puede probarse iniciando la protección en un equipo con
configuración de red personalizada, interrumpiendo la herramienta o alterando el
estado entre sesiones y verificando que el siguiente arranque ofrece una ruta de
restauración clara.

**Acceptance Scenarios**:

1. **Given** que el equipo ya usa una configuración de red personalizada antes
   de activar la herramienta, **When** el usuario inicia la protección,
   **Then** la herramienta identifica esa configuración, la registra como estado
   original y explica qué preservará antes de continuar.
2. **Given** que una sesión anterior terminó de forma incompleta, **When** el
   usuario vuelve a abrir la herramienta, **Then** recibe primero una opción
   clara para restaurar la red o retomar el estado seguro antes de ejecutar
   nuevos cambios.

### Edge Cases

- ¿Qué ocurre si el usuario intenta desactivar la protección cuando no existe un
  estado original de red completo para restaurar?
- ¿Cómo responde la herramienta si la configuración de red del sistema cambia
  fuera de Sentinel mientras la protección está activa?
- ¿Qué pasa si el usuario cancela la restauración a mitad del proceso?
- ¿Cómo se comporta la navegación si el usuario mantiene presionadas teclas de
  desplazamiento o repite entradas rápidamente?
- ¿Qué sucede si la conectividad ya estaba degradada antes de abrir la
  herramienta?
- ¿Qué sucede si el tamaño de la terminal no permite mostrar una tabla completa
  de estado sin perder legibilidad?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: El sistema MUST ofrecer su experiencia principal como una CLI
  guiada por menús en terminal y no exigir flags para las tareas esenciales de
  uso diario.
- **FR-002**: El sistema MUST permitir que las tareas principales se completen
  usando solo el teclado.
- **FR-003**: El sistema MUST mostrar de forma visible qué opción del menú está
  activa y permitir una navegación predecible entre opciones, pantallas y
  acciones de regreso.
- **FR-004**: El sistema MUST incluir al menos las acciones de activar
  protección, desactivar protección, revisar estado actual y ejecutar
  recuperación desde la experiencia guiada.
- **FR-005**: El sistema MUST capturar la configuración de red activa del equipo
  antes de aplicar cualquier cambio que pueda afectar la conectividad.
- **FR-006**: El sistema MUST preservar la configuración de red original del
  usuario como referencia de restauración mientras la protección esté activa.
- **FR-007**: El sistema MUST restaurar la configuración de red original al
  desactivar la herramienta, incluso si el usuario sale desde un flujo guiado y
  no desde comandos manuales.
- **FR-008**: El sistema MUST verificar al finalizar una desactivación o
  restauración que la red quedó en un estado válido para el usuario y comunicar
  el resultado antes de cerrar el flujo.
- **FR-009**: El sistema MUST ofrecer un flujo guiado de recuperación cuando una
  activación, desactivación o restauración no termine correctamente.
- **FR-010**: El sistema MUST detectar en el siguiente arranque si existe una
  operación previa incompleta y priorizar la recuperación antes de permitir
  nuevos cambios de red.
- **FR-011**: El sistema MUST advertir al usuario cuando detecte configuraciones
  previas de red que puedan verse afectadas y explicar qué información se
  conservará y qué se modificará temporalmente.
- **FR-012**: El sistema MUST evitar que el reinicio del equipo sea la ruta
  normal necesaria para recuperar la conectividad después de desactivar la
  herramienta o de resolver un fallo.
- **FR-013**: El sistema MUST comunicar con mensajes claros el estado actual de
  protección, el estado de la red y la siguiente acción recomendada en español
  en cada paso crítico.
- **FR-014**: El sistema MUST presentar el resumen de estado, salud y
  recuperación en tablas legibles cuando el usuario necesite comparar valores o
  identificar rápidamente la situación del sistema.
- **FR-015**: El sistema MUST mantener una presentación visual limpia y
  consistente entre pantallas para evitar ruido visual durante la operación.
- **FR-016**: El sistema MUST limpiar o refrescar la terminal al cambiar de
  menú, vista o resultado final para evitar acumulación de texto que dificulte
  reconocer la pantalla activa.
- **FR-017**: El sistema MUST mantener todos los menús, etiquetas, mensajes de
  estado, advertencias, confirmaciones y ayudas en español.
- **FR-018**: El sistema MUST incluir pruebas que validen la integridad de la
  configuración original de red del equipo durante activación, desactivación,
  recuperación e interrupciones.
- **FR-019**: El sistema MUST mantener el alcance de esta mejora centrado en la
  experiencia guiada por menú, la operación precisa con teclado, la preservación
  de red y la recuperación segura; la automatización avanzada por scripts queda
  fuera de alcance en esta fase.

### Key Entities *(include if feature involves data)*

- **Menu Session**: Representa una sesión de uso de la CLI guiada, incluyendo la
  opción seleccionada, la pantalla actual y el resultado visible para el
  usuario.
- **Network Configuration Snapshot**: Representa el estado original de red del
  equipo antes de que Sentinel haga cambios, usado como base para restauración.
- **Protection State**: Representa si la protección está inactiva, activa, en
  transición o en recuperación.
- **Status Summary**: Representa la información consolidada de estado y salud
  que la herramienta muestra al usuario en tablas dentro de la CLI.
- **Recovery Outcome**: Representa el resultado de una restauración o reparación
  de red, incluyendo estado final, advertencias y siguiente acción sugerida.

## Assumptions & Scope Boundaries *(optional)*

- Se interpreta "fechas del teclado" como "flechas del teclado"; por defecto, la
  navegación precisa se entiende como operación confiable mediante flechas,
  confirmación y regreso desde teclado.
- El usuario objetivo es una persona operando Sentinel localmente en su equipo,
  no un proceso automatizado ni una integración externa.
- La configuración original de red que debe preservarse es la que el equipo
  tenga activa inmediatamente antes de que Sentinel aplique cambios.
- Si el sistema detecta cambios externos a la red mientras Sentinel está activo,
  la herramienta debe priorizar informar y recuperar un estado estable, no
  sobrescribir silenciosamente esos cambios.
- La localización requerida en esta fase es español completo para toda la
  experiencia visible al usuario; otros idiomas quedan fuera de alcance.
- La experiencia visual limpia se entiende como pantallas despejadas, textos
  vigentes por contexto y ausencia de historial residual durante la navegación.
- La personalización extensa de menús, atajos o perfiles de red queda fuera de
  alcance en esta fase.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Al menos el 95% de los usuarios de prueba completan una activación
  o desactivación desde el menú principal en menos de 2 minutos usando solo el
  teclado.
- **SC-002**: El 100% de las desactivaciones exitosas restauran la configuración
  original de red sin requerir reinicio del equipo en la suite de aceptación
  definida para el producto.
- **SC-003**: El 100% de las operaciones interrumpidas o fallidas muestran una
  ruta de recuperación guiada en el siguiente arranque antes de permitir nuevos
  cambios de red.
- **SC-004**: Al menos el 90% de los usuarios de prueba identifican el estado
  actual de protección y la siguiente acción recomendada en menos de 30
  segundos después de abrir la herramienta mediante tablas y textos en español.
- **SC-005**: La validación de aceptación no registra defectos críticos en los
  que desactivar la herramienta deje al usuario sin conectividad funcional.
- **SC-006**: El 100% de los menús, estados, advertencias y confirmaciones
  visibles en la suite de aceptación se presentan en español.
- **SC-007**: El 100% de los casos de prueba definidos para integridad de red
  pasan en los escenarios de activación, desactivación, recuperación e
  interrupción.
- **SC-008**: Ninguno de los flujos principales validados deja texto residual de
  pantallas anteriores que impida identificar la vista activa.
