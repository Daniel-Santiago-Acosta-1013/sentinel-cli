# Sentinel CLI

Sentinel CLI is a terminal-first DNS blocker focused on safe system recovery.
It enables a local DNS runtime, switches the system DNS to that local resolver,
and keeps a restorable snapshot before changing the network state.

This npm package publishes the `sentinel` executable as a prebuilt release
artifact produced by the project's release automation.

## Install

```bash
npm install -g @daniel_santiago/sentinel-cli
```

After installation, run:

```bash
sentinel
```

## What Sentinel does

- Starts a local DNS runtime to filter blocked domains.
- Guides activation and recovery through an interactive terminal flow.
- Captures a snapshot before changing DNS settings.
- Surfaces explicit degraded states when Sentinel cannot guarantee a safe change.
- Keeps local state, snapshots, and recovery metadata for traceability.

## Current scope

- Primary real-network target: macOS
- Primary interface: interactive CLI in a TTY
- Safe development mode: `SENTINEL_FAKE_PLATFORM=1`

## Notes for npm users

- This package is a CLI binary package, not a JavaScript library.
- Releases are expected to be published only by the official GitHub Actions
  release workflow after tag, version, and artifact consistency checks pass.
- If you want to inspect or contribute to the source project, see the GitHub
  repository below.

## Repository

- Source: <https://github.com/Daniel-Santiago-Acosta-1013/sentinel-cli>
- Homebrew install: `brew install Daniel-Santiago-Acosta-1013/homebrew-tap/sentinel`

## License

Apache-2.0
