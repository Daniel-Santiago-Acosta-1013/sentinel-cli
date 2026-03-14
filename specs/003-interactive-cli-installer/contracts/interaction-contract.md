# Interaction Contract: Interactive Sentinel CLI

## Entry Contract

- Users start Sentinel by calling `sentinel` directly from the terminal.
- The normal user flow does not depend on flags.
- Sentinel opens into a primary interactive screen with only core actions.

## Core Interactive Actions

The first release exposes exactly these user-facing actions from the interactive
flow:

- Open Sentinel home screen
- Run safety checks
- Enable protection
- Disable protection
- View status and health
- Recover network
- View install/update state
- Exit Sentinel

## Interaction Expectations

- Navigation MUST remain keyboard-driven and predictable.
- The initial screen MUST make the current protection state visible.
- Any risky action MUST require an explicit confirmation inside the interactive
  session.
- Sentinel MUST explain blocked actions in plain language and show the next safe
  step.
- The interface MUST prioritize readability over density and avoid exposing
  advanced maintenance actions in the first screen.

## Screen Output Requirements

- Home screen MUST show product state, risk state, and core actions.
- Safety screen MUST show check status and whether activation can proceed.
- Recovery screen MUST explain what will be restored before confirmation.
- Status screen MUST show whether Sentinel is inactive, active, or degraded.

## Failure Contract

- If a safety check fails, activation MUST not proceed.
- If Sentinel detects degraded health, it MUST surface recovery before allowing
  further risky actions.
- If interactive rendering fails, the session MUST exit with a clear message
  rather than making hidden background changes.
