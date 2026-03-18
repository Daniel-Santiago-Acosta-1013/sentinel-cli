# Research: Automatización Segura de Releases

## Decision 1: La release se autoriza solo por igualdad exacta entre tag y `main@HEAD`

**Decision**: El workflow debe activarse por tag estable, pero solo puede pasar
la compuerta de release si el SHA resuelto por ese tag es exactamente igual al
HEAD vigente de `main`. La comprobación debe repetirse al inicio del workflow y
justo antes de la primera publicación externa.

**Rationale**: El requisito central del feature no es “publicar desde un tag”,
es “publicar desde el último cambio autorizado de `main`”. Revalidar evita
falsos positivos cuando `main` avanza entre la creación del tag y la ejecución
real del pipeline.

**Alternatives considered**:

- Aceptar cualquier tag alcanzable desde `main`. Rechazado porque permitiría
  publicar commits viejos.
- Aceptar tags creados en cualquier rama si luego se mergean. Rechazado porque
  no garantiza que el tag represente el HEAD vigente.
- Validar solo una vez al inicio. Rechazado porque deja una ventana de carrera
  si `main` avanza antes de la publicación.

## Decision 2: `Cargo.toml` es la versión oficial y el manifiesto de artefactos es la evidencia canónica

**Decision**: La versión oficial del proyecto será la declarada en `Cargo.toml`;
el tag debe coincidir exactamente con esa versión; y el pipeline debe generar un
manifiesto de artefactos con checksums que sirva como evidencia canónica para
GitHub Release, npm y Homebrew.

**Rationale**: Sentinel ya expone su versión desde el paquete Rust. Reusar esa
fuente evita duplicidad y permite que el binario, el tag y la distribución
externa hablen el mismo idioma. El manifiesto agrega reproducibilidad y
trazabilidad entre canales.

**Alternatives considered**:

- Tratar el tag como única verdad de versión. Rechazado porque permitiría
  divergencia con el código fuente.
- Mantener versiones independientes para npm o Homebrew. Rechazado por romper
  consistencia e idempotencia.
- Confiar solo en nombres de archivos sin manifiesto. Rechazado por insuficiente
  para auditoría post-release.

## Decision 3: GitHub Release materializa primero el conjunto canónico y los canales externos derivan de él

**Decision**: El pipeline debe construir artefactos reproducibles, materializar
un GitHub Release con esos artefactos y su manifiesto, y solo después publicar
npm y Homebrew usando la misma evidencia aprobada.

**Rationale**: Separar “build canónico” de “distribución por canal” establece un
ancla auditable. npm y Homebrew no deben reconstruir por separado ni depender de
estados implícitos; deben consumir el mismo release ya comprobado.

**Alternatives considered**:

- Publicar npm primero y luego derivar Homebrew. Rechazado porque deja menos
  trazabilidad central y complica auditoría.
- Construir artefactos por canal. Rechazado por riesgo de diferencias entre
  outputs.
- Omitir GitHub Release y publicar directo a los canales. Rechazado por debilitar
  reproducibilidad y evidencias compartidas.

## Decision 4: Los reintentos se resuelven por inspección de estado, no por re-publicación ciega

**Decision**: Antes de publicar o reintentar, el pipeline debe inspeccionar el
estado observable de GitHub Release, npm y Homebrew para clasificar la versión
como `blocked`, `materialized`, `partial` o candidata a continuación segura.

**Rationale**: Un fallo parcial no puede resolverse suponiendo que nada ocurrió.
La seguridad operativa exige observar el mundo externo y decidir según evidencia
real, no según la intención previa del workflow.

**Alternatives considered**:

- Reintentar siempre el pipeline completo. Rechazado porque puede duplicar
  publicaciones o empeorar una desalineación.
- Depender solo del estado interno de GitHub Actions. Rechazado porque no refleja
  necesariamente el estado real en los canales externos.
- Permitir overwrite si la versión ya existe. Rechazado porque rompe
  inmutabilidad y auditabilidad.

## Decision 5: Homebrew se actualiza desde plantilla local y se sincroniza hacia un tap oficial

**Decision**: El repositorio principal debe mantener una plantilla local de
fórmula Homebrew y el workflow debe renderizarla con versión, URL y checksum del
artefacto canónico para sincronizarla hacia un tap oficial controlado por el
proyecto.

**Rationale**: Mantener la plantilla en este repo preserva revisión y
trazabilidad junto al código de release. El tap es un canal de distribución, no
la fuente de verdad. Esta separación reduce acoplamiento y deja auditable el
origen de cada fórmula publicada.

**Alternatives considered**:

- Editar manualmente la fórmula en otro repositorio. Rechazado por alto riesgo
  operativo y poca trazabilidad.
- Tratar el tap como fuente de verdad. Rechazado porque la release debe derivar
  del commit autorizado y del manifiesto canónico, no del canal secundario.
- Publicar a Homebrew Core como primer paso. Rechazado porque introduce latencia
  y revisión externa fuera del control necesario para este feature.

## Decision 6: El pipeline debe fallar en modo seguro cuando no pueda verificar hechos críticos

**Decision**: Cualquier imposibilidad para resolver `main@HEAD`, leer el commit
del tag, verificar versión, comprobar checksums o inspeccionar npm/Homebrew debe
cerrar el intento como bloqueado o parcial, nunca como completado.

**Rationale**: La automatización de release es una operación de confianza. Si el
pipeline no puede demostrar que todo coincide, la opción segura es detener o
dejar el intento en estado degradado con evidencia suficiente para intervención
humana.

**Alternatives considered**:

- Asumir éxito parcial mientras no haya error explícito. Rechazado por crear
  falsos positivos.
- Omitir post-verificación si la publicación respondió sin error. Rechazado
  porque la respuesta del canal no garantiza alineación final.
