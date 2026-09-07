# Mechanism: uni-pool

command: sh scripts/uni-pool.sh
inputs:
  - scripts/uni-pool.sh
  - src/uni/mod.rs
  - src/uni/pool.rs
  - src/uni/segments.rs
  - tests/uni_pool.rs
  - Cargo.toml
  - Cargo.lock
requirements:
  - UNI-008
results: per-requirement
