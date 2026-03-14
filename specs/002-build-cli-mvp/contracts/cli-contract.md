# CLI Contract: Sentinel CLI MVP

## Command Surface

The MVP exposes exactly these user-facing command groups:

- `sentinel enable`
- `sentinel disable`
- `sentinel status`
- `sentinel allow add <domain>`
- `sentinel allow remove <domain>`
- `sentinel rules list`
- `sentinel recover`
- `sentinel events [--limit <n>]`

## Global Options

- `--json`: machine-readable output for automation and scripting
- `--verbose`: include extended operational detail in human-readable output
- `--no-color`: disable terminal color styling

No other global flags are part of the MVP contract.

## Output Contract

### Human-readable mode

- Default output MUST be concise and action-oriented.
- Success responses MUST include the resulting protection state.
- Warning and error responses MUST include the safest next action.
- Tabular output MAY be used for `status`, `rules list`, and `events`.

### JSON mode

Every command in JSON mode MUST return an object with at least:

```json
{
  "ok": true,
  "command": "status",
  "state": "active",
  "message": "Protection is active"
}
```

Additional command-specific fields MAY appear, but these keys form the stable
minimum contract for automation.

## Exit Codes

- `0`: command completed successfully
- `1`: user-correctable operational failure
- `2`: invalid usage or validation failure
- `3`: recovery required or partial protection state detected

## Command Expectations

### `enable`

- MUST create or validate a recoverable network snapshot before activation
- MUST fail safely if recoverability cannot be guaranteed
- MUST report resulting state clearly

### `disable`

- MUST stop protection and restore the last safe snapshot when available
- MUST remain idempotent when protection is already inactive

### `status`

- MUST report `inactive`, `active`, or `degraded`
- MUST include active rule count and exclusion count

### `allow add` / `allow remove`

- MUST validate the provided domain input
- MUST keep allow rules scoped to supported MVP rule types only
- MUST explain whether a change was applied, ignored, or rejected

### `rules list`

- MUST show only rules relevant to the current MVP scope
- MUST distinguish block rules from allow rules

### `recover`

- MUST attempt restoration using the most recent valid snapshot
- MUST communicate success, failure, or next manual step

### `events`

- MUST return recent user-visible operational events
- MUST support bounded result sets via `--limit`
