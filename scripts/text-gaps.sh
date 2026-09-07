#!/bin/sh
# Mechanism for TXT-011 … TXT-015 (commitment `text-gaps`). Runs
# the integration filter per requirement and reports
# per-requirement results. Missing result lines stay unverified.
# NOTE: tests/text_gaps.rs lands with the implementation; until
# then the integration target is missing and every line reports
# fail. That red is truthful, not a runner defect.
set -u
for pair in "TXT-011 gesture_selection req_011" "TXT-012 viewport_selection req_012" "TXT-013 iterators req_013" "TXT-014 wrap_cache req_014" "TXT-015 editor_view req_015"; do
    set -- $pair
    if cargo test -q -p suprtui --lib "text::$2" >/dev/null 2>&1 \
        && cargo test -q -p suprtui --test text_gaps "$3" >/dev/null 2>&1; then
        echo "cairn: $1: pass"
    else
        echo "cairn: $1: fail"
    fi
done
