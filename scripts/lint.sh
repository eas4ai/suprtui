#!/bin/sh
# Mechanism for LINT-001 (standing spec `docs/spec/quality.md`).
# Clippy over all targets reports no warnings and `cargo fmt`
# is clean. Missing result lines stay unverified.
set -u
if cargo clippy -q -p suprtui --all-targets >/dev/null 2>&1 \
    && cargo fmt -p suprtui -- --check >/dev/null 2>&1; then
    echo "cairn: LINT-001: pass"
else
    echo "cairn: LINT-001: fail"
fi
