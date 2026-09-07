#!/bin/sh
# Mechanism for REN-006 … REN-010 (commitment `render-terminal`).
# Runs the spec-literal unit test plus the integration filter per
# requirement and reports per-requirement results. Missing result lines
# stay unverified.
# NOTE: tests/render_terminal.rs lands with the implementation; until
# then the integration target is missing and every line reports fail.
# That red is truthful, not a runner defect.
set -u
for pair in "REN-006 thread_parity req_006" "REN-007 lifecycle_sequences req_007" "REN-008 hit_grid req_008" "REN-009 split_offset req_009" "REN-010 image_fallback req_010"; do
    set -- $pair
    if cargo test -q -p suprtui --lib "render::$2" >/dev/null 2>&1 \
        && cargo test -q -p suprtui --test render_terminal "$3" >/dev/null 2>&1; then
        echo "cairn: $1: pass"
    else
        echo "cairn: $1: fail"
    fi
done
