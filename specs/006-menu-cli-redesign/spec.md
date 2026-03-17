# Feature Specification: Rediseño de CLI por Vistas

**Feature Branch**: `006-menu-cli-redesign`  
**Created**: 2026-03-16  
**Status**: Draft  
**Input**: User description: "necesito una reestructuracion en la Ui y Ux de sentinel, puesto que actualmente tiene una manera extraña de funcionar, la idea es que tenga el funcionamiento comun de una herramienta CLI, es decir se divida por vistas y haya un porceso de que cada seleccionable del home lleve a una vista diferente o muestre su estado de manera independiente no de manera reactiva como esta actualmente, selecciono una accion y esta me lleva a otra vista limpiando la pantalla para que no quede en la parte superior el texto de la vista anterior, la diea es que el home sea lo mas limpio posible, en el home debes de agregar este logo ASCII /Users/santiagoacosta/Desktop/sentinel-cli-logo.txt, la idea tambien es agregar ANSI styling + Unicode symbols + spinners, para la estetica del CLI, coherente al flujo actual, en esta especificacion debe de primar la estica sin dañar la funcionalidad, ahciendo un flijo coherente a como lo es UN CLI, recurda que el flijo debe de ser menu-driven CLI que es basicamente un flijo sin ser reactivo NO TUI, por otra parte se debe de eliminar el codigo de TUI por que este software no es una TUI sino un CLI"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Entrar a un home limpio y orientado a acciones (Priority: P1)

Como usuario de Sentinel, quiero abrir la herramienta y encontrar un home limpio,
claro y reconocible para entender de inmediato qué puedo hacer y entrar a la
acción correcta sin ruido visual ni texto heredado.

**Why this priority**: El home es el punto de entrada de toda la experiencia.
Si no comunica claramente las acciones principales ni limpia la presentación, el
resto del rediseño pierde valor.

**Independent Test**: Puede probarse iniciando Sentinel desde terminal y
verificando que el home muestra el logo ASCII aprobado, un conjunto breve de
opciones principales y una navegación inicial comprensible sin necesidad de
memorizar comandos.

**Acceptance Scenarios**:

1. **Given** que el usuario abre Sentinel en una terminal interactiva,
   **When** aparece la pantalla inicial, **Then** ve un home limpio con el logo
   ASCII aprobado, una lista breve de acciones principales y ayuda mínima para
   navegar.
2. **Given** que el usuario vuelve al home desde otra vista, **When** se
   completa la transición, **Then** el home aparece limpio, sin residuos
   visibles de la vista anterior.

---

### User Story 2 - Navegar por vistas independientes como en un CLI tradicional (Priority: P1)

Como usuario, quiero que cada opción seleccionable del home me lleve a una vista
dedicada o a un estado independiente para comprender el contexto actual sin que
la interfaz reaccione dentro de la misma pantalla de forma confusa.

**Why this priority**: El problema central descrito por el usuario es el
comportamiento reactivo y acumulativo. Cambiar a vistas independientes es la
base del nuevo flujo.

**Independent Test**: Puede probarse recorriendo cada opción principal del home
y validando que cada selección limpia la terminal, abre una vista propia y
permite volver sin mezclar contenido entre pantallas.

**Acceptance Scenarios**:

1. **Given** que el usuario está en el home, **When** selecciona una acción
   principal, **Then** Sentinel limpia la pantalla activa y presenta una vista
   dedicada para esa acción con solo la información relevante a ese contexto.
2. **Given** que el usuario completa una acción o revisa un estado,
   **When** decide regresar, **Then** puede volver al home o a la vista previa
   mediante una ruta de navegación explícita y predecible.
3. **Given** que el usuario cambia varias veces entre acciones,
   **When** avanza y retrocede entre vistas, **Then** ninguna pantalla conserva
   texto residual que dificulte identificar dónde se encuentra.

---

### User Story 3 - Ejecutar acciones con una estética CLI coherente sin perder funcionalidad (Priority: P2)

Como usuario, quiero una experiencia visual más cuidada en terminal, con señales
de estado, símbolos y progreso acordes con un CLI moderno, para sentir que la
herramienta es más clara y confiable sin sacrificar las funciones actuales.

**Why this priority**: La estética no reemplaza el flujo, pero sí mejora la
comprensión y la confianza durante operaciones sensibles. Debe elevar la
experiencia sin introducir regresiones funcionales.

**Independent Test**: Puede probarse ejecutando las acciones principales y
verificando que muestran jerarquía visual consistente, indicadores de progreso
cuando corresponda y mensajes finales claros sin alterar los resultados
funcionales existentes.

**Acceptance Scenarios**:

1. **Given** que el usuario inicia una operación que tarda perceptiblemente,
   **When** la acción está en curso, **Then** Sentinel muestra un indicador de
   progreso visible hasta que exista un resultado final.
2. **Given** que una operación termina con éxito, advertencia o error,
   **When** la vista correspondiente presenta el resultado, **Then** el usuario
   puede distinguir el estado mediante señales visuales claras y un mensaje
   final entendible.
3. **Given** que el rediseño visual está activo, **When** el usuario recorre los
   flujos principales, **Then** sigue pudiendo completar las mismas tareas
   esenciales que antes del rediseño.

### Edge Cases

- ¿Qué ocurre si la terminal no tiene suficiente alto o ancho para mostrar el
  logo ASCII completo y las opciones principales con legibilidad?
- ¿Cómo responde la herramienta si el terminal no soporta color, símbolos
  extendidos o animaciones visibles?
- ¿Qué sucede si el usuario intenta cambiar de vista mientras una operación
  todavía está en progreso?
- ¿Qué pasa si una acción termina con error después de limpiar la pantalla y el
  usuario necesita volver al home sin perder el mensaje final?
- ¿Cómo se comporta el flujo cuando el usuario abre una vista que no tiene datos
  disponibles todavía o cuyo estado es desconocido?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: El sistema MUST presentar la experiencia interactiva principal de
  Sentinel como una CLI guiada por menús y vistas, no como una interfaz de
  actualización continua dentro de una misma pantalla.
- **FR-002**: El sistema MUST iniciar en un home limpio que priorice acciones
  principales y reduzca al mínimo el texto introductorio no esencial.
- **FR-003**: El sistema MUST mostrar en el home el logo ASCII de Sentinel
  provisto por el usuario como activo visual aprobado para esta experiencia.
- **FR-004**: El sistema MUST hacer que cada opción seleccionable del home lleve
  a una vista dedicada o a una salida de estado independiente, en lugar de
  actualizar el contenido principal de manera reactiva en el mismo contexto
  visual.
- **FR-005**: El sistema MUST limpiar la pantalla al entrar a una nueva vista,
  al regresar al home y al mostrar un resultado final, de forma que la vista
  activa sea identificable sin ruido residual.
- **FR-006**: El sistema MUST ofrecer una navegación explícita entre home,
  vistas de acción, vistas de estado, confirmaciones y salida final.
- **FR-007**: El sistema MUST permitir que el usuario regrese desde una vista
  secundaria a un punto de navegación claro sin reiniciar manualmente la
  herramienta.
- **FR-008**: El sistema MUST mantener disponibles en la nueva experiencia las
  funciones esenciales actuales de Sentinel, incluyendo acciones de operación,
  consulta de estado y recuperación ya ofrecidas por el flujo interactivo.
- **FR-009**: El sistema MUST mostrar el estado o resultado de cada acción en su
  propia vista, separado del home y de otras acciones no relacionadas.
- **FR-010**: El sistema MUST presentar confirmaciones y advertencias en vistas
  específicas cuando una acción requiera validación del usuario antes de
  continuar.
- **FR-011**: El sistema MUST usar recursos visuales compatibles con terminal,
  incluyendo énfasis de color, símbolos y señales de estado, para diferenciar
  acciones, advertencias, éxitos y errores sin depender exclusivamente de texto
  plano.
- **FR-012**: El sistema MUST mostrar un indicador visible de progreso mientras
  una acción de usuario siga en ejecución y todavía no exista un resultado
  definitivo.
- **FR-013**: El sistema MUST presentar un resumen final claro al terminar cada
  operación, incluyendo resultado, siguiente acción recomendada y forma de
  regresar o salir.
- **FR-014**: El sistema MUST conservar una estética coherente entre home,
  vistas de acción y vistas de resultado, de modo que la herramienta se perciba
  como un CLI uniforme y no como una mezcla de estilos o modos interactivos.
- **FR-015**: El sistema MUST degradar su presentación visual de manera legible
  cuando el terminal no pueda representar color, símbolos extendidos,
  animaciones o el logo completo.
- **FR-016**: El sistema MUST mantener sin regresiones los resultados
  funcionales esperados de los flujos principales existentes después del
  rediseño visual y de navegación.
- **FR-017**: El sistema MUST retirar la experiencia interactiva de pantalla
  completa anterior para que el producto exponga un único flujo interactivo
  coherente con un CLI por menús y vistas.
- **FR-018**: El sistema MUST diferenciar claramente el home de las vistas
  secundarias para que el usuario identifique en todo momento si está
  seleccionando una acción, revisando estado, confirmando una operación o viendo
  un resultado final.

### Key Entities *(include if feature involves data)*

- **Home View**: Representa la pantalla inicial limpia que agrupa branding,
  opciones principales y orientación mínima de navegación.
- **Navigation View**: Representa cualquier vista secundaria dedicada a una
  acción, estado, confirmación o resultado dentro del flujo de Sentinel.
- **Menu Action**: Representa una opción seleccionable desde el home que abre
  una vista independiente para ejecutar o consultar una capacidad concreta.
- **Operation Result**: Representa el resultado final mostrado al usuario tras
  ejecutar una acción, incluyendo éxito, advertencia, error y siguiente paso.
- **Visual Cue Set**: Representa el conjunto de recursos visuales del terminal
  usados para comunicar jerarquía, estado, progreso y énfasis de forma
  consistente.

## Assumptions & Scope Boundaries *(optional)*

- El rediseño cubre la experiencia interactiva principal en terminal; los modos
  no interactivos, automatizados o internos permanecen fuera del alcance salvo
  cuando necesiten reflejar mensajes compatibles con el nuevo flujo.
- Se asume que el logo ASCII ubicado en
  `/Users/santiagoacosta/Desktop/sentinel-cli-logo.txt` es el activo aprobado
  para el home y que Sentinel puede incorporar una versión equivalente dentro
  del producto para no depender de esa ruta en ejecución.
- Se asume que las acciones principales actuales del flujo interactivo siguen
  siendo las capacidades prioritarias que deben mantenerse disponibles tras el
  rediseño.
- La mejora visual debe reforzar comprensión y confianza, no introducir nuevas
  tareas, permisos ni cambios de lógica de negocio.
- La retirada de la experiencia anterior de pantalla completa aplica a la
  interacción visible para el usuario final; el alcance de esta especificación
  no incluye rediseñar procesos internos que no formen parte del flujo
  interactivo.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Al menos el 90% de los usuarios de prueba identifican la acción
  correcta desde el home en menos de 15 segundos tras abrir Sentinel.
- **SC-002**: El 100% de las opciones principales validadas desde el home abren
  una vista independiente o un resultado dedicado, sin actualizarse en línea
  dentro de la misma pantalla inicial.
- **SC-003**: El 100% de los flujos principales validados dejan la terminal sin
  texto residual de vistas previas al entrar a una nueva vista o volver al
  home.
- **SC-004**: Al menos el 95% de los usuarios de prueba completan una tarea
  principal desde el home hasta su resultado final en no más de 3 pasos de
  navegación.
- **SC-005**: El 100% de las operaciones que tarden más de 1 segundo muestran
  una señal visible de que siguen en progreso antes de presentar el resultado.
- **SC-006**: La validación de aceptación de los flujos principales no registra
  regresiones funcionales críticas frente al comportamiento esperado previo al
  rediseño.
- **SC-007**: Al menos el 85% de los usuarios de prueba califican la experiencia
  interactiva como más clara y coherente que la versión anterior.
