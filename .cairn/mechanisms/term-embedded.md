# Mechanism: term-embedded

command: sh scripts/term-embedded.sh
inputs:
  - scripts/term-embedded.sh
  - src/lib.rs
  - src/term/mod.rs
  - src/term/embedded.rs
  - tests/term_embedded.rs
  - Cargo.toml
  - Cargo.lock
requirements:
  - TRM-006
  - TRM-007
results: per-requirement
