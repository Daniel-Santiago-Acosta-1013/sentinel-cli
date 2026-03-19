# Implementation Plan: Ajustes de Bloqueo y Release Alineado

**Branch**: `008-settings-release-alignment` | **Date**: 2026-03-19 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/008-settings-release-alignment/spec.md`

## Summary

Extender la CLI menu-driven actual de Sentinel sin romper su línea estética ni
su superficie pública: el home pierde la indicación `✓ Riesgo: Normal`, gana la
entrada principal `Ajustes`, y la nueva vista `Dominios bloqueados` permite
administrar el listado activo de dominios bloqueados dentro del flujo
interactivo existente. En paralelo, `Estado de Sentinel` se simplifica y añade
una tabla de `Actividad de bloqueo` basada en eventos estructurados. Para
release, el plan migra la creación de versiones a un workflow centralizado en
GitHub Actions con dos etapas separadas: alineación versionada del repositorio
y, solo después, release/deploy desde un estado trazable y consistente.

## Technical Context

**Language/Version**: Rust 1.90.0 (edition 2024) para el binario CLI y POSIX
shell para automatización de release  
**Primary Dependencies**: `crossterm`, `comfy-table`, `tokio`, `serde`,
`serde_json`, `toml`, `miette`, `thiserror`, `tracing`,
`tracing-subscriber`, `directories`, `uuid`, `chrono`, `hickory-proto`,
`git`, `cargo`, `tar`, `shasum`, `curl`, `npm` CLI y GitHub Actions  
**Storage**: Archivos locales bajo el directorio de soporte de la aplicación
(`TOML`, `JSON`, `JSONL`, mirror del blocklist) y archivos del repositorio para
versionado/release (`Cargo.toml`, `packaging/`, `scripts/release/`,
`.github/workflows/`)  
**Testing**: `cargo test`, `cargo clippy`, suites `contract`, `integration` y
`snapshot`, más fixtures de release en `tests/support/`  
**Target Platform**: Sesiones interactivas de terminal para usuarios finales y
GitHub-hosted runners para CI/CD; foco operativo actual en macOS/Linux  
**Project Type**: Aplicación CLI única en Rust con automatización de
distribución en el mismo repositorio  
**Performance Goals**: El home y las vistas de estado deben renderizarse de
forma inmediata en terminal; la tabla de actividad debe reflejar el estado
vigente al abrir la vista; el job previo de versionado debe bloquear o preparar
la release en pocos minutos antes de cualquier despliegue  
**Constraints**: Mantener el flujo menu-driven actual, no agregar nuevos
comandos o flags públicos, preservar seguridad operativa de flujos que afectan
red, evitar sobrescribir trabajo manual del usuario en la gestión de dominios,
centralizar el versionado solo en el workflow oficial y fallar en modo seguro
ante cualquier desalineación de versión o canal  
**Scale/Scope**: Un home principal, una nueva rama de navegación (`Ajustes`),
una vista CRUD para dominios bloqueados, una ampliación de `Estado de Sentinel`
con métricas de bloqueo, un workflow principal de release con al menos dos
jobs, varios scripts de soporte de release/versionado y actualización de
pruebas/documentación relacionadas

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **Minimal Command Surface**: PASS. La feature conserva la entrada principal
  `sentinel` y no introduce nuevos comandos ni flags; `Ajustes` vive dentro del
  menú existente y el versionado se mueve a CI sin exponer superficie adicional
  al usuario final.
- **Exceptional Terminal Experience**: PASS. Los cambios atacan directamente
  claridad visual del home, reducción de ruido en estado, navegación coherente y
  gestión legible de dominios bloqueados.
- **Safety Before Blocking**: PASS. La administración de dominios toca la lista
  efectiva de bloqueo y, por tanto, exige escritura atómica, validación de
  dominios y recarga consistente; el flujo de release también debe operar con
  bloqueo preventivo ante inconsistencia.
- **Focused, Modular Quality**: PASS. La implementación puede concentrarse en
  `src/cli`, `src/storage`, `src/engine` y `scripts/release/` sin mezclar la
  lógica interactiva con la de distribución ni inflar módulos existentes.
- **Deliberate Delivery**: PASS. El alcance se limita a la nueva experiencia de
  ajustes/bloqueo, la mejora de estado y la centralización exacta del versionado
  y deploy. Quedan fuera nuevos comandos, nuevos canales de release, rediseños
  visuales mayores y telemetría adicional no necesaria.

**Post-Design Re-check**: PASS. El diseño propuesto mantiene superficie pública
mínima, conserva una salida de terminal sobria y orientada a acción, trata con
seguridad los cambios al listado efectivo de bloqueo, separa módulos de CLI,
storage y release, y difiere cualquier expansión no esencial como filtros
avanzados, import/export de dominios o nuevos canales de distribución.

## Project Structure

### Documentation (this feature)

```text
specs/008-settings-release-alignment/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   ├── interactive-settings-cli.md
│   └── release-versioning-workflow.md
└── tasks.md
```

### Source Code (repository root)

```text
.github/
└── workflows/
    └── release.yml

packaging/
├── homebrew/
│   └── sentinel.rb.tpl
└── npm/
    ├── README.md
    ├── bin/
    │   └── sentinel.js
    └── package.json

scripts/
├── install-sentinel.sh
└── release/
    ├── authorize_release.sh
    ├── build_release_artifacts.sh
    ├── common.sh
    ├── inspect_release_state.sh
    ├── publish_github_release.sh
    ├── publish_homebrew.sh
    ├── publish_npm.sh
    ├── resolve_version.sh
    └── summarize_release.sh

src/
├── app.rs
├── cli/
│   ├── copy.rs
│   ├── menu_state.rs
│   ├── navigation.rs
│   ├── output.rs
│   ├── renderer.rs
│   ├── styles.rs
│   ├── terminal.rs
│   └── views.rs
├── core/
│   ├── events.rs
│   └── rules.rs
├── engine/
│   ├── dns.rs
│   └── runtime.rs
├── install/
│   └── version.rs
└── storage/
    ├── config.rs
    ├── events.rs
    ├── install.rs
    ├── mod.rs
    └── state.rs

tests/
├── contract/
│   ├── interaction_contract.rs
│   └── release_automation_contract.rs
├── integration/
│   ├── end_to_end_cli.rs
│   ├── interactive_activation.rs
│   └── release_pipeline.rs
├── snapshot/
│   ├── home_and_activation.rs
│   └── recovery_and_status.rs
└── support/
    └── release_fixtures.rs
```

**Structure Decision**: Mantener Sentinel como un único binario CLI en Rust.
Los cambios visuales y de navegación se concentran en `src/cli` y `src/app.rs`;
la administración y persistencia del listado efectivo de bloqueo se apoyan en
`src/storage` y `src/core`; la captura de actividad de bloqueo se conecta desde
`src/engine`/`src/storage`; y el versionado/release sigue como una capa de
mantenimiento separada en `.github/workflows/`, `scripts/release/` y
`packaging/`.

## Complexity Tracking

No se requieren excepciones constitucionales en esta fase. La complejidad nueva
queda justificada por dos necesidades directas del usuario: administrar el
listado activo de dominios bloqueados desde la propia CLI y eliminar la
posibilidad de desalineación entre versión interna, tag y canales de release.

## Current Product and Release Gap

- El home actual todavía muestra una insignia de riesgo que el usuario ya no
  quiere ver en la pantalla principal.
- La navegación principal no incluye una sección de ajustes para administrar el
  blocklist activo desde la propia CLI.
- El producto no expone aún una vista interactiva para revisar, agregar, editar
  y eliminar dominios bloqueados.
- `Estado de Sentinel` todavía mezcla campos de baja señal (`Riesgo`,
  `Resumen`, `Accion sugerida`) con información útil de estado.
- La actividad real de bloqueo no se resume en una tabla operativa para el
  usuario.
- El release actual depende de tags locales empujados manualmente y no incluye
  un job previo de alineación total de versión antes del deploy.
- La versión vive en varias superficies dependientes y hoy no existe un paso
  único que las sincronice, registre el cambio y luego publique desde ese
  estado exacto.

## Scope Boundaries

**In scope**

- Eliminar `✓ Riesgo: Normal` del home.
- Incorporar `Ajustes` como opción principal del menú.
- Agregar la vista `Dominios bloqueados` con alta, edición, eliminación y
  revisión del listado vigente.
- Ajustar la vista `Estado de Sentinel` para quitar `Riesgo`, `Resumen` y
  `Accion sugerida`.
- Añadir la tabla `Actividad de bloqueo` con las cuatro métricas pedidas.
- Persistir y validar el listado activo de dominios bloqueados con recarga
  coherente.
- Registrar actividad de bloqueo suficiente para poblar la nueva tabla.
- Reestructurar el workflow de release para que el versionado quede
  centralizado en GitHub Actions con job previo de alineación, commit y tag.
- Garantizar consistencia entre versión interna, tag, artefactos, npm y
  Homebrew.
- Actualizar pruebas, fixtures y documentación necesarias para respaldar ambos
  cambios.

**Out of scope**

- Añadir nuevos comandos o flags públicos para administrar dominios.
- Rediseñar por completo la estética general del CLI fuera de lo necesario para
  conservar coherencia visual.
- Introducir filtros avanzados, paginación compleja o importación/exportación
  masiva de dominios bloqueados.
- Incorporar nuevas métricas de telemetría más allá de las cuatro solicitadas.
- Añadir prereleases, canales de distribución adicionales o publicación local
  manual como rutas soportadas del release oficial.
- Redefinir la lógica de negocio de activación, desactivación o recuperación más
  allá de lo imprescindible para integrar la nueva gestión del blocklist.

## Components and Files Likely Impacted

- **`src/cli/navigation.rs`**: nuevos `Route` y `MenuActionId` para `Ajustes` y
  `Dominios bloqueados`.
- **`src/cli/menu_state.rs`**: composición del menú principal, estado de la
  nueva vista, selección de elementos y acciones CRUD.
- **`src/cli/views.rs`**: render del home sin badge de riesgo, nueva pantalla de
  ajustes, nueva pantalla de dominios bloqueados y render combinado de estado +
  actividad.
- **`src/cli/output.rs`**: tabla de estado recortada y nueva tabla de actividad
  de bloqueo.
- **`src/cli/copy.rs`**: textos de navegación, estados vacíos, mensajes de
  confirmación y errores de validación para dominios.
- **`src/app.rs`**: orquestación de nuevas acciones, carga/recarga de datos de
  dominios bloqueados y refresco de actividad en cada navegación relevante.
- **`src/storage/config.rs`** y/o **nuevo store en `src/storage/`**: persistir
  el listado efectivo gestionable y su validación.
- **`src/storage/events.rs`**: nuevo evento o enriquecimiento de eventos para
  registrar bloqueos útiles para agregación.
- **`src/storage/state.rs`**: posible snapshot agregado de actividad si se
  requiere cacheo visible en la UI.
- **`src/core/rules.rs`**: composición entre reglas built-in y dominios
  bloqueados gestionados por el usuario.
- **`src/engine/dns.rs`** y **`src/engine/runtime.rs`**: emisión de eventos de
  bloqueo y uso del listado efectivo actualizado.
- **`.github/workflows/release.yml`**: cambio de activación a workflow con input
  de versión y separación explícita entre job de versionado y job de release.
- **`scripts/release/common.sh`**: utilidades compartidas para alinear,
  resolver y verificar superficies de versión.
- **nuevo script de alineación en `scripts/release/`**: actualización
  automática de `Cargo.toml` y demás superficies dependientes de versión.
- **`scripts/release/resolve_version.sh`**, **`build_release_artifacts.sh`**,
  **`publish_github_release.sh`**, **`publish_npm.sh`** y
  **`publish_homebrew.sh`**: adaptación al nuevo flujo basado en versión
  solicitada, commit auditable y tag generado por CI.
- **`packaging/npm/package.json`**, **`packaging/homebrew/sentinel.rb.tpl`**,
  **`scripts/install-sentinel.sh`**, **`src/install/version.rs`** y fixtures de
  tests de release: superficies que deben quedar alineadas con la nueva versión.
- **`tests/snapshot/home_and_activation.rs`** y
  **`tests/snapshot/recovery_and_status.rs`**: actualización de snapshots del
  home y de la vista de estado.
- **`tests/contract/interaction_contract.rs`** y
  **`tests/integration/end_to_end_cli.rs`**: validación del nuevo flujo
  menu-driven para ajustes y dominios bloqueados.
- **`tests/contract/release_automation_contract.rs`**,
  **`tests/integration/release_pipeline.rs`** y
  **`tests/support/release_fixtures.rs`**: adaptación al versionado centralizado
  y a la actualización automática de superficies.
- **`README.md`** y **`packaging/npm/README.md`**: documentación del flujo
  actualizado para usuarios y mantenedores si hacen referencia al home, estado o
  release.

## Release Alignment Notes

- Las superficies mínimas que deben quedar alineadas en cada ejecución oficial
  son `Cargo.toml`, `packaging/npm/package.json` y
  `packaging/homebrew/sentinel.rb.tpl`.
- El job previo al release debe exponer como outputs, como mínimo, la versión
  final, el tag creado y el commit auditable generado.
- El job de publicación debe tratar cualquier desalineación entre manifiesto,
  repositorio y canales como bloqueo operativo, nunca como warning.

## Phase Strategy

### Phase 0 - Research and Decision Closure

- Cerrar el modelo de persistencia del listado activo de dominios bloqueados.
- Decidir cómo agregar actividad de bloqueo sin introducir una base de datos
  nueva ni ruido operativo innecesario.
- Definir el inventario de superficies versionadas que el job previo debe
  alinear automáticamente.
- Fijar el orden exacto de jobs y compuertas para el workflow de release
  centralizado.
- Confirmar pruebas mínimas para UX de CLI, actividad de bloqueo y consistencia
  de release.

### Phase 1 - Design and Contracts

- Modelar entidades para navegación, dominios bloqueados, actividad de bloqueo,
  solicitud de versión y ejecución de release.
- Definir un contrato visible para la nueva sección interactiva `Ajustes`.
- Definir un contrato operativo para el workflow de versionado y release
  centralizado.
- Preparar quickstart de validación manual y automatizada para ambas áreas del
  feature.
- Actualizar el contexto del agente con la nueva capa de versionado centralizado
  y la ampliación del flujo CLI.

### Phase 2 - Implementation Planning

- Desglosar tareas por:
  - navegación y render del home/ajustes/estado
  - persistencia y validación del blocklist administrable
  - captura y agregación de actividad de bloqueo
  - snapshots, contratos e integraciones de CLI
  - workflow de versionado centralizado
  - scripts de alineación y publicación
  - documentación y fixtures de release

## Validation and Test Coverage

- **CLI UX**:
  - snapshots del home sin `Riesgo`
  - snapshots de `Estado de Sentinel` con tabla recortada y nueva actividad
  - recorridos scriptados hacia `Ajustes` y `Dominios bloqueados`
- **Blocklist management**:
  - agregar dominio válido
  - rechazar dominio inválido o duplicado
  - editar dominio existente
  - eliminar último dominio y mostrar estado vacío
  - persistencia entre reinicios de la app
- **Blocking activity**:
  - agregación correcta de `Bloqueos desde la activación`
  - conteo de dominios únicos bloqueados
  - `Último bloqueo` coherente con el evento más reciente
  - orden estable de `Top dominios bloqueados`
- **Release/versioning**:
  - job previo actualiza todas las superficies previstas
  - se genera commit auditable antes del tag
  - el release job consume exactamente esa versión y tag
  - bloqueo ante desalineación entre repo, artefactos, npm o Homebrew
  - fixtures y contratos cubren reintentos y estados parciales
- **Static validation**:
  - `cargo test`
  - `cargo clippy --all-targets --all-features`

## Operational Risks and Mitigations

- **Sobrescritura accidental del listado activo**: mitigar con escritura atómica,
  normalización/dedupe previa y seed inicial solo cuando el archivo gestionado
  no exista.
- **Regresiones en navegación menu-driven**: mitigar limitando cambios a
  `src/cli` y respaldando con snapshots e integraciones scriptadas.
- **Actividad de bloqueo ruidosa o costosa**: mitigar registrando eventos
  mínimos estructurados y agregando solo lo necesario para las cuatro métricas.
- **Desalineación de versiones en superficies ocultas**: mitigar con un
  inventario explícito de archivos dependientes y una validación post-update
  antes de crear tag y artefactos.
- **Release parcial desde CI**: mitigar separando job de versionado del job de
  deploy, con resumen final y bloqueo preventivo si el estado no es inequívoco.
- **Conflictos por commit automático de versión**: mitigar haciendo que el job
  previo trabaje sobre el HEAD autorizado del workflow y falle si el repositorio
  cambia antes de poder crear el tag.

## Workflow and Documentation Updates

- Sustituir el disparo actual por tags `push` por un workflow oficial con input
  de nueva versión.
- Introducir un job previo de versionado con responsabilidades explícitas:
  actualizar superficies, validar alineación, crear commit y crear tag.
- Mantener un job separado para build/release/deploy que dependa del job previo
  y consuma solo el estado versionado ya validado.
- Actualizar scripts de release para operar con la versión resuelta por el job
  previo y no asumir tags creados localmente.
- Documentar en `README.md` y artefactos de la feature la nueva ruta oficial de
  release para mantenedores.
- Documentar en quickstart la validación esperada tanto de la CLI como del
  pipeline centralizado.
