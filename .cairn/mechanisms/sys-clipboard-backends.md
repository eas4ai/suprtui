# Mechanism: sys-clipboard-backends

command: sh scripts/sys-clipboard-backends.sh
inputs:
  - scripts/sys-clipboard-backends.sh
  - src/lib.rs
  - src/clipboard.rs
  - tests/sys_clipboard_backends.rs
  - Cargo.toml
  - Cargo.lock
requirements:
  - SYS-009
results: per-requirement
