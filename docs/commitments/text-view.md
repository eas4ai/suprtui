# Commitment: text-view

Status: Agreed 2026-09-07.

Requirements: TXT-005, TXT-006, TXT-007, TXT-008, TXT-009, TXT-010

## What it delivers

The view domain over text storage: width and wrap-mode virtual
lines with cluster-safe breaks, word and line selection with exact
extraction, cluster-safe cursor edits, reference-keyed highlights,
syntax styles that rewrite matching spans, and total offset-bounds
safety. Storage (edits, spans, undo, dirt) stays closed under
`text-store`.

## Records

- Specification: `docs/spec/text.md` (TXT-005 … TXT-010).
- Code: `src/text.rs` (extends the text-store module).
- Mechanism: `.cairn/mechanisms/text-view.md` with runner
  `scripts/text-view.sh`.

## Formats

Rust edition 2024, stable toolchain, no `unsafe`. Each requirement has
a spec-literal unit test (`text::wrapping`, `text::selection`,
`text::cursor_cluster_safety`, `text::highlight_refs`,
`text::syntax_style`, `text::offset_bounds`) plus ported vectors in
`tests/text_view.rs` named `req_00n_<slug>`. No new external
dependencies.

## Tests that prove it

- `cargo test -p suprtui text::wrapping` (etc. per requirement).
- `cargo test -p suprtui --test text_view` — all pass.
- `sh scripts/text-view.sh` — prints `cairn: TXT-005: pass`
  through `cairn: TXT-010: pass`.
- `cargo clippy -p suprtui --all-targets` — no warnings.
- `cargo fmt -p suprtui -- --check` — clean.

## Done-when

Every requirement in this commitment passes its mechanism with the
spec-literal unit tests plus the ported suites, and clippy is clean.

## Mechanism demonstration (SPEC-022)

To be demonstrated with controlled faults, one per requirement, each
a source edit reverted in the same session, not a setup error.
