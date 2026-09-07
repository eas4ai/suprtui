# Mechanism: buffer-core

command: sh scripts/buffer-core.sh
inputs:
  - scripts/buffer-core.sh
  - src/ansi.rs
  - src/buffer/mod.rs
  - src/lib.rs
  - src/link.rs
  - tests/buffer_core.rs
  - tests/link_pool.rs
  - Cargo.toml
  - Cargo.lock
requirements:
  - BUF-001
  - BUF-002
  - BUF-003
  - BUF-004
  - BUF-005
  - BUF-006
  - BUF-007
results: per-requirement
