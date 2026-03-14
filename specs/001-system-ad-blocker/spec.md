# Feature Specification: CLI System Ad Blocking

**Feature Branch**: `001-system-ad-blocker`  
**Created**: 2026-03-14  
**Status**: Draft  
**Input**: User description: "quiero que diseñemos una herrmaineta CLI que permita bloquead anuncios a nivel de sitema de manera precisa, cuidadndo configuraciones de red que tenga el equipo para evitar dejar el dispositivo sin conexion, el stack a usar va a ser Rust y las dependencias mas indicadas para este fin lo ideal es crear el motor vpn mas no ser un wrapper de una vpn para interceptar el rafico, esto para tener un control total y que la herramienta, necesito que escojas una arquitectura mas indicada para que el codigo sea escalable que no se vuelva codigo escalable y que sea un poryecto organizado usa Rustdoc comments de manera suprudente minima y escencial para uno como humano saber que es lo que hace el codigo"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Activar bloqueo seguro del sistema (Priority: P1)

Como usuario de terminal, quiero activar el bloqueo de anuncios a nivel de sistema sin perder conectividad para proteger toda mi navegación y el tráfico de aplicaciones locales con un solo comando.

**Why this priority**: Es el flujo principal del producto. Si no puede activarse de forma segura y reversible, la herramienta no entrega valor utilizable.

**Independent Test**: Puede probarse ejecutando el comando de activación en un equipo con configuración de red existente, verificando que el bloqueo queda activo, que la conexión se mantiene y que la configuración previa puede restaurarse.

**Acceptance Scenarios**:

1. **Given** un equipo con conexión funcional y configuraciones de red activas, **When** el usuario ejecuta el comando para activar el bloqueo, **Then** la herramienta aplica el bloqueo sin dejar el dispositivo sin acceso a red.
2. **Given** que la activación encuentra una condición insegura o un fallo operativo, **When** la herramienta no puede garantizar conectividad, **Then** cancela o revierte el cambio y comunica el motivo al usuario.

---

### User Story 2 - Preservar y restaurar el estado de red (Priority: P2)

Como usuario, quiero que la herramienta preserve el estado previo de red y lo restaure al desactivar o ante un error para evitar configuraciones rotas después del uso.

**Why this priority**: La confianza en una herramienta de nivel sistema depende de que no deje efectos colaterales duraderos sobre la conectividad del equipo.

**Independent Test**: Puede probarse activando el bloqueo, modificando el estado controlado por la herramienta y luego ejecutando desactivación o recuperación para verificar que la red vuelve al estado anterior conocido.

**Acceptance Scenarios**:

1. **Given** que el bloqueo fue activado correctamente, **When** el usuario ejecuta el comando para desactivarlo, **Then** la herramienta restaura la configuración de red previamente resguardada.
2. **Given** una activación parcial o una interrupción inesperada, **When** el usuario ejecuta un comando de recuperación o la herramienta detecta reinicio pendiente, **Then** se restaura un estado de red válido y se informa el resultado.

---

### User Story 3 - Operar con control y observabilidad (Priority: P3)

Como usuario, quiero consultar estado, reglas activas y eventos relevantes para entender qué está bloqueando la herramienta y corregir problemas sin desactivar toda la protección.

**Why this priority**: La precisión operativa requiere visibilidad. Sin estado claro y trazabilidad mínima, los falsos positivos y fallos son difíciles de resolver.

**Independent Test**: Puede probarse activando el bloqueo, consultando el estado y revisando eventos o contadores de bloqueo para confirmar que la herramienta expone información útil para diagnóstico.

**Acceptance Scenarios**:

1. **Given** que la protección está activa, **When** el usuario consulta el estado, **Then** la herramienta muestra si el bloqueo está habilitado, la salud operativa y la última configuración aplicada.
2. **Given** que ciertos dominios o destinos están siendo bloqueados, **When** el usuario consulta la actividad reciente, **Then** la herramienta muestra información suficiente para identificar bloqueos esperados o inesperados.

### Edge Cases

- ¿Qué ocurre si el equipo cambia de red mientras la protección está activa?
- ¿Cómo responde la herramienta si el usuario ya tiene una configuración especial de red, como DNS manual, proxy o rutas personalizadas?
- ¿Qué sucede si el proceso termina abruptamente durante una activación o una restauración?
- ¿Cómo se comporta el sistema cuando una actualización de reglas introduce un falso positivo que afecta un servicio legítimo?
- ¿Qué pasa si el usuario intenta activar la protección cuando ya está activa o desactivarla cuando ya está inactiva?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: El sistema MUST permitir activar el bloqueo de anuncios a nivel de sistema mediante un comando explícito.
- **FR-002**: El sistema MUST crear un resguardo del estado de red relevante antes de aplicar cambios que puedan afectar conectividad.
- **FR-003**: El sistema MUST validar que existe una ruta de recuperación antes de confirmar la activación del bloqueo.
- **FR-004**: El sistema MUST cancelar o revertir cambios parciales cuando detecte que no puede mantener una configuración de red válida.
- **FR-005**: El sistema MUST permitir desactivar el bloqueo y restaurar el último estado de red resguardado por la herramienta.
- **FR-006**: El sistema MUST ofrecer un comando de verificación de estado que informe si la protección está activa, degradada o desactivada.
- **FR-007**: El sistema MUST registrar eventos operativos relevantes de activación, desactivación, recuperación, error y cambios de reglas.
- **FR-008**: El sistema MUST aplicar reglas de bloqueo de forma precisa para reducir el impacto sobre tráfico legítimo no publicitario.
- **FR-009**: El sistema MUST permitir excluir destinos o comportamientos específicos del bloqueo cuando el usuario necesite corregir falsos positivos.
- **FR-010**: El sistema MUST detectar intentos repetidos o conflictivos de activación y desactivación y responder de forma idempotente o con instrucciones claras.
- **FR-011**: El sistema MUST preservar la capacidad del usuario para recuperar conectividad incluso después de cierres inesperados o reinicios del equipo.
- **FR-012**: El sistema MUST informar de forma clara qué cambios controla directamente la herramienta y cuáles quedan fuera de su alcance.
- **FR-013**: El sistema MUST limitar su alcance a funciones de bloqueo, restauración, estado y administración de reglas, sin asumir la gestión general de toda la red del equipo.

### Key Entities *(include if feature involves data)*

- **Network Snapshot**: Representa el estado de red resguardado antes de aplicar cambios, incluyendo parámetros necesarios para volver a una configuración válida.
- **Protection Session**: Representa una instancia activa o reciente de protección, con estado operativo, momento de activación y resultado de verificaciones de salud.
- **Blocking Rule Set**: Representa la colección versionada de reglas de bloqueo y exclusión aplicables en una sesión.
- **Recovery Event**: Representa un incidente que obliga a revertir, reparar o confirmar el estado de red tras un fallo o interrupción.

### Assumptions

- La herramienta está dirigida a un único operador local con permisos suficientes para aplicar cambios de red en su propio equipo.
- El valor principal del MVP es proteger navegación y tráfico local común del sistema, no administrar entornos empresariales multiusuario.
- Las exclusiones y ajustes finos serán administrados por comandos de la CLI y no por una interfaz gráfica en esta fase.
- La herramienta debe priorizar siempre la restauración segura de conectividad por encima de mantener el bloqueo activo.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Al menos el 95% de las activaciones en equipos con conectividad previa válida finalizan con la protección habilitada sin pérdida de conexión perceptible para el usuario.
- **SC-002**: El 100% de los intentos fallidos de activación dejan al equipo en un estado de red recuperable mediante la propia herramienta.
- **SC-003**: Los usuarios pueden activar o desactivar la protección y confirmar el estado resultante en menos de 2 minutos en su primer intento.
- **SC-004**: Al menos el 90% de los falsos positivos reportados por usuarios pueden corregirse sin desactivar completamente la protección.
- **SC-005**: Al menos el 90% de los usuarios de prueba identifican correctamente si la herramienta está activa, degradada o recuperada consultando solo la salida de estado.
