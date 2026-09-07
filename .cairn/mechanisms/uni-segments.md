# Mechanism: uni-segments

command: sh scripts/uni-segments.sh
inputs:
  - scripts/uni-segments.sh
  - scripts/extract-utf8-goldens.py
  - src/uni/mod.rs
  - src/uni/segments.rs
  - src/uni_tables.rs
  - tests/uni_segments.rs
  - tests/golden_tables.inc
  - Cargo.toml
  - Cargo.lock
requirements:
  - UNI-004
  - UNI-005
  - UNI-006
  - UNI-007
  - UNI-009
results: per-requirement
