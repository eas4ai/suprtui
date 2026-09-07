#!/bin/sh
# Mechanism for UNI-001, UNI-002, UNI-003 (first commitment `uni-width`).
# Runs one integration-test filter per requirement and reports
# per-requirement results. Missing result lines stay unverified.
set -u
i=1
for req in UNI-001 UNI-002 UNI-003; do
    if cargo test -q -p suprtui --test uni_ported "req_00${i}" >/dev/null 2>&1; then
        echo "cairn: $req: pass"
    else
        echo "cairn: $req: fail"
    fi
    i=$((i + 1))
done
