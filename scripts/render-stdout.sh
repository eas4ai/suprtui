#!/bin/sh
# Mechanism for REN-012 (commitment `render-stdout`). Runs the
# stdout-backend suite and reports the per-requirement result.
# Missing result lines stay unverified.
# NOTE: StdoutBackend and tests/render_stdout.rs land with the
# implementation; until then the targets are missing and the line
# reports fail. That red is truthful, not a runner defect.
set -u
if cargo test -q -p suprtui --lib "render::stdout_backend" >/dev/null 2>&1 \
    && cargo test -q -p suprtui --test render_stdout >/dev/null 2>&1; then
    echo "cairn: REN-012: pass"
else
    echo "cairn: REN-012: fail"
fi
