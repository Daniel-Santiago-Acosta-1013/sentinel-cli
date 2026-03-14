# Data Model: Interactive Sentinel CLI

## InteractiveSession

**Purpose**: Represents the current guided terminal session and the user's
position inside the Sentinel experience.

**Fields**:
- `screen`: current visible screen or menu
- `selected_action`: current highlighted action
- `status_summary`: short summary shown to the user
- `risk_level`: `normal | warning | critical`
- `pending_confirmation`: whether the session is waiting for confirmation
- `last_message`: most recent result or guidance message

**Validation Rules**:
- `screen` MUST map to a supported interactive screen.
- `risk_level=critical` MUST surface a recovery or exit option before allowing
  sensitive actions to continue.

## SafetyCheckResult

**Purpose**: Captures the outcome of a preflight validation before activation or
other sensitive operations.

**Fields**:
- `check_id`: stable identifier for the validation run
- `timestamp`: execution time
- `status`: `pass | warn | fail`
- `connectivity_ready`: whether Sentinel can proceed safely
- `recovery_ready`: whether rollback is available
- `issues`: list of user-visible issues found
- `recommended_action`: next action shown to the user

**Validation Rules**:
- `status=fail` MUST block unsafe activation.
- `recovery_ready` MUST be true before protection can be enabled.

## BlocklistBundle

**Purpose**: Represents the bundled default list of ad domains used by Sentinel.

**Fields**:
- `bundle_id`: stable bundle identifier
- `version`: shipped blocklist version
- `domain_count`: total domains available for blocking
- `source_label`: curated source description for internal tracking
- `loaded_at`: time when Sentinel loaded the bundle
- `integrity_state`: whether the bundle passed validation

**Validation Rules**:
- `domain_count` MUST be greater than zero for a valid release bundle.
- `integrity_state` MUST pass before the bundle is used in protection mode.

## InstallationState

**Purpose**: Represents Sentinel's installed presence on the device and what
the installer should do next.

**Fields**:
- `installed`: whether Sentinel is currently present
- `path_entry`: detected executable path
- `installed_version`: current installed version, if any
- `target_version`: version selected for installation
- `action`: `install | update | reinstall | none`
- `last_install_result`: summary of the most recent installer outcome

**Validation Rules**:
- `action=update` MUST only appear when an installed version exists and differs
  from the target version.
- `action=reinstall` MUST be selectable when the install is invalid, incomplete,
  or explicitly forced by the installer flow.

## NetworkRecoverySnapshot

**Purpose**: Represents the saved network state Sentinel can use to restore
connectivity after a failed or interrupted operation.

**Fields**:
- `snapshot_id`: stable identifier
- `captured_at`: creation time
- `affected_services`: list of affected network services
- `dns_state`: restorable DNS-related state
- `routing_state`: restorable routing-related state
- `restorable`: whether the snapshot passed validation

**Validation Rules**:
- `restorable` MUST be true before Sentinel may proceed with activation.
- `affected_services` MUST not be empty when protection changes live network
  state.
