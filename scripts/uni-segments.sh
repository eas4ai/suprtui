#!/bin/sh
# Mechanism for UNI-004, UNI-005, UNI-006, UNI-007, UNI-009 (commitment `uni-segments`).
# Runs one integration-test filter per requirement and reports
# per-requirement results. Missing result lines stay unverified.
set -u
i=4
for req in UNI-004 UNI-005 UNI-006 UNI-007; do
    if cargo test -q -p suprtui --test uni_segments "req_00${i}" >/dev/null 2>&1; then
        echo "cairn: $req: pass"
    else
        echo "cairn: $req: fail"
    fi
    i=$((i + 1))
done
if cargo test -q -p suprtui --test uni_segments "req_009" >/dev/null 2>&1; then
    echo "cairn: UNI-009: pass"
else
    echo "cairn: UNI-009: fail"
fi
