# Mechanism: layout-engine

command: sh scripts/layout-engine.sh
inputs:
  - scripts/layout-engine.sh
  - src/lib.rs
  - src/layout.rs
  - tests/layout_engine.rs
  - Cargo.toml
  - Cargo.lock
requirements:
  - LAY-001
  - LAY-002
  - LAY-003
  - LAY-004
results: per-requirement
