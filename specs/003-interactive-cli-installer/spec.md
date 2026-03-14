# Feature Specification: Interactive Sentinel CLI

**Feature Branch**: `003-interactive-cli-installer`  
**Created**: 2026-03-14  
**Status**: Draft  
**Input**: User description: "necesito que ahora esta herramineta sea un CLI totalmente interativo la idea es que no reciba flgas ni nada por el estilo, que lo test, corran para priorizar la seguridad de este software para que no dañe la integridad de la conexion de red, necesito que este CLI tenga una esttica muy limpia y elaborada, asi como que por el momento se tenga nada mas las funcionalidades principales de manera precisa, la idea es que se cuente ya con un bloqueo grande de dominios de anuncios y una metodologia pulcra y porbada para que este bloqueo sea eficinete la idea es que contemos con un .sh que nos permita instalar a sentinel en el path y que sea solo llamarlo y usar, ademas que el sh se encargue de actilizar cuando haya una version nueva, como re intalar todo de menra precisa"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Activar protección desde una experiencia guiada (Priority: P1)

Como usuario local, quiero abrir Sentinel y seguir una experiencia interactiva
sin flags ni comandos complejos para activar el bloqueo de anuncios con una
interfaz clara, limpia y segura.

**Why this priority**: Es el cambio más importante del feature. Si el producto
no puede usarse como un flujo guiado y seguro desde terminal, no cumple la
nueva dirección del CLI.

**Independent Test**: Puede probarse iniciando Sentinel desde terminal,
siguiendo el flujo interactivo principal y verificando que el usuario puede
activar la protección, entender el resultado y salir sin perder conectividad.

**Acceptance Scenarios**:

1. **Given** que Sentinel está instalado y la protección está inactiva,
   **When** el usuario abre la herramienta desde terminal, **Then** ve una
   interfaz interactiva limpia que le presenta solo las acciones principales del
   producto.
2. **Given** que el usuario elige activar la protección, **When** la
   herramienta ejecuta sus validaciones previas, **Then** solo continúa si puede
   garantizar una activación segura o explica por qué se detuvo.

---

### User Story 2 - Instalar, actualizar o reinstalar Sentinel con un solo script (Priority: P2)

Como usuario, quiero disponer de un script shell único para instalar Sentinel
en el `PATH`, actualizarlo cuando haya una versión nueva y reinstalarlo si hace
falta, sin pasos manuales ambiguos.

**Why this priority**: La usabilidad real del producto depende de una
distribución limpia. Si instalar o mantener Sentinel requiere pasos manuales
frágiles, la experiencia se degrada antes de usar el bloqueo.

**Independent Test**: Puede probarse ejecutando el script de instalación en un
equipo sin Sentinel, luego en un equipo con Sentinel ya presente y finalmente en
un escenario de reinstalación, verificando que la herramienta queda utilizable
desde el `PATH`.

**Acceptance Scenarios**:

1. **Given** un equipo sin Sentinel instalado, **When** el usuario ejecuta el
   script oficial, **Then** Sentinel queda disponible desde el `PATH` y listo
   para abrirse directamente.
2. **Given** un equipo con Sentinel ya instalado, **When** el usuario ejecuta
   el mismo script, **Then** la herramienta decide correctamente si actualizar o
   reinstalar y comunica el resultado de forma clara.

---

### User Story 3 - Confiar en el estado y la seguridad del bloqueo (Priority: P3)

Como usuario, quiero que Sentinel verifique su estado, ejecute comprobaciones de
seguridad y me permita recuperar la conectividad desde la misma experiencia
interactiva para no comprometer la integridad de mi red.

**Why this priority**: Un bloqueador a nivel sistema sin comprobaciones
evidentes y recuperación guiada no es confiable, aunque tenga buena estética o
un bloqueo amplio.

**Independent Test**: Puede probarse activando la protección, revisando el
estado interactivo, forzando una condición recuperable y comprobando que
Sentinel guía al usuario para restaurar una red válida.

**Acceptance Scenarios**:

1. **Given** que Sentinel está activo o degradado, **When** el usuario abre la
   vista de estado o salud, **Then** la herramienta muestra el estado actual, el
   nivel de riesgo y la siguiente acción recomendada.
2. **Given** que una comprobación de seguridad detecta riesgo de afectar la red,
   **When** el usuario intenta continuar, **Then** Sentinel prioriza la
   protección de la conectividad y ofrece un flujo claro de recuperación.

### Edge Cases

- ¿Qué ocurre si el usuario abre Sentinel mientras la protección ya está activa?
- ¿Qué pasa si el sistema ya tiene una configuración de red especial antes de
  activar el bloqueo?
- ¿Cómo responde Sentinel si una instalación o actualización se interrumpe a la
  mitad?
- ¿Qué sucede si la versión instalada está dañada o incompleta y el usuario
  ejecuta el script de reinstalación?
- ¿Cómo se comporta la interfaz si una comprobación de seguridad falla pero la
  red todavía sigue operativa?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: El sistema MUST ofrecer una experiencia interactiva de terminal
  como interfaz principal del producto, sin requerir flags para el flujo normal
  de uso.
- **FR-002**: El sistema MUST mostrar únicamente las funcionalidades principales
  del producto dentro de la experiencia interactiva inicial.
- **FR-003**: El sistema MUST ejecutar comprobaciones de seguridad antes de
  activar cambios que puedan afectar la conectividad del equipo.
- **FR-004**: El sistema MUST detener o revertir una operación si no puede
  preservar la integridad de la configuración de red.
- **FR-005**: El sistema MUST permitir activar y desactivar la protección desde
  la experiencia interactiva sin obligar al usuario a recordar comandos
  avanzados.
- **FR-006**: El sistema MUST mostrar estado, salud operativa y acciones
  recomendadas con una presentación de terminal limpia y elaborada.
- **FR-007**: El sistema MUST incluir un conjunto amplio de dominios de anuncios
  listos para bloquear desde la primera ejecución soportada.
- **FR-008**: El sistema MUST aplicar una metodología de bloqueo precisa que
  reduzca el impacto sobre destinos legítimos dentro del alcance del producto.
- **FR-009**: El sistema MUST permitir ejecutar comprobaciones o pruebas de
  seguridad antes de confirmar cambios sensibles.
- **FR-010**: El sistema MUST ofrecer una ruta interactiva de recuperación para
  restaurar un estado de red válido cuando ocurra un fallo o interrupción.
- **FR-011**: El sistema MUST incluir un script shell oficial para instalar
  Sentinel en el `PATH` con una sola ejecución.
- **FR-012**: El sistema MUST permitir que el mismo script shell detecte una
  instalación existente y resuelva si corresponde actualizar o reinstalar.
- **FR-013**: El sistema MUST comunicar con claridad el resultado de
  instalación, actualización o reinstalación y los siguientes pasos si algo
  falla.
- **FR-014**: El sistema MUST mantener el alcance del producto limitado a
  bloqueo, estado, seguridad, recuperación e instalación/mantenimiento esencial
  en esta fase.

### Key Entities *(include if feature involves data)*

- **Interactive Session**: Representa una sesión de uso en terminal con estado
  visible, acción elegida y resultado mostrado al usuario.
- **Safety Check Result**: Representa la salida de una comprobación previa sobre
  conectividad, riesgo operativo y posibilidad de recuperación.
- **Blocklist Bundle**: Representa el conjunto de dominios de anuncios que la
  herramienta usa para bloquear de forma predeterminada.
- **Installation State**: Representa la situación actual de Sentinel en el
  equipo, incluyendo presencia en `PATH`, versión instalada y necesidad de
  actualización o reinstalación.
- **Network Recovery Snapshot**: Representa el estado de red guardado para
  restaurar conectividad si una operación falla.

## Assumptions & Scope Boundaries *(optional)*

- El usuario principal es un operador local que quiere usar Sentinel de manera
  guiada desde terminal, no una API o una interfaz gráfica.
- La fase actual elimina la dependencia de flags para uso normal; la interacción
  avanzada y la automatización externa quedan fuera de alcance por ahora.
- La primera entrega prioriza una experiencia limpia con las funciones centrales:
  instalar, abrir, activar, revisar estado, recuperar y mantener el bloqueo.
- El script shell oficial será la vía principal de instalación, actualización y
  reinstalación en esta etapa.
- La cobertura de dominios de anuncios debe ser amplia desde el inicio, pero la
  personalización extensa de listas queda fuera de alcance en esta fase.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Al menos el 90% de los usuarios de prueba pueden instalar
  Sentinel y abrirlo desde el `PATH` en menos de 5 minutos sin ayuda externa.
- **SC-002**: Al menos el 95% de los usuarios de prueba pueden activar o
  desactivar la protección usando solo la experiencia interactiva principal en
  su primer intento.
- **SC-003**: El 100% de las operaciones detenidas por seguridad dejan al equipo
  en un estado de red recuperable mediante la propia herramienta.
- **SC-004**: Al menos el 90% de los usuarios de prueba identifican con
  claridad el estado actual del sistema y la siguiente acción recomendada desde
  la interfaz interactiva.
- **SC-005**: Al menos el 90% de las ejecuciones de instalación sobre equipos ya
  preparados resuelven correctamente si deben actualizar o reinstalar sin pasos
  manuales adicionales.
