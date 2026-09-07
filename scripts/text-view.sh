#!/bin/sh
# Mechanism for TXT-005 … TXT-010 (commitment `text-view`).
# Runs the spec-literal unit test plus the integration filter per
# requirement and reports per-requirement results. Missing result lines
# stay unverified.
# NOTE: tests/text_view.rs lands with the implementation; until then
# the integration target is missing and every line reports fail. That
# red is truthful, not a runner defect.
set -u
for pair in "TXT-005 wrapping req_005" "TXT-006 selection req_006" "TXT-007 cursor_cluster_safety req_007" "TXT-008 highlight_refs req_008" "TXT-009 syntax_style req_009" "TXT-010 offset_bounds req_010"; do
    set -- $pair
    if cargo test -q -p suprtui --lib "text::$2" >/dev/null 2>&1 \
        && cargo test -q -p suprtui --test text_view "$3" >/dev/null 2>&1; then
        echo "cairn: $1: pass"
    else
        echo "cairn: $1: fail"
    fi
done
