# Quickstart: Automatización Segura de Releases

## Goal

Validar que la automatización de releases de Sentinel solo publica desde el
HEAD vigente de `main`, mantiene consistencia exacta de versión entre todos los
canales y deja evidencia auditable ante éxito, bloqueo o fallo parcial.

## Prerequisites

- Toolchain de Rust del proyecto instalado.
- Acceso al repositorio con Git y permisos para crear tags de prueba.
- Entorno de CI con secretos de publicación configurados para npm y el tap
  oficial de Homebrew.
- Entorno de validación aislado o de staging para no contaminar canales
  productivos durante la verificación inicial.

## Validation Steps

1. Ejecutar la validación base del repositorio:

```bash
cargo test
cargo clippy
```

2. Confirmar que el workflow y los scripts esperados existen:

```bash
ls .github/workflows/release.yml
ls scripts/release/
```

3. Confirmar que la versión oficial del proyecto es la esperada en
`Cargo.toml` y en la lógica de versión del binario.

4. Preparar tres escenarios de tags en un entorno controlado:

- un tag estable apuntando exactamente a `main@HEAD`
- un tag estable apuntando a un commit viejo de `main`
- un tag estable apuntando a un commit fuera del HEAD vigente de `main`

5. Ejecutar el workflow de release con el tag correcto y verificar que:

- supera la compuerta de autorización
- genera artefactos y `release-manifest.env` con checksums
- publica el release canónico
- publica npm y Homebrew con la misma versión
- deja estado final `completed`

6. Ejecutar el workflow con los tags incorrectos y verificar que:

- el pipeline se bloquea antes de cualquier publicación externa
- la razón del bloqueo identifica con claridad por qué el tag no representa el
  último cambio autorizado de `main`

7. Forzar un escenario de fallo parcial después de materializar un canal y
confirmar que:

- el resultado final es `partial`
- el resumen final identifica el canal afectado
- el siguiente paso seguro queda explícito para mantenimiento

8. Reintentar una versión ya completada correctamente y verificar que:

- el sistema detecta materialización previa
- no duplica la publicación
- el resultado final es `materialized`

9. Verificar el estado observable post-release:

- GitHub Release muestra versión, commit y checksums esperados
- npm muestra la misma versión autorizada
- Homebrew referencia la misma versión y el mismo artefacto canónico

10. Confirmar la validación automatizada actual del repo:

- `cargo test` pasa con la nueva suite de contratos e integración de release
- `cargo clippy --all-targets --all-features` termina sin warnings

## Expected Outcome

Cada release estable de Sentinel queda alineada con el HEAD vigente de `main`,
usa una sola fuente de verdad para versión y artefactos, y deja evidencia
auditable suficiente para distinguir con precisión entre bloqueo, materialización
previa, parcialidad y completitud.

## Validation Notes

- Validado en esta implementación con `cargo test`.
- Validado en esta implementación con `cargo clippy --all-targets --all-features`.
- La suite automatizada usa `RELEASE_STATE_DIR` para simular GitHub Release,
  npm y Homebrew sin depender de publicación real durante pruebas.
