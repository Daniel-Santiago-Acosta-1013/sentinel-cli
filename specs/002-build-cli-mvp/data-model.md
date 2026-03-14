# Data Model: Sentinel CLI MVP

## ProtectionState

**Purpose**: Represents the current health and lifecycle state of the
protection system.

**Fields**:
- `mode`: `inactive | activating | active | degraded | recovering | failed`
- `started_at`: optional timestamp of the current active session
- `last_transition_at`: timestamp of the most recent state change
- `last_error_code`: optional stable error identifier
- `snapshot_id`: optional reference to the active `NetworkSnapshot`
- `active_rule_count`: integer count of loaded rule entries
- `active_exclusion_count`: integer count of active exclusions

**Validation Rules**:
- `mode` MUST be one of the supported lifecycle values.
- `active_rule_count` and `active_exclusion_count` MUST be zero or greater.
- `snapshot_id` MUST exist when `mode` is `active`, `degraded`, or `recovering`.

**State Transitions**:
- `inactive -> activating -> active`
- `activating -> failed`
- `active -> degraded`
- `active -> recovering -> inactive`
- `failed -> recovering -> inactive`

## NetworkSnapshot

**Purpose**: Captures the minimum recoverable network state required to restore
connectivity after changes.

**Fields**:
- `id`: stable snapshot identifier
- `captured_at`: timestamp of capture
- `interface_scope`: list of affected interfaces
- `dns_settings`: captured DNS values relevant to restoration
- `routing_settings`: captured route values relevant to restoration
- `proxy_settings`: captured proxy-related values if present
- `restorable`: boolean indicating whether the snapshot passed validation

**Validation Rules**:
- `interface_scope` MUST include at least one affected interface for an active
  protection session.
- `restorable` MUST be `true` before activation can complete.

**Relationships**:
- One `NetworkSnapshot` MAY be referenced by many `OperationEvent` entries.
- One `ProtectionState` MAY reference one active `NetworkSnapshot`.

## RuleEntry

**Purpose**: Defines an MVP rule used to block or exclude a destination.

**Fields**:
- `id`: stable rule identifier
- `kind`: `block | allow`
- `match_type`: `domain | suffix`
- `value`: normalized rule value
- `source`: `built_in | user`
- `enabled`: boolean flag
- `created_at`: timestamp
- `updated_at`: timestamp

**Validation Rules**:
- `value` MUST be normalized before persistence.
- `kind=allow` MUST take precedence over matching `block` rules.
- Duplicate enabled rules with the same `kind`, `match_type`, and `value` MUST
  be rejected or collapsed.

**Relationships**:
- Many `RuleEntry` records influence one `ProtectionState`.

## OperationEvent

**Purpose**: Records a user-visible action or runtime event relevant to status,
debugging, and recovery.

**Fields**:
- `id`: stable event identifier
- `timestamp`: creation time
- `type`: `activate | disable | status | allow | remove_allow | error | recover`
- `severity`: `info | warning | error`
- `message`: short human-readable summary
- `detail_code`: optional stable machine-readable detail
- `related_snapshot_id`: optional reference to `NetworkSnapshot`
- `related_rule_id`: optional reference to `RuleEntry`

**Validation Rules**:
- `message` MUST remain concise enough for terminal display.
- `severity=error` MUST include either `detail_code` or an explicit recovery
  path in the rendered output.

**Relationships**:
- Many `OperationEvent` records MAY reference one `NetworkSnapshot`.
- Many `OperationEvent` records MAY reference one `RuleEntry`.
