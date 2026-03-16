# Contract: Seguridad e Integridad de Red

## Objetivo

Definir las garantías mínimas que Sentinel debe cumplir antes, durante y
después de cambiar configuración de red del equipo.

## Precondiciones para activar protección

- Debe existir al menos un servicio de red detectable.
- Debe capturarse un snapshot restorable de la configuración original.
- Debe verificarse que la ruta de recuperación está disponible antes de tocar DNS.
- Si el puerto o runtime requerido no es seguro, la activación debe detenerse.

## Reglas durante cambios de red

- Sentinel solo puede modificar servicios incluidos en el snapshot activo.
- Los DNS originales deben preservarse por servicio, incluso cuando estén vacíos.
- Si una mutación falla a mitad de proceso, la herramienta debe entrar en flujo
  de recuperación y no declarar éxito parcial.

## Reglas para desactivación o recuperación

- La desactivación debe restaurar el snapshot original asociado a la sesión activa.
- Después de restaurar, Sentinel debe releer el estado de red y compararlo con
  el snapshot esperado antes de finalizar.
- Si la comparación falla, el estado persistido debe quedar como degradado o en
  recuperación, nunca como inactivo exitoso.

## Reglas de validación automatizada

- Las pruebas deben cubrir:
  - servicios con DNS personalizados previos
  - múltiples servicios activos
  - interrupción de activación o desactivación
  - recuperación desde snapshot faltante o incompleto
  - verificación posterior exitosa y fallida
- Los tests deben afirmar que el DNS restaurado coincide con el DNS capturado
  originalmente por servicio.
- Ningún caso de aceptación puede requerir reinicio del equipo como paso normal
  para recuperar conectividad tras una desactivación correcta.
