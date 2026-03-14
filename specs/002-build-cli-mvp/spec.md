# Feature Specification: Sentinel CLI MVP

**Feature Branch**: `002-build-cli-mvp`  
**Created**: 2026-03-14  
**Status**: Draft  
**Input**: User description: "listo necesito contruir la CLI y las funciones principales ya funcionales para el bloqueo de anincios de menra precisa con un diseño estetitico y con las claridad del constitution, para tener una primera version del aplicativo en rust, de manera precisa"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Activar protección con claridad y seguridad (Priority: P1)

Como usuario local, quiero activar el bloqueo de anuncios con un comando claro y
una salida legible para proteger el tráfico del equipo sin perder conectividad.

**Why this priority**: Es el valor principal del producto. Si el usuario no
puede activar la protección con seguridad y entender el resultado de inmediato,
no existe un MVP útil.

**Independent Test**: Puede probarse activando la protección en un equipo con
conexión funcional y verificando que el sistema queda protegido, que la salida
del comando comunica el estado final y que la conectividad se mantiene.

**Acceptance Scenarios**:

1. **Given** un equipo con conectividad funcional y la protección inactiva,
   **When** el usuario ejecuta el comando principal para activar el bloqueo,
   **Then** la protección queda activa y la salida muestra estado, resultado y
   siguiente paso recomendado.
2. **Given** que la activación detecta una condición insegura o una falla en la
   preparación del entorno, **When** el sistema no puede garantizar una
   recuperación segura, **Then** el cambio se cancela o revierte y la salida
   explica el motivo y la acción de recuperación.

---

### User Story 2 - Ajustar precisión sin saturar la herramienta (Priority: P2)

Como usuario, quiero gestionar exclusiones puntuales y reglas esenciales para
reducir falsos positivos sin tener que desactivar toda la protección.

**Why this priority**: La precisión define la utilidad real del bloqueador. El
MVP necesita control fino sobre casos legítimos, pero sin crecer a una consola
de administración compleja.

**Independent Test**: Puede probarse creando una exclusión para un destino
legítimo, confirmando que la protección sigue activa para el resto del tráfico y
que la salida del comando deja claro qué cambio se aplicó.

**Acceptance Scenarios**:

1. **Given** que la protección está activa y un destino legítimo está siendo
   afectado, **When** el usuario agrega una exclusión soportada por la CLI,
   **Then** el sistema conserva la protección general y deja de bloquear ese
   destino específico.
2. **Given** que el usuario consulta las reglas esenciales configuradas,
   **When** la herramienta muestra el resultado, **Then** la información aparece
   resumida, ordenada y enfocada solo en reglas accionables del MVP.

---

### User Story 3 - Ver estado y recuperar confianza operativa (Priority: P3)

Como usuario, quiero consultar el estado actual, revisar actividad reciente y
recuperar la red cuando algo salga mal para operar la herramienta con confianza.

**Why this priority**: Un CLI que altera el tráfico del sistema necesita
observabilidad y recuperación explícita. Sin eso, la experiencia se siente
opaca y riesgosa.

**Independent Test**: Puede probarse activando la protección, consultando el
estado, provocando una condición recuperable y ejecutando el flujo de
restauración para verificar que la red vuelve a un estado válido.

**Acceptance Scenarios**:

1. **Given** que la protección está activa, **When** el usuario consulta el
   estado, **Then** la herramienta muestra si está activa, degradada o
   desactivada, junto con un resumen corto de salud operativa.
2. **Given** que ocurrió un fallo o una interrupción en una operación previa,
   **When** el usuario ejecuta el flujo de recuperación soportado, **Then** el
   sistema intenta restaurar un estado de red válido y comunica el resultado con
   instrucciones claras.

### Edge Cases

- ¿Qué ocurre si el usuario intenta activar la protección cuando ya está activa?
- ¿Qué sucede si el equipo tiene configuración previa especial, como DNS manual,
  proxy, rutas personalizadas o filtros locales?
- ¿Cómo responde el sistema si el proceso termina abruptamente durante una
  activación, desactivación o recuperación?
- ¿Qué pasa si una exclusión solicitada entra en conflicto con una regla ya
  aplicada por el usuario?
- ¿Cómo se muestra el estado cuando la protección está parcialmente operativa,
  pero necesita atención del usuario?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: El sistema MUST permitir activar la protección de bloqueo de
  anuncios mediante un comando principal explícito.
- **FR-002**: El sistema MUST verificar y resguardar el estado de red relevante
  antes de aplicar cambios que puedan afectar la conectividad.
- **FR-003**: El sistema MUST rechazar o revertir operaciones cuando no pueda
  garantizar un camino de recuperación seguro.
- **FR-004**: El sistema MUST permitir desactivar la protección y restaurar el
  último estado de red resguardado por la herramienta.
- **FR-005**: El sistema MUST ofrecer un comando de estado que informe si la
  protección está activa, degradada o desactivada.
- **FR-006**: El sistema MUST presentar salidas legibles, consistentes y
  orientadas a acción para comandos exitosos, advertencias y errores.
- **FR-007**: El sistema MUST permitir crear y eliminar exclusiones puntuales
  para destinos legítimos sin desactivar la protección global.
- **FR-008**: El sistema MUST permitir listar las reglas esenciales y las
  exclusiones activas dentro del alcance del MVP.
- **FR-009**: El sistema MUST registrar eventos operativos mínimos de
  activación, desactivación, exclusión, error y recuperación.
- **FR-010**: El sistema MUST ofrecer un flujo de recuperación explícito para
  restaurar un estado de red válido después de errores o interrupciones.
- **FR-011**: El sistema MUST responder de forma idempotente o con mensajes
  claros cuando el usuario repita operaciones incompatibles con el estado actual.
- **FR-012**: El sistema MUST limitar el MVP a comandos esenciales de
  activación, desactivación, estado, recuperación y gestión básica de reglas.
- **FR-013**: El sistema MUST comunicar de manera explícita qué cambios están
  bajo control de la herramienta y qué configuraciones externas quedan fuera de
  su alcance.

### Key Entities *(include if feature involves data)*

- **Protection State**: Representa el estado operativo actual de la protección,
  incluyendo si está activa, degradada o desactivada y la última transición
  conocida.
- **Network Snapshot**: Representa la configuración de red resguardada para
  restaurar conectividad segura después de cambios o fallos.
- **Rule Entry**: Representa una regla esencial del MVP, ya sea de bloqueo o de
  exclusión, junto con su alcance y estado actual.
- **Operation Event**: Representa un evento visible para el usuario relacionado
  con activación, desactivación, exclusión, error o recuperación.

## Assumptions & Scope Boundaries *(optional)*

- El MVP está orientado a un único usuario local con permisos suficientes para
  operar la herramienta en su propio equipo.
- La primera versión prioriza una experiencia CLI limpia y confiable; una
  interfaz gráfica, sincronización remota y automatizaciones avanzadas quedan
  fuera de alcance.
- La gestión de reglas del MVP se limita a operaciones esenciales y no pretende
  cubrir todas las variantes de personalización posibles.
- El producto debe privilegiar recuperación y claridad operativa por encima de
  mantener el bloqueo activo en condiciones inciertas.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Al menos el 95% de los usuarios de prueba pueden activar la
  protección y confirmar el resultado correcto en menos de 3 minutos en su
  primer intento.
- **SC-002**: El 100% de las activaciones fallidas dejan al equipo en un estado
  de red recuperable mediante la propia herramienta.
- **SC-003**: Al menos el 90% de los usuarios de prueba identifican
  correctamente el estado de la herramienta usando solo la salida del comando de
  estado.
- **SC-004**: Al menos el 85% de los falsos positivos cubiertos por el MVP se
  resuelven mediante exclusiones puntuales sin desactivar completamente la
  protección.
- **SC-005**: Al menos el 90% de los usuarios de prueba califican la experiencia
  de uso inicial como clara y ordenada al ejecutar los comandos principales del
  MVP.
