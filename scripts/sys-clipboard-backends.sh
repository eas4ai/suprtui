#!/bin/sh
# Mechanism for SYS-009 (commitment `sys-clipboard-backends`).
# Runs the platform-backend suite and reports the per-requirement
# result. Missing result lines stay unverified.
# NOTE: the platform section of src/clipboard.rs and
# tests/sys_clipboard_backends.rs land with the implementation;
# until then the targets are missing and the line reports fail.
# That red is truthful, not a runner defect.
set -u
if cargo test -q -p suprtui --lib "clipboard::platform" >/dev/null 2>&1 \
    && cargo test -q -p suprtui --test sys_clipboard_backends >/dev/null 2>&1; then
    echo "cairn: SYS-009: pass"
else
    echo "cairn: SYS-009: fail"
fi
