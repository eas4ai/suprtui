# Mechanism: buffer-draw

command: sh scripts/buffer-draw.sh
inputs:
  - scripts/buffer-draw.sh
  - src/ansi.rs
  - src/buffer/draw.rs
  - src/buffer/mod.rs
  - src/lib.rs
  - src/link.rs
  - tests/buffer_draw.rs
  - Cargo.toml
  - Cargo.lock
requirements:
  - BUF-008
  - BUF-009
  - BUF-010
  - BUF-011
  - BUF-012
  - BUF-013
results: per-requirement
