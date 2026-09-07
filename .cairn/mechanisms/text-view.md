# Mechanism: text-view

command: sh scripts/text-view.sh
inputs:
  - scripts/text-view.sh
  - src/lib.rs
  - src/text.rs
  - Cargo.toml
  - Cargo.lock
requirements:
  - TXT-005
  - TXT-006
  - TXT-007
  - TXT-008
  - TXT-009
  - TXT-010
results: per-requirement
