#!/bin/sh
# Mechanism for TRM-006, TRM-007 (commitment `term-embedded`). Runs the
# integration filter per requirement and reports per-requirement
# results. Missing result lines stay unverified.
# NOTE: tests/term_embedded.rs lands with the implementation; until
# then the integration target is missing and every line reports fail.
# That red is truthful, not a runner defect.
set -u
for pair in "TRM-006 embedded_compose req_006" "TRM-007 response_drain req_007"; do
    set -- $pair
    if cargo test -q -p suprtui --lib "term::$2" >/dev/null 2>&1 \
        && cargo test -q -p suprtui --test term_embedded "$3" >/dev/null 2>&1; then
        echo "cairn: $1: pass"
    else
        echo "cairn: $1: fail"
    fi
done
