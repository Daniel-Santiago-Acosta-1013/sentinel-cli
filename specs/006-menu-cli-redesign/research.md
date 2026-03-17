# Research: Rediseño de CLI por Vistas

## Decision 1: Adoptar un flujo por rutas y vistas explícitas

**Decision**: Reorganizar la experiencia interactiva como una secuencia de
vistas independientes con navegación explícita entre home, acciones,
confirmaciones, estados y resultados.

**Rationale**: El problema principal del producto es la percepción de pantalla
reactiva única. Un modelo por rutas hace que cada selección cambie de contexto
de forma visible, permite limpiar la pantalla entre vistas y encaja mejor con
una CLI guiada tradicional.

**Alternatives considered**:

- Mantener la pantalla única actual y solo cambiar copy/estilo. Rechazado
  porque no resuelve la confusión contextual ni la sensación de TUI.
- Migrar a una TUI más pulida. Rechazado porque contradice el objetivo explícito
  del feature.

## Decision 2: Reusar la lógica de negocio actual y aislar el cambio en la capa CLI

**Decision**: Mantener `control/`, `storage/`, `blocking/`, `engine/` e
`install/` como capas de negocio y concentrar la migración en `app.rs` y
`src/cli`.

**Rationale**: El valor del cambio está en UX/UI y arquitectura interactiva, no
en reescribir flujos de seguridad o recuperación ya existentes. Reducir el
área de cambio limita regresiones en conectividad y rollback.

**Alternatives considered**:

- Reescribir el flujo interactivo y los controladores a la vez. Rechazado por
  riesgo innecesario sobre operaciones sensibles de red.
- Mantener `src/tui` como fallback oculto. Rechazado porque prolonga la
  duplicidad arquitectónica que el feature quiere eliminar.

## Decision 3: Aplicar estética CLI con mejora progresiva y fallback limpio

**Decision**: Introducir ANSI styling, símbolos Unicode y spinners como una
mejora progresiva con degradación a texto plano cuando el terminal no soporte
esas capacidades o cuando el contexto de prueba requiera salida estable.

**Rationale**: El usuario pidió una CLI profesional, pero la constitución exige
claridad y moderación visual. Un sistema progresivo mejora percepción sin
volver dependiente el flujo de capacidades avanzadas del terminal.

**Alternatives considered**:

- Salida siempre coloreada y animada. Rechazado por compatibilidad y riesgo de
  snapshots inestables.
- Evitar cualquier mejora visual. Rechazado porque no cumple el objetivo de
  elevar la estética del CLI.

## Decision 4: Eliminar completamente la capa TUI y su dependencia

**Decision**: Retirar `src/tui` y la dependencia `ratatui` una vez que el flujo
CLI por vistas cubra la experiencia interactiva completa y sus pruebas.

**Rationale**: El producto no debe sentirse ni mantenerse como TUI. Mantener
esa capa introduce ambigüedad arquitectónica y aumenta el costo de mantenimiento.

**Alternatives considered**:

- Dejar `src/tui` sin uso temporalmente. Rechazado porque mantiene deuda
  técnica visible.
- Conservar `ratatui` para usos futuros. Rechazado por violar el principio de
  command surface y modularidad enfocada sin necesidad actual.

## Decision 5: Validar por contratos de vista, no solo por texto incidental

**Decision**: Actualizar contratos, snapshots e integraciones para validar
home, navegación, transiciones limpias, feedback de progreso y mantenimiento de
flujos críticos, evitando depender de texto incidental que pueda cambiar por
pulido estético.

**Rationale**: El rediseño cambiará gran parte de la salida visible. Probar
estructuras de vista y comportamientos reduce fragilidad y mantiene confianza
en el feature.

**Alternatives considered**:

- Regrabar snapshots masivamente sin rediseñar contratos. Rechazado porque no
  protege los comportamientos clave.
- Confiar solo en pruebas manuales. Rechazado por insuficiente para un flujo que
  toca red y recuperación.
