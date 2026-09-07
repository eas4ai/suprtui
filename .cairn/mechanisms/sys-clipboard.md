# Mechanism: sys-clipboard

command: sh scripts/sys-clipboard.sh
inputs:
  - scripts/sys-clipboard.sh
  - src/lib.rs
  - src/clipboard.rs
  - tests/sys_clipboard.rs
  - Cargo.toml
  - Cargo.lock
requirements:
  - SYS-003
results: per-requirement
