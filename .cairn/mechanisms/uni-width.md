# Mechanism: uni-width

command: sh scripts/uni-width.sh
inputs:
  - scripts/uni-width.sh
  - src/uni.rs
  - Cargo.toml
  - Cargo.lock
requirements:
  - UNI-001
  - UNI-002
  - UNI-003
results: per-requirement
