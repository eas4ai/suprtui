# Mechanism: lint

command: sh scripts/lint.sh
inputs:
  - scripts/lint.sh
  - src/
  - tests/
requirements:
  - LINT-001
results: per-requirement
