# Commitment: text-store

Status: Agreed 2026-09-07.

Requirements: TXT-001, TXT-002, TXT-003, TXT-004

## What it delivers

Text storage on plain byte-offset `String` content: insert, delete,
and split with plain-text readback, style spans that shift and stretch
with edits, snapshot undo and redo with redo cleared on new edits, and
registered views marked dirty under an advancing epoch. Rope balancing
is an internal detail the spec leaves unobserved. Wrapping, selection,
cursor motion, highlights, syntax styles, and bounds errors stay with
`text-view`, which owns the view domain.

## Records

- Specification: `docs/spec/text.md` (TXT-001 … TXT-004).
- Code: `src/text.rs`.
- Mechanism: `.cairn/mechanisms/text-store.md` with runner
  `scripts/text-store.sh`.

## Formats

Rust edition 2024, stable toolchain, no `unsafe`. Each requirement has
a spec-literal unit test (`text::edits`, `text::span_tracking`,
`text::undo_redo`, `text::view_dirty_tracking`) plus ported vectors in
`tests/text_store.rs` named `req_00n_<slug>`. No new external
dependencies.

## Tests that prove it

- `cargo test -p suprtui text::edits` (etc. per requirement).
- `cargo test -p suprtui --test text_store` — all pass.
- `sh scripts/text-store.sh` — prints `cairn: TXT-001: pass`
  through `cairn: TXT-004: pass`.
- `cargo clippy -p suprtui --all-targets` — no warnings.
- `cargo fmt -p suprtui -- --check` — clean.

## Done-when

Every requirement in this commitment passes its mechanism with the
spec-literal unit tests plus the ported suites, and clippy is clean.

## Mechanism demonstration (SPEC-022)

Demonstrated 2026-09-07 with controlled faults, each a source edit
reverted in the same session, not a setup error: an off-by-one split
first failed TXT-001/TXT-004 jointly by panicking an empty split, and
the precise replacement (split drops the boundary char) fails exactly
TXT-001; unshifted spans fail exactly TXT-002; a retained redo stack
exactly TXT-003; a frozen epoch exactly TXT-004.
