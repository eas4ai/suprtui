#!/bin/sh
# Mechanism for SYS-010, SYS-011 (commitment `sys-small`). Runs the
# integration filter per requirement and reports per-requirement
# results. Missing result lines stay unverified.
# NOTE: the emitter/file-logger sections of src/sys.rs and
# tests/sys_small.rs land with the implementation; until then the
# targets are missing and every line reports fail. That red is
# truthful, not a runner defect.
set -u
for pair in "SYS-010 emitter req_010" "SYS-011 file_logger req_011"; do
    set -- $pair
    if cargo test -q -p suprtui --lib "sys::$2" >/dev/null 2>&1 \
        && cargo test -q -p suprtui --test sys_small "$3" >/dev/null 2>&1; then
        echo "cairn: $1: pass"
    else
        echo "cairn: $1: fail"
    fi
done
