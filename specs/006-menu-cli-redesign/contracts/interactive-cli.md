# Interactive CLI Contract: Rediseño de CLI por Vistas

## Purpose

Definir el contrato visible de la experiencia interactiva de Sentinel después de
la reestructuración a CLI clásica menu-driven.

## Entry Contract

- La entrada interactiva principal sigue siendo `sentinel` sin exigir nuevos
  flags para navegar por las acciones esenciales.
- La herramienta debe iniciarse en una terminal interactiva cuando el usuario
  quiera usar el flujo guiado.
- El flujo de transcript usado por pruebas debe seguir pudiendo recorrer la
  experiencia mediante entradas simuladas y obtener una salida estable.

## Home View Contract

- El home es la primera vista visible salvo que la herramienta detecte un estado
  de recuperación prioritaria ya previsto por la lógica actual.
- El home debe mostrar:
  - logo ASCII de Sentinel
  - nombre/subtítulo breve
  - lista corta de acciones principales
  - ayuda mínima de navegación
- El home no debe mezclar resultados operativos, tablas extensas ni mensajes
  arrastrados desde vistas previas.

## Navigation Contract

- Cada opción seleccionable del home debe abrir una vista dedicada o un estado
  independiente.
- Toda transición hacia una nueva vista debe limpiar la pantalla visible antes
  de mostrar el nuevo contenido.
- Toda vista secundaria debe dejar claro cómo volver, continuar o salir.
- La navegación debe mantenerse predecible con teclado y no depender de render
  reactivo persistente.

## Confirmation and Result Contract

- Las acciones sensibles deben mostrar una vista de confirmación o advertencia
  explícita antes de ejecutar cambios.
- Toda operación terminada debe cerrar en una vista de resultado con:
  - estado final
  - resumen breve
  - siguiente acción recomendada
  - opción de volver o salir
- Si una operación falla, el mensaje final debe permanecer visible hasta que el
  usuario navegue manualmente.

## Visual Feedback Contract

- La experiencia puede usar color ANSI, símbolos Unicode y spinners siempre que
  mejoren comprensión.
- El énfasis visual debe diferenciar al menos éxito, advertencia, error, estado
  activo y selección actual.
- Las operaciones perceptiblemente largas deben mostrar progreso visible.
- Si el terminal no soporta esos recursos, la salida debe seguir siendo legible
  usando texto plano.

## Compatibility Contract

- El rediseño no debe introducir nuevos comandos, flags ni configuraciones
  requeridas para el uso diario del flujo interactivo.
- Las capacidades actuales de estado, seguridad, activación, desactivación,
  instalación y recuperación deben seguir accesibles desde la experiencia
  guiada.
- El producto final no debe contener una experiencia TUI alternativa para el
  usuario.
