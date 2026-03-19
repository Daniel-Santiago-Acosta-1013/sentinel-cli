# Interactive Settings CLI Contract: Sentinel CLI

## Purpose

Definir el contrato visible de la ampliación del flujo menu-driven para incluir
`Ajustes`, la administración de `Dominios bloqueados` y la vista mejorada de
`Estado de Sentinel`.

## Home Contract

- El home sigue siendo la entrada principal de la experiencia interactiva.
- El home ya no debe mostrar la indicación `✓ Riesgo: Normal`.
- El home debe incluir `Ajustes` como opción principal visible al mismo nivel
  que las demás acciones.
- El home debe conservar la estética general, la ayuda mínima y la navegación
  consistente con el resto de Sentinel.

## Settings Navigation Contract

- `Ajustes` debe abrir una vista propia dentro del mismo flujo de rutas.
- `Dominios bloqueados` debe estar disponible como subopción visible de
  `Ajustes`.
- Toda vista bajo `Ajustes` debe ofrecer un camino claro para volver o salir.
- La navegación no debe requerir nuevos comandos, flags ni modos especiales.

## Blocked Domains Management Contract

- La vista `Dominios bloqueados` debe mostrar el listado vigente o un estado
  vacío comprensible.
- El usuario debe poder agregar, editar y eliminar dominios desde esa misma
  rama de navegación.
- La CLI debe rechazar dominios inválidos o duplicados con un mensaje claro.
- Después de una modificación confirmada, la vista debe reflejar el listado
  actualizado sin dejar al usuario en un estado ambiguo.

## Status View Contract

- La tabla principal de `Estado de Sentinel` no debe incluir `Riesgo`,
  `Resumen` ni `Accion sugerida`.
- La vista debe incluir una sección separada `Actividad de bloqueo`.
- `Actividad de bloqueo` debe presentarse como tabla con exactamente estas
  métricas:
  - `Bloqueos desde la activación`
  - `Dominios únicos bloqueados`
  - `Último bloqueo`
  - `Top dominios bloqueados`
- Si no hay datos de actividad, la vista debe expresarlo sin inventar valores.

## Visual Consistency Contract

- Los cambios deben respetar la línea estética actual del producto.
- La nueva navegación y las nuevas tablas deben sentirse como parte del mismo
  CLI, no como un flujo aparte.
- El énfasis visual debe seguir siendo sobrio y orientado a comprensión, no a
  decoración.

## Safety and Compatibility Contract

- La administración del listado activo no debe dejar el blocklist en un estado
  corrupto o ambiguo.
- El flujo interactivo existente de activación, estado, logs y recuperación
  debe seguir accesible.
- La salida debe seguir siendo legible en transcript y en terminal interactiva.
