# Contract: Interacción de la CLI

## Objetivo

Definir el comportamiento estable de la experiencia interactiva visible para el
usuario y para las pruebas de contrato/snapshot.

## Reglas de entrada

- La navegación principal usa `↑`, `↓`, `Enter`, `Esc` y la salida explícita.
- Las acciones sensibles requieren confirmación explícita antes de alterar red.
- El modo scriptado debe poder representar estas entradas con tokens estables
  equivalentes para pruebas automatizadas.

## Reglas de salida

- Toda etiqueta, ayuda, advertencia, confirmación y resumen visible debe estar
  en español.
- Cada cambio de vista debe limpiar o refrescar la terminal antes de renderizar
  el nuevo contenido.
- La vista activa debe incluir:
  - título de pantalla
  - resumen breve del estado actual
  - lista de acciones disponible
  - ayuda de teclado en español

## Contrato de vistas

### Inicio

- Debe mostrar el estado actual de protección.
- Debe ofrecer acciones para seguridad, activar/desactivar, ver estado y recuperar.

### Estado

- Debe mostrar al menos una tabla con filas para protección, riesgo, snapshot
  activo y siguiente acción recomendada.
- Si la terminal es angosta, puede reducir columnas pero no omitir los campos
  esenciales de seguridad.

### Confirmación

- Debe nombrar la acción sensible en español.
- Debe advertir que la acción cambia o restaura el estado de red.

### Recuperación

- Debe indicar qué snapshot o referencia se usará para restaurar.
- Debe mostrar el resultado de la verificación posterior a la restauración.

## Marcadores estables para pruebas

- El transcript scriptado debe incluir identificadores visibles y estables para:
  - `Pantalla`
  - `Protección`
  - `Riesgo`
  - `Siguiente acción`
- Las tablas de estado deben conservar encabezados en español consistentes
  aunque cambie el contenido de valores.

## Errores y estados degradados

- Cuando un chequeo falle, la salida debe explicar por qué se bloqueó la acción
  y cuál es la alternativa segura.
- Cuando la restauración no coincida con el snapshot esperado, la herramienta
  debe marcar estado degradado y ofrecer recuperación antes de permitir otra
  mutación de red.
