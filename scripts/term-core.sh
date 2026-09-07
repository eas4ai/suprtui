#!/bin/sh
# Mechanism for TRM-001 … TRM-005, TRM-008 … TRM-010 (commitment
# `term-core`). Runs the spec-literal unit test plus the integration
# filter per requirement and reports per-requirement results. Missing
# result lines stay unverified.
# NOTE: tests/term_core.rs lands with the implementation; until then
# the integration target is missing and every line reports fail. That
# red is truthful, not a runner defect.
set -u
for pair in "TRM-001 capability_defaults req_001" "TRM-002 mode_restore req_002" "TRM-003 alt_screen_balance req_003" "TRM-004 key_encoding req_004" "TRM-005 mouse_encoding req_005" "TRM-008 environment req_008" "TRM-009 clipboard_budget req_009" "TRM-010 image_protocol req_010"; do
    set -- $pair
    if cargo test -q -p suprtui --lib "term::$2" >/dev/null 2>&1 \
        && cargo test -q -p suprtui --test term_core "$3" >/dev/null 2>&1; then
        echo "cairn: $1: pass"
    else
        echo "cairn: $1: fail"
    fi
done
