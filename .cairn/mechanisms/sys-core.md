# Mechanism: sys-core

command: sh scripts/sys-core.sh
inputs:
  - scripts/sys-core.sh
  - src/lib.rs
  - src/sys.rs
  - tests/sys_core.rs
  - Cargo.toml
  - Cargo.lock
requirements:
  - SYS-001
  - SYS-002
  - SYS-004
  - SYS-005
  - SYS-006
  - SYS-007
  - SYS-008
results: per-requirement
