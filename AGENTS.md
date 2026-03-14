# sentinel-cli Development Guidelines

Auto-generated from all feature plans. Last updated: 2026-03-14

## Active Technologies
- Rust 1.90.0 (edition 2024) plus POSIX shell for installation workflows + `ratatui`, `crossterm`, `tokio`, `hickory-proto`, `serde`, `serde_json`, `toml`, `miette`, `thiserror`, `tracing`, `tracing-subscriber`, `directories`, `uuid`, shell tooling via `sh`, `curl`, `tar`, and `shasum` (003-interactive-cli-installer)
- Local filesystem state under application support directories (`TOML` for user configuration, `JSON`/`JSONL` for runtime state, health checks, install metadata, and recovery events) (003-interactive-cli-installer)

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
- 003-interactive-cli-installer: Added Rust 1.90.0 (edition 2024) plus POSIX shell for installation workflows + `ratatui`, `crossterm`, `tokio`, `hickory-proto`, `serde`, `serde_json`, `toml`, `miette`, `thiserror`, `tracing`, `tracing-subscriber`, `directories`, `uuid`, shell tooling via `sh`, `curl`, `tar`, and `shasum`

- 002-build-cli-mvp: Added Rust 1.90.0 (edition 2024) + `clap`, `tokio`, `tun`, `hickory-proto`, `serde`, `serde_json`, `toml`, `miette`, `thiserror`, `tracing`, `tracing-subscriber`, `comfy-table`, `directories`

<!-- MANUAL ADDITIONS START -->
<!-- MANUAL ADDITIONS END -->
