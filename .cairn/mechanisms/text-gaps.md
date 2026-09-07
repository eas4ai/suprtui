# Mechanism: text-gaps

command: sh scripts/text-gaps.sh
inputs:
  - scripts/text-gaps.sh
  - src/lib.rs
  - src/text.rs
  - tests/text_gaps.rs
  - Cargo.toml
  - Cargo.lock
requirements:
  - TXT-011
  - TXT-012
  - TXT-013
  - TXT-014
  - TXT-015
results: per-requirement
