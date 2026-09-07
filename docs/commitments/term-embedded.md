# Commitment: term-embedded

Status: Agreed 2026-09-07.

Requirements: TRM-006, TRM-007

## What it delivers

The embedded virtual terminal over `src/term/embedded.rs`: a
caller-owned VT engine (pure-Rust `vte` parser plus a hand-rolled
screen grid, per `docs/decisions/embedded-terminal-uses-pure-rust-vte-parser-plus-hand-rolled-grid-no-zig.md`)
that accepts program output bytes, resizes, scrolls a scrollback
viewport, and tracks a cell selection; composes the visible screen
onto an `OptimizedBuffer` at a requested origin with dirty-row
granularity and clipping; reports cursor state; and queues PTY
responses for exactly-once in-order draining under a 1 MiB cap.
Input-side encoders (key, mouse, paste, focus) are not a TRM
requirement and stay out of scope.

## Records

- Specification: `docs/spec/term.md` (TRM-006, TRM-007).
- Code: `src/term/embedded.rs` (with `src/term.rs` as its parent
  module).
- Mechanism: `.cairn/mechanisms/term-embedded.md` with runner
  `scripts/term-embedded.sh`.

## Formats

Rust edition 2024, stable toolchain, no `unsafe`. Each requirement
has ported vectors in `tests/term_embedded.rs` named `req_00n_<slug>`.
One new external dependency: `vte` (parser only; no build scripts,
no native code).

## Tests that prove it

- `cargo test -p suprtui --test term_embedded` — all pass.
- `sh scripts/term-embedded.sh` — prints `cairn: TRM-006: pass`
  and `cairn: TRM-007: pass`.
- `cargo clippy -p suprtui --all-targets` — no warnings.
- `cargo fmt -p suprtui -- --check` — clean.

## Done-when

Both requirements pass their mechanism with the ported suites, and
clippy is clean.

## Mechanism demonstration (SPEC-022)

To be demonstrated with controlled faults, one per requirement, each
a source edit reverted in the same session, not a setup error.
