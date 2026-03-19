# Quickstart: Ajustes de Bloqueo y Release Alineado

## Goal

Validar que Sentinel CLI mantiene su línea estética actual mientras incorpora
`Ajustes` con administración completa de `Dominios bloqueados`, mejora la vista
`Estado de Sentinel` con `Actividad de bloqueo`, y centraliza el versionado y
release en un workflow oficial trazable y consistente.

## Prerequisites

- Toolchain de Rust del proyecto instalado.
- Terminal interactiva disponible para validación manual.
- Entorno local con permisos habituales para ejecutar Sentinel en modo de
  transcript o interactivo.
- Acceso a GitHub Actions y secretos de publicación configurados para validar el
  flujo centralizado de release en un entorno controlado.

## Validation Steps

1. Ejecutar la validación base del repositorio:

```bash
cargo test
cargo clippy --all-targets -- -D warnings
```

2. Ejecutar Sentinel en modo transcript o interactivo y verificar el home:

```bash
cargo run
```

- el home no muestra `✓ Riesgo: Normal`
- `Ajustes` aparece como opción principal
- el resto del layout mantiene coherencia visual con el producto actual

3. Recorrer la nueva rama de navegación:

```bash
SENTINEL_SCRIPT="down,down,enter,enter,back,back,exit" cargo run
```

- `Ajustes` abre una vista propia
- `Dominios bloqueados` es accesible desde esa vista
- existe una ruta clara para volver al home

4. Validar la administración del listado activo:

- agregar un dominio válido y confirmar que aparece en el listado
- intentar agregar un dominio inválido o duplicado y confirmar rechazo claro
- editar un dominio existente y verificar que el listado se actualiza
- eliminar un dominio y, si era el último, confirmar el estado vacío

5. Abrir `Estado de Sentinel` y confirmar que:

- la tabla principal no muestra `Riesgo`, `Resumen` ni `Accion sugerida`
- existe la sección `Actividad de bloqueo`
- la nueva tabla contiene solo las cuatro métricas pedidas

6. Forzar o simular actividad de bloqueo suficiente para verificar que:

- `Bloqueos desde la activación` incrementa correctamente
- `Dominios únicos bloqueados` refleja el total único
- `Último bloqueo` apunta al evento más reciente
- `Top dominios bloqueados` se ordena de forma estable

7. Confirmar cobertura automatizada de la UX actualizada:

- snapshots del home y de estado pasan con la nueva salida
- contratos e integraciones de CLI cubren `Ajustes` y `Dominios bloqueados`

8. Validar el workflow oficial de versionado centralizado en un entorno de
prueba:

- iniciar el workflow `Release` con `workflow_dispatch` y un input de nueva
  versión estable
- confirmar que el primer job ejecuta `scripts/release/update_versions.sh`,
  actualiza `Cargo.toml`, `packaging/npm/package.json` y
  `packaging/homebrew/sentinel.rb.tpl`
- confirmar que ese job crea un commit auditable y luego el tag alineado
- confirmar que el segundo job construye y publica desde ese mismo estado
- bloquear el flujo si alguna superficie o canal queda desalineado

9. Verificar consistencia final de release:

- la versión en `Cargo.toml` coincide con la versión final publicada
- el tag creado corresponde a la misma versión
- artefactos, npm y Homebrew muestran exactamente la misma versión
- el resumen final distingue claramente éxito completo, bloqueo o estado parcial

## Expected Outcome

Sentinel conserva una experiencia CLI coherente y limpia, añade control visible
sobre el blocklist activo y muestra actividad de bloqueo relevante sin ruido.
Además, el release deja de depender de tags locales y pasa a un flujo oficial en
CI con versionado auditable y consistencia total entre repositorio, artefactos y
canales.
