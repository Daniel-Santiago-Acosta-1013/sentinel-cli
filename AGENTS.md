# sentinel-cli Development Guidelines

Auto-generated from all feature plans. Last updated: 2026-03-19

## Active Technologies
- Rust 1.90.0 (edition 2024) plus POSIX shell for installation workflows + `ratatui`, `crossterm`, `tokio`, `hickory-proto`, `serde`, `serde_json`, `toml`, `miette`, `thiserror`, `tracing`, `tracing-subscriber`, `directories`, `uuid`, shell tooling via `sh`, `curl`, `tar`, and `shasum` (003-interactive-cli-installer)
- Local filesystem state under application support directories (`TOML` for user configuration, `JSON`/`JSONL` for runtime state, health checks, install metadata, and recovery events) (003-interactive-cli-installer)
- Rust 1.90.0 (edition 2024) + `crossterm`, `comfy-table`, `tokio`, `serde`, `serde_json`, `toml`, `miette`, `thiserror`, `tracing`, `tracing-subscriber`, `directories`, `uuid`, `chrono`, `hickory-proto` (004-menu-cli-network-safety)
- Archivos locales bajo application support (`TOML` para configuración y `JSON`/`JSONL` para estado, snapshots, instalación y eventos) (004-menu-cli-network-safety)
- Rust 1.90.0 (edition 2024) + POSIX shell para automatización de releases, GitHub Actions, `git`, `cargo`, `tar`, `shasum`, `curl`, `npm` CLI y sincronización de tap de Homebrew (007-gate-release-publishing)
- Archivos del repositorio para distribución (`Cargo.toml`, `packaging/`, manifiestos de artefactos, checksums y metadatos de GitHub Release) (007-gate-release-publishing)
- Rust 1.90.0 (edition 2024) para el binario CLI y POSIX + `crossterm`, `comfy-table`, `tokio`, `serde`, (008-settings-release-alignment)
- Archivos locales bajo el directorio de soporte de la aplicación (008-settings-release-alignment)

- Rust 1.90.0 (edition 2024) + `clap`, `tokio`, `tun`, `hickory-proto`, `serde`, `serde_json`, `toml`, `miette`, `thiserror`, `tracing`, `tracing-subscriber`, `comfy-table`, `directories` (002-build-cli-mvp)

## Project Structure

```text
src/
tests/
```

## Commands

cargo test [ONLY COMMANDS FOR ACTIVE TECHNOLOGIES][ONLY COMMANDS FOR ACTIVE TECHNOLOGIES] cargo clippy

## Code Style

Rust 1.90.0 (edition 2024): Follow standard conventions

## Recent Changes
- 008-settings-release-alignment: Added Rust 1.90.0 (edition 2024) para el binario CLI y POSIX + `crossterm`, `comfy-table`, `tokio`, `serde`,
- 007-gate-release-publishing: Added Rust 1.90.0 (edition 2024) + POSIX shell para automatización de releases con GitHub Actions, npm y Homebrew
- 006-menu-cli-redesign: Added Rust 1.90.0 (edition 2024) + `crossterm`, `comfy-table`, `tokio`, `serde`,


<!-- MANUAL ADDITIONS START -->
<!-- MANUAL ADDITIONS END -->
