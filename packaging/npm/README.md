# Sentinel CLI

The npm package README source now lives in `docs/npm/README.md`.

The release pipeline copies that document into the staged npm package before
publishing so the npm page stays aligned with the repository.

The official release path is now centralized in GitHub Actions. A dispatch
workflow receives the target version, updates repository version surfaces,
creates the release commit and tag, and only then publishes the npm package
from that aligned state.
