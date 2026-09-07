# Commitment: layout-engine

Status: Agreed 2026-09-07.

Requirements: LAY-001, LAY-002, LAY-003, LAY-004

## What it delivers

Flex layout over `src/layout.rs` on the decided `taffy` engine (per
`docs/decisions/flex-layout-uses-the-taffy-crate-with-yoga-classical-style-defaults.md`):
row/column directions, grow/shrink, wrapping, justification and
alignment, margins, padding, gaps, min/max sizes; classical Yoga
defaults (column direction, point scale one) in the wrapper's style
constructor; caller-owned boxed measure targets bound at most one
per node, consulted during layout and never after clearing; and
absolute computed positions and sizes for every node including
nested children.

## Records

- Specification: `docs/spec/layout.md` (LAY-001 … LAY-004).
- Code: `src/layout.rs`.
- Mechanism: `.cairn/mechanisms/layout-engine.md` with runner
  `scripts/layout-engine.sh`.

## Formats

Rust edition 2024, stable toolchain, no `unsafe`. Each requirement
has ported vectors in `tests/layout_engine.rs` named `req_00n_<slug>`.
One new external dependency: `taffy` (pure Rust, no build scripts).

## Tests that prove it

- `cargo test -p suprtui --test layout_engine` — all pass.
- `sh scripts/layout-engine.sh` — prints `cairn: LAY-001: pass`
  through `cairn: LAY-004: pass`.
- `cargo clippy -p suprtui --all-targets` — no warnings.
- `cargo fmt -p suprtui -- --check` — clean.

## Done-when

Every requirement passes its mechanism with the ported suites, and
clippy is clean.

## Mechanism demonstration (SPEC-022)

To be demonstrated with controlled faults, one per requirement, each
a source edit reverted in the same session, not a setup error.
