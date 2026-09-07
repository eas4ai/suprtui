#!/bin/sh
# Mechanism for BUF-008 … BUF-013 (commitment `buffer-draw`).
# Runs the spec-literal unit test plus the integration filter per
# requirement and reports per-requirement results. Missing result lines
# stay unverified.
set -u
for pair in "BUF-008 blending req_008" "BUF-009 scissor req_009" "BUF-010 wide_cells req_010" "BUF-011 ansi_output req_011" "BUF-012 composite req_012_" "BUF-013 cross_pool_composite req_013_"; do
    set -- $pair
    if cargo test -q -p suprtui --lib "buffer::draw::$2" >/dev/null 2>&1 \
        && cargo test -q -p suprtui --test buffer_draw "$3" >/dev/null 2>&1; then
        echo "cairn: $1: pass"
    else
        echo "cairn: $1: fail"
    fi
done
