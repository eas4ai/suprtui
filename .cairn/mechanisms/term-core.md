# Mechanism: term-core

command: sh scripts/term-core.sh
inputs:
  - scripts/term-core.sh
  - src/lib.rs
  - src/term.rs
  - Cargo.toml
  - Cargo.lock
requirements:
  - TRM-001
  - TRM-002
  - TRM-003
  - TRM-004
  - TRM-005
  - TRM-008
  - TRM-009
  - TRM-010
results: per-requirement
