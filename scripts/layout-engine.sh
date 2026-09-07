#!/bin/sh
# Mechanism for LAY-001 … LAY-004 (commitment `layout-engine`). Runs
# the integration filter per requirement and reports per-requirement
# results. Missing result lines stay unverified.
# NOTE: tests/layout_engine.rs lands with the implementation; until
# then the integration target is missing and every line reports fail.
# That red is truthful, not a runner defect.
set -u
for pair in "LAY-001 flex req_001" "LAY-002 config_defaults req_002" "LAY-003 measure_targets req_003" "LAY-004 computed_positions req_004"; do
    set -- $pair
    if cargo test -q -p suprtui --lib "layout::$2" >/dev/null 2>&1 \
        && cargo test -q -p suprtui --test layout_engine "$3" >/dev/null 2>&1; then
        echo "cairn: $1: pass"
    else
        echo "cairn: $1: fail"
    fi
done
