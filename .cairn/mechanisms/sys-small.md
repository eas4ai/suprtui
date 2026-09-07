# Mechanism: sys-small

command: sh scripts/sys-small.sh
inputs:
  - scripts/sys-small.sh
  - src/lib.rs
  - src/sys.rs
  - tests/sys_small.rs
  - Cargo.toml
  - Cargo.lock
requirements:
  - SYS-010
  - SYS-011
results: per-requirement
