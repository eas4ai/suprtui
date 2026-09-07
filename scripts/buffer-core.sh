#!/bin/sh
# Mechanism for BUF-001 … BUF-007 (commitment `buffer-core`).
# Runs the spec-literal unit test plus the integration filter per
# requirement and reports per-requirement results. Missing result lines
# stay unverified.
set -u
n=1
for req in BUF-001 BUF-002 BUF-003 BUF-004 BUF-005 BUF-006 BUF-007; do
    unit=""; suite=""
    case $n in
        1) unit="buffer::color_model"; suite="req_001" ;;
        2) unit="buffer::palette"; suite="req_002" ;;
        3) unit="buffer::cell_roundtrip"; suite="req_003" ;;
        4) unit="buffer::bounds"; suite="req_004" ;;
        5) unit="buffer::resize_errors"; suite="req_005" ;;
        6) unit="buffer::resize_clears"; suite="req_006" ;;
        7) unit="buffer::clear"; suite="req_007" ;;
    esac
    if cargo test -q -p suprtui --lib "$unit" >/dev/null 2>&1 \
        && cargo test -q -p suprtui --test buffer_core "$suite" >/dev/null 2>&1; then
        echo "cairn: $req: pass"
    else
        echo "cairn: $req: fail"
    fi
    n=$((n + 1))
done
