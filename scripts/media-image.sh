#!/bin/sh
# Mechanism for MED-001, MED-002 (commitment `media-image`). Runs the
# integration filter per requirement and reports per-requirement
# results. Missing result lines stay unverified.
# NOTE: tests/media_image.rs lands with the implementation; until
# then the integration target is missing and every line reports fail.
# That red is truthful, not a runner defect.
set -u
for pair in "MED-001 decode req_001" "MED-002 image_roundtrip req_002"; do
    set -- $pair
    if cargo test -q -p suprtui --lib "media::$2" >/dev/null 2>&1 \
        && cargo test -q -p suprtui --test media_image "$3" >/dev/null 2>&1; then
        echo "cairn: $1: pass"
    else
        echo "cairn: $1: fail"
    fi
done
