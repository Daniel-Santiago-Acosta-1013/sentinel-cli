# Quickstart: Rediseño de CLI por Vistas

## Goal

Validar manualmente y por suite automatizada que Sentinel ya se comporta como
una CLI clásica por vistas, sin residuos de TUI ni regresiones en los flujos
principales.

## Prerequisites

- Rust toolchain del proyecto instalado.
- Terminal interactiva disponible.
- Permisos y entorno habituales para ejecutar Sentinel en modo local.

## Validation Steps

1. Ejecutar validación base del proyecto:

```bash
cargo test
cargo clippy
```

2. Abrir Sentinel en una terminal interactiva:

```bash
cargo run
```

3. Verificar el home:

- muestra el logo ASCII de Sentinel
- se percibe limpio y minimalista
- contiene solo acciones principales y ayuda breve
- no muestra residuos de otra vista

4. Validar scripts de referencia del flujo guiado:

```bash
SENTINEL_SCRIPT="down,enter,enter,enter,exit" cargo run
SENTINEL_SCRIPT="down,enter,enter,enter,down,enter,enter,enter,exit" cargo run
```

5. Navegar por cada acción principal del home y confirmar que:

- cada selección abre una vista distinta o un resultado independiente
- la terminal se limpia al entrar
- existe una ruta clara para volver o salir

6. Ejecutar al menos un flujo largo o sensible y confirmar que:

- aparece progreso visible si la operación tarda más de 1 segundo
- existe confirmación previa cuando la acción modifica la red
- el resultado final indica estado y siguiente paso

7. Validar degradación visual:

- la salida sigue siendo legible si el entorno no usa color o símbolos
- los mensajes siguen teniendo sentido aun sin spinner

8. Confirmar limpieza arquitectónica final:

- no existe `src/tui/`
- `Cargo.toml` ya no declara `ratatui`
- la navegación vive en `src/cli/navigation.rs` y las vistas en `src/cli/views.rs`

## Expected Outcome

Sentinel se siente como una CLI profesional y guiada: home claro, vistas
separadas, navegación predecible, progreso visible cuando corresponde y
ninguna dependencia restante de una arquitectura TUI.
