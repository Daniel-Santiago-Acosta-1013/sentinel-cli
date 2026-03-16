# Quickstart: CLI Guiada por Menús Segura

## Prerrequisitos

- Rust 1.90.0 o superior disponible mediante `cargo`
- macOS 14+ para validación real de red
- Terminal interactiva compatible con limpieza de pantalla y captura de teclado

## Bootstrap de desarrollo

1. Instala dependencias y compila el binario con `cargo build`.
2. Ejecuta la herramienta en modo seguro con plataforma falsa:
   `SENTINEL_FAKE_PLATFORM=1 SENTINEL_SCRIPT=salir cargo run`
3. Ejecuta la suite base:
   `cargo test`

## Flujo manual de validación

1. Inicia Sentinel en plataforma falsa y confirma que la vista principal se
   muestra en español y sin texto residual de sesiones anteriores.
2. Navega con flechas y `Enter`; confirma que al cambiar de pantalla la
   terminal se limpia o refresca antes de imprimir la nueva vista.
3. Abre la vista de estado y valida que la información de protección, red y
   seguridad aparece en tablas legibles.
4. Ejecuta chequeos de seguridad, activa protección y desactívala; confirma que
   la red vuelve al estado original sin requerir reinicio.
5. Fuerza un fallo o interrupción y verifica que el siguiente arranque conduce
   primero por recuperación guiada.
6. Repite la validación con configuración DNS personalizada simulada y múltiples
   servicios para comprobar que el snapshot conserva el estado original completo.

## Validación automatizada esperada

- `cargo test --test integration`
- `cargo test --test contract`
- `cargo test --test snapshot`
- Casos de integración para activación, desactivación, recuperación e
  interrupciones con plataforma falsa
- Contratos de transcript en español y tablas de estado estables
- Validaciones de integridad de snapshot comparando DNS original vs. DNS
  restaurado por servicio

## Señales de aceptación antes de implementar tareas

- La experiencia principal no depende de `ratatui` para el flujo normal del usuario.
- Los transcripts visibles y snapshots de salida están completamente en español.
- Existe cobertura automatizada para verificar que la desactivación no deja la
  red en estado degradado sin recuperación guiada.
- La vista de estado usa tablas y conserva legibilidad en terminales angostas.
