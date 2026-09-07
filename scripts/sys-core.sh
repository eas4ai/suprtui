#!/bin/sh
# Mechanism for SYS-001, SYS-002, SYS-004 … SYS-008 (commitment
# `sys-core`; SYS-003 belongs to `sys-clipboard`). Runs the
# integration filter per requirement and reports per-requirement
# results. Missing result lines stay unverified.
# NOTE: tests/sys_core.rs lands with the implementation; until then
# the integration target is missing and every line reports fail.
# That red is truthful, not a runner defect.
set -u
for pair in "SYS-001 instance_isolation req_001" "SYS-002 event_bus req_002" "SYS-004 link_pool req_004" "SYS-005 span_feed req_005" "SYS-006 split_scrollback req_006" "SYS-007 logging req_007" "SYS-008 error_discipline req_008"; do
    set -- $pair
    if cargo test -q -p suprtui --lib "sys::$2" >/dev/null 2>&1 \
        && cargo test -q -p suprtui --test sys_core "$3" >/dev/null 2>&1; then
        echo "cairn: $1: pass"
    else
        echo "cairn: $1: fail"
    fi
done
