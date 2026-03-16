# Research: CLI Guiada por Menús Segura

## Decision: Sustituir el flujo principal de `ratatui` por renderizado lineal con `crossterm`

**Rationale**: El requerimiento ya no es una TUI en pantalla alterna sino una
CLI guiada por menús que limpie o refresque la terminal sin dejar texto
acumulado. Un renderizador lineal controlado con `crossterm` permite mantener
teclas precisas, refresco limpio y una experiencia más cercana a una CLI
tradicional que a una aplicación TUI completa.

**Alternatives considered**:
- Mantener `ratatui` y retocar estilos: sigue pareciendo una TUI y conserva
  complejidad innecesaria para el objetivo actual.
- Cambiar a prompts secuenciales sin navegación: reduce complejidad, pero
  pierde visibilidad del estado y degrada la experiencia guiada.

## Decision: Mostrar los estados operativos en tablas usando `comfy-table`

**Rationale**: El feature exige estados legibles, comparables y estéticos.
`comfy-table` ya encaja con la necesidad de tablas limpias y es más apropiado
que construir tablas manuales con espacios frágiles por ancho de terminal.

**Alternatives considered**:
- Tablas armadas a mano con padding: más propensas a romperse con anchos
  variables y localización al español.
- Listas de texto simples: más fáciles de implementar, pero insuficientes para
  el requisito de claridad visual en estado y recuperación.

## Decision: Centralizar toda la copia visible en español dentro de módulos de CLI

**Rationale**: La herramienta debe ser completamente visible en español, y la
mezcla actual de textos en inglés dispersos entre estado, pantallas y tests
haría más probable la inconsistencia. Un punto único de textos reduce errores y
vuelve más predecibles los contratos y snapshots.

**Alternatives considered**:
- Traducir cadenas directamente donde se usan: más rápido al inicio, pero deja
  inconsistencias y dificulta mantenimiento.
- Añadir un sistema completo de i18n: más flexible, pero fuera de alcance
  porque solo se requiere español en esta fase.

## Decision: Verificar integridad de red después de desactivar o recuperar

**Rationale**: El problema crítico reportado no es solo fallar al restaurar, sino
  terminar con la red dañada hasta reiniciar el equipo. Además de capturar
  snapshots, la herramienta debe comparar el estado restaurado con el snapshot
  original y marcar degradación si hay diferencias relevantes.

**Alternatives considered**:
- Asumir éxito después de escribir la restauración: demasiado riesgoso para una
  herramienta que toca DNS del sistema.
- Delegar verificación al usuario: incompatible con el objetivo de no dejar el
  equipo en mal estado.

## Decision: Extender la plataforma falsa para probar múltiples servicios y DNS personalizados

**Rationale**: Los tests actuales ya usan una plataforma falsa y eso permite
  validar seguridad sin tocar la red real. Para cubrir el problema descrito,
  esa plataforma debe simular múltiples servicios, DNS previos, cambios
  externos e interrupciones para que la suite detecte regresiones antes de
  tocar un equipo real.

**Alternatives considered**:
- Probar solo contra un servicio falso único: no cubre restauración completa ni
  configuraciones reales del usuario.
- Probar únicamente en equipos reales: aumenta riesgo, lentitud e inestabilidad
  de la suite.

## Decision: Mantener el modo scriptado como contrato de pruebas, pero con transcript en español

**Rationale**: Las pruebas de integración ya dependen de una ejecución scriptada
  reproducible. Mantener ese modo permite validar navegación, tablas, limpieza
  de salida y estados de seguridad sin depender de interacción manual.

**Alternatives considered**:
- Eliminar el modo scriptado y probar solo input en TTY real: demasiado frágil
  y lento para una suite repetible.
- Mantener el transcript actual en inglés: contradice el requerimiento de
  experiencia totalmente en español y debilita el contrato visible.

## Decision: Mantener el alcance de rendimiento acotado a UX fluida y operaciones seguras

**Rationale**: El rendimiento relevante aquí no es throughput masivo, sino
  evitar esperas innecesarias al navegar y no cerrar operaciones de red hasta
  comprobar seguridad. La planificación debe optimizar refresco de vista,
  chequeos rápidos y validación fiable en vez de introducir concurrencia o
  optimizaciones especulativas.

**Alternatives considered**:
- Priorizar animaciones o efectos visuales: añade costo sin aportar a claridad
  ni seguridad.
- Priorizar microoptimizaciones de bajo nivel antes de tener contratos estables:
  aumenta complejidad y no resuelve el defecto principal.
