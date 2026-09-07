# Mechanism: render-stdout

command: sh scripts/render-stdout.sh
inputs:
  - scripts/render-stdout.sh
  - src/lib.rs
  - src/render.rs
  - tests/render_stdout.rs
  - Cargo.toml
  - Cargo.lock
requirements:
  - REN-012
results: per-requirement
