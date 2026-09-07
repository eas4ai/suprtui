# Commitment: render-terminal

Status: Agreed 2026-09-07.

Requirements: REN-006, REN-007, REN-008, REN-009, REN-010

## What it delivers

Terminal rendering on top of the render core: threaded and
single-threaded byte parity, setup and shutdown lifecycle sequences,
hit testing with scissor, render offsets with footer surfaces, and
image fallback materialization. The core (double buffering, diff
publish, skip, cursor, rollback, memory backend, stats) stays closed
under `render-core`.

## Records

- Specification: `docs/spec/render.md` (REN-006 … REN-010).
- Code: `src/render.rs` (extends the render-core module).
- Mechanism: `.cairn/mechanisms/render-terminal.md` with runner
  `scripts/render-terminal.sh`.

## Formats

Rust edition 2024, stable toolchain, no `unsafe`. Each requirement has
a spec-literal unit test (`render::thread_parity`,
`render::lifecycle_sequences`, `render::hit_grid`,
`render::split_offset`, `render::image_fallback`) plus ported vectors
in `tests/render_terminal.rs` named `req_00n_<slug>`. No new external
dependencies.

## Tests that prove it

- `cargo test -p suprtui render::thread_parity` (etc. per requirement).
- `cargo test -p suprtui --test render_terminal` — all pass.
- `sh scripts/render-terminal.sh` — prints `cairn: REN-006: pass`
  through `cairn: REN-010: pass`.
- `cargo clippy -p suprtui --all-targets` — no warnings.
- `cargo fmt -p suprtui -- --check` — clean.

## Done-when

Every requirement in this commitment passes its mechanism with the
spec-literal unit tests plus the ported suites, and clippy is clean.

## Mechanism demonstration (SPEC-022)

Demonstrated 2026-09-07 with controlled faults, each a source edit
reverted in the same session, not a setup error: dropped threaded
bytes fail exactly REN-006, a kept alternate screen exactly REN-007,
a staged-grid hit read exactly REN-008, an ignored offset exactly
REN-009, and cleared fallback cells exactly REN-010.
