# Commitment: render-core

Status: Agreed 2026-09-07.

Requirements: REN-001, REN-002, REN-003, REN-004, REN-005, REN-011

## What it delivers

Frame rendering on top of the buffer domain against a memory
backend: next-buffer drawing with diff publish, unchanged-frame
skip, cursor move/style tracking, failed-frame rollback, exact
memory-backend capture, and frame statistics. Threading parity,
lifecycle sequences, the hit grid, split offsets, and image
fallback stay with `render-terminal`, which owns the terminal
capabilities and image paths they need.

## Records

- Specification: `docs/spec/render.md` (REN-001 … REN-005, REN-011).
- Code: `src/render.rs` (the spec names a single file, not a module
  directory).
- Mechanism: `.cairn/mechanisms/render-core.md` with runner
  `scripts/render-core.sh`.

## Formats

Rust edition 2024, stable toolchain, no `unsafe`. Each requirement has
a spec-literal unit test (`render::frame_publish`,
`render::unchanged_skips`, `render::cursor_tracking`,
`render::failed_frame_rolls_back`, `render::memory_backend_exact`,
`render::stats_count`) plus ported vectors in `tests/render_core.rs`
named `req_00n_<slug>`. No new external dependencies.

## Tests that prove it

- `cargo test -p suprtui render::frame_publish` (etc. per requirement).
- `cargo test -p suprtui --test render_core` — all pass.
- `sh scripts/render-core.sh` — prints `cairn: REN-001: pass`
  through `cairn: REN-005: pass` and `cairn: REN-011: pass`.
- `cargo clippy -p suprtui --all-targets` — no warnings.
- `cargo fmt -p suprtui -- --check` — clean.

## Done-when

Every requirement in this commitment passes its mechanism with the
spec-literal unit tests plus the ported suites, and clippy is clean.

## Mechanism demonstration (SPEC-022)

Demonstrated 2026-09-07 with controlled faults, each a source edit
reverted in the same session, not a setup error. Dropping cell-byte
emission fails REN-001/REN-004/REN-005 jointly (the three requirements
asserting emitted content share one path); clearing the current buffer
on skip fails exactly REN-002; a wrong steady-line code fails exactly
REN-003; a missing forced repaint fails exactly REN-004; recording
empty frames first failed REN-002/REN-003/REN-005 jointly through
phantom frame counts, and the precise replacement (truncating each
committed frame by one byte) fails exactly REN-005; counting skipped
frames fails exactly REN-011.
