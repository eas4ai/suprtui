#!/bin/sh
# Mechanism for SYS-003 (commitment `sys-clipboard`). Runs the
# clipboard lifecycle suite and reports the per-requirement result.
# Missing result lines stay unverified.
# NOTE: src/clipboard.rs and tests/sys_clipboard.rs land with the
# implementation; until then the targets are missing and the line
# reports fail. That red is truthful, not a runner defect.
set -u
if cargo test -q -p suprtui --lib "clipboard::lifecycle" >/dev/null 2>&1 \
    && cargo test -q -p suprtui --test sys_clipboard >/dev/null 2>&1; then
    echo "cairn: SYS-003: pass"
else
    echo "cairn: SYS-003: fail"
fi
