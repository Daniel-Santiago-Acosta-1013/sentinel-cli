# Install Script Contract: Sentinel CLI

## Entry Contract

- Users execute one official script to install, update, or reinstall Sentinel.
- The script MUST work without requiring users to manually move binaries into
  the `PATH`.

## Required Behaviors

- Detect whether Sentinel is already installed.
- Determine whether the correct action is install, update, or reinstall.
- Place the executable where it can be invoked as `sentinel`.
- Report the final installed version and resulting executable path.
- Stop with a clear explanation if integrity validation or replacement fails.

## Update and Reinstall Semantics

- `install`: no previous working Sentinel installation is found.
- `update`: a working installation exists and a newer target version is
  available.
- `reinstall`: an installation exists but is missing, damaged, incomplete, or
  explicitly needs replacement.

## Safety Requirements

- The script MUST not replace the working executable until the new artifact is
  fully available and validated.
- The script MUST communicate whether manual privileges are required.
- The script MUST leave the previous working installation intact if the new
  installation step fails before final replacement.
