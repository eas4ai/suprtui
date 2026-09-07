#!/bin/sh
# Mechanism for UNI-008 (commitment `uni-pool`).
# Runs the spec-literal unit falsifier plus the integration suite filter
# and reports the per-requirement result. A missing result line stays
# unverified.
set -u
if cargo test -q -p suprtui uni::pool_isolation >/dev/null 2>&1 \
    && cargo test -q -p suprtui --test uni_pool "req_008" >/dev/null 2>&1; then
    echo "cairn: UNI-008: pass"
else
    echo "cairn: UNI-008: fail"
fi
