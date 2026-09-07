# Mechanism: render-core

command: sh scripts/render-core.sh
inputs:
  - scripts/render-core.sh
  - src/buffer/mod.rs
  - src/lib.rs
  - Cargo.toml
  - Cargo.lock
requirements:
  - REN-001
  - REN-002
  - REN-003
  - REN-004
  - REN-005
  - REN-011
results: per-requirement
