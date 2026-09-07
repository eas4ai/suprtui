# Mechanism: render-terminal

command: sh scripts/render-terminal.sh
inputs:
  - scripts/render-terminal.sh
  - src/buffer/mod.rs
  - src/lib.rs
  - src/render.rs
  - Cargo.toml
  - Cargo.lock
requirements:
  - REN-006
  - REN-007
  - REN-008
  - REN-009
  - REN-010
results: per-requirement
