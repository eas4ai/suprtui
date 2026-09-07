# Mechanism: media-audio

command: sh scripts/media-audio.sh
inputs:
  - scripts/media-audio.sh
  - src/lib.rs
  - src/audio.rs
  - tests/media_audio.rs
  - Cargo.toml
  - Cargo.lock
requirements:
  - MED-003
  - MED-004
  - MED-005
  - MED-006
  - MED-007
results: per-requirement
