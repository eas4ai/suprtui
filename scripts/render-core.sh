#!/bin/sh
# Mechanism for REN-001 … REN-005, REN-011 (commitment `render-core`).
# Runs the spec-literal unit test plus the integration filter per
# requirement and reports per-requirement results. Missing result lines
# stay unverified.
# NOTE: src/render.rs and tests/render_core.rs land with the
# implementation; until then the integration target is missing and every
# line reports fail. That red is truthful, not a runner defect.
set -u
for pair in "REN-001 frame_publish req_001" "REN-002 unchanged_skips req_002" "REN-003 cursor_tracking req_003" "REN-004 failed_frame_rolls_back req_004" "REN-005 memory_backend_exact req_005" "REN-011 stats_count req_011"; do
    set -- $pair
    if cargo test -q -p suprtui --lib "render::$2" >/dev/null 2>&1 \
        && cargo test -q -p suprtui --test render_core "$3" >/dev/null 2>&1; then
        echo "cairn: $1: pass"
    else
        echo "cairn: $1: fail"
    fi
done
