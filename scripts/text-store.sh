#!/bin/sh
# Mechanism for TXT-001 … TXT-004 (commitment `text-store`).
# Runs the spec-literal unit test plus the integration filter per
# requirement and reports per-requirement results. Missing result lines
# stay unverified.
# NOTE: tests/text_store.rs lands with the implementation; until then
# the integration target is missing and every line reports fail. That
# red is truthful, not a runner defect.
set -u
for pair in "TXT-001 edits req_001" "TXT-002 span_tracking req_002" "TXT-003 undo_redo req_003" "TXT-004 view_dirty_tracking req_004"; do
    set -- $pair
    if cargo test -q -p suprtui --lib "text::$2" >/dev/null 2>&1 \
        && cargo test -q -p suprtui --test text_store "$3" >/dev/null 2>&1; then
        echo "cairn: $1: pass"
    else
        echo "cairn: $1: fail"
    fi
done
