# Mechanism: media-image

command: sh scripts/media-image.sh
inputs:
  - scripts/media-image.sh
  - src/lib.rs
  - src/media.rs
  - tests/media_image.rs
  - Cargo.toml
  - Cargo.lock
requirements:
  - MED-001
  - MED-002
results: per-requirement
