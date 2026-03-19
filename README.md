# Sentinel CLI

Sentinel CLI es una herramienta de terminal para proteger el DNS del sistema
contra dominios publicitarios mediante un runtime local, cambios reversibles en
la configuración de red y flujos de recuperación seguros.

Hoy el producto está orientado a **macOS** y su punto de entrada público es un
**flujo interactivo en terminal**. La prioridad del proyecto es operar con
trazabilidad, rollback claro y una experiencia guiada fácil de entender.

## Qué hace

- Activa un runtime DNS local para bloquear dominios no deseados.
- Aplica cambios sobre los servicios de red detectados en macOS.
- Captura snapshots antes de tocar la red para poder restaurarla.
- Muestra un flujo interactivo con vistas separadas para:
  - activar o desactivar Sentinel
  - administrar `Ajustes`
  - listar y mantener `Dominios bloqueados`
  - revisar estado
  - ver logs
  - recuperar la red
- Mantiene estado, eventos e instalación en archivos locales del usuario.

## Estado actual del proyecto

- Plataforma principal: macOS
- Interfaz actual soportada: CLI interactiva en TTY
- Modo seguro para desarrollo y pruebas: `SENTINEL_FAKE_PLATFORM=1`
- Instalador incluido: `scripts/install-sentinel.sh`

## Requisitos

- Rust toolchain compatible con la versión del proyecto
- macOS para uso real sobre la red del sistema
- Terminal interactiva para el flujo normal

Para desarrollo y pruebas no destructivas puedes ejecutar Sentinel con
`SENTINEL_FAKE_PLATFORM=1`, lo que evita tocar la red real.

## Instalación

### Opción 1: npm

```bash
npm install -g @daniel_santiago/sentinel-cli
```

### Opción 2: Homebrew

```bash
brew install Daniel-Santiago-Acosta-1013/homebrew-tap/sentinel
```

### Opción 3: usar el instalador del repositorio

```bash
sh scripts/install-sentinel.sh
```

El instalador:
- detecta si corresponde `install`, `update` o `reinstall`
- valida el binario antes de reemplazar el ejecutable anterior
- instala `sentinel` en un directorio invocable por PATH

### Opción 4: ejecutar desde código fuente

```bash
cargo run
```

## Uso

### Ejecución interactiva

```bash
cargo run
```

Si el entorno es un TTY, Sentinel abre el flujo guiado en terminal.

### Desarrollo seguro sin tocar la red real

```bash
SENTINEL_FAKE_PLATFORM=1 cargo run
```

### Controles de navegación

- `↑` / `↓`: mover selección
- `Enter`: confirmar
- `Esc`: volver
- `q`: salir
- al editar dominios: escribir para reemplazar el valor, `Backspace` para borrar

## Configuración y datos

Sentinel gestiona sus archivos mediante el directorio de soporte de la
aplicación definido por el sistema operativo. También puede forzarse un root
custom con la variable `SENTINEL_HOME`.

Estructura principal:

- `config/config.toml`: configuración persistente
- `state/state.json`: estado operativo
- `state/events.jsonl`: eventos y trazas recientes
- `data/install.json`: metadatos de instalación
- `state/snapshots/`: snapshots de red para recuperación

Valores de configuración por defecto relevantes:

- DNS upstream: `1.1.1.1:53`
- DNS local: `127.0.0.1`
- confirmación previa a cambios de red: habilitada

## Variables de entorno útiles

### Para desarrollo y pruebas

- `SENTINEL_FAKE_PLATFORM=1`: evita cambios reales sobre macOS
- `SENTINEL_HOME=/ruta/custom`: cambia el directorio raíz de estado
- `SENTINEL_DNS_PORT=19053`: cambia el puerto DNS local
- `SENTINEL_DNS_BIND=127.0.0.1`: cambia la IP de bind del runtime
- `SENTINEL_SCRIPT="..."`: ejecuta el flujo guiado en modo script para tests

### Internas

Estas variables existen para runtime o validación interna y no forman parte del
surface normal para usuarios:

- `SENTINEL_INTERNAL_MODE=runtime`
- `SENTINEL_INTERNAL_MODE=print-version`

## Desarrollo

### Ejecutar tests

```bash
cargo test
```

### Validación estática

```bash
cargo clippy --all-targets --all-features
```

### Formato

```bash
cargo fmt
```

## Estructura del repositorio

```text
src/
├── blocking/   # runtime y bundle de bloqueo
├── cli/        # navegación, render, copy y terminal interactiva
├── control/    # activación, seguridad, snapshots y recuperación
├── install/    # versión e instalación
├── platform/   # integración con macOS
└── storage/    # configuración, estado, eventos e instalación

tests/
├── contract/
├── integration/
├── snapshot/
└── support/
```

## Calidad y seguridad operativa

Sentinel prioriza:

- cambios reversibles sobre la red
- recuperación explícita antes de declarar éxito
- mensajes de error con siguiente paso seguro
- separación clara entre estado activo, degradado y recuperación

Si una operación no puede garantizar un camino seguro de vuelta, debe detenerse
o dejar el sistema en un estado explícitamente recuperable.

## Releases

El repositorio incluye automatización de release para mantenimiento en:

- `.github/workflows/release.yml`
- `scripts/release/`

Ese flujo valida consistencia de versión, artefactos y canales de distribución
antes de publicar.

La creación de versiones ya no se inicia empujando tags locales. El flujo
oficial se ejecuta desde GitHub Actions con `workflow_dispatch`, recibe la
nueva versión como input, alinea `Cargo.toml` y las superficies de packaging,
genera un commit auditable, crea el tag y solo después ejecuta el release y
deploy desde ese mismo estado.

## Licencia

Este proyecto se distribuye bajo la licencia Apache 2.0. Consulta
[LICENSE](/Users/santiagoacosta/Documents/personal-projects/sentinel-cli/LICENSE).
