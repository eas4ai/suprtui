# Mechanism: uni-width

command: sh scripts/uni-width.sh
inputs:
  - scripts/uni-width.sh
  - src/uni.rs
  - src/uni_tables.rs
  - tests/uni_ported.rs
  - tests/width_map_runs.inc
  - tests/fixtures/multilingual.txt
  - Cargo.toml
  - Cargo.lock
requirements:
  - UNI-001
  - UNI-002
  - UNI-003
results: per-requirement
