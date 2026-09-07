#!/bin/sh
# Mechanism for MED-003 … MED-007 (commitment `media-audio`). Runs
# the integration filter per requirement and reports per-requirement
# results. Missing result lines stay unverified.
# NOTE: tests/media_audio.rs lands with the implementation; until
# then the integration target is missing and every line reports fail.
# That red is truthful, not a runner defect.
set -u
for pair in "MED-003 engine_lifecycle req_003" "MED-004 streams req_004" "MED-005 devices req_005" "MED-006 capture req_006" "MED-007 sound_ids req_007"; do
    set -- $pair
    if cargo test -q -p suprtui --lib "audio::$2" >/dev/null 2>&1 \
        && cargo test -q -p suprtui --test media_audio "$3" >/dev/null 2>&1; then
        echo "cairn: $1: pass"
    else
        echo "cairn: $1: fail"
    fi
done
