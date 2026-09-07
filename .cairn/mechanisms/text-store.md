# Mechanism: text-store

command: sh scripts/text-store.sh
inputs:
  - scripts/text-store.sh
  - src/lib.rs
  - src/text.rs
  - Cargo.toml
  - Cargo.lock
requirements:
  - TXT-001
  - TXT-002
  - TXT-003
  - TXT-004
results: per-requirement
