# Commitment: text-gaps

Status: Agreed 2026-09-07.

Requirements: TXT-011, TXT-012, TXT-013, TXT-014, TXT-015

## What it delivers

The scoped text-domain remainder, all in `src/text.rs` with
vectors in `tests/text_gaps.rs`: a press/move/release gesture
machine with word/line/cell behaviors over cells (TXT-011);
viewport-coordinate selection with scroll offsets (TXT-012);
grapheme-cluster and line iterators plus offset/coords conversion
(TXT-013); a recompute counter proving the wrap cache invalidates
exactly on content/width/wrap change (TXT-014); and an
`EditorView` unit owning viewport, cursor visibility, and
cell-space selection with logical/visual mapping (TXT-015). No
new dependencies, no `unsafe`. Rope internals stay out pending
the owner's decision on the measured costs.

## Records

- Specification: `docs/spec/text.md` (TXT-011 … TXT-015).
- Code: `src/text.rs`.
- Mechanism: `.cairn/mechanisms/text-gaps.md` with runner
  `scripts/text-gaps.sh`.

## Formats

Rust edition 2024, stable toolchain, no `unsafe`. Vectors in
`tests/text_gaps.rs` named `req_01n_<slug>`. No new external
dependencies.

## Tests that prove it

- `cargo test -p suprtui --test text_gaps` — all pass.
- `sh scripts/text-gaps.sh` — prints `cairn: TXT-011: pass`
  through `cairn: TXT-015: pass`.
- `cargo clippy -p suprtui --all-targets` — no warnings.
- `cargo fmt -p suprtui -- --check` — clean.

## Done-when

Each requirement passes its mechanism with ported reference
vectors (gesture, viewport, iterator, cache, editor-view), and
clippy is clean.

## Mechanism demonstration (SPEC-022)

Demonstrated 2026-09-07 with controlled faults, each a source edit
reverted in the same session, not a setup error: expanding padding
clicks into the line fails `req_011_padding_click_is_zero_width`,
honoring the horizontal offset while wrapping fails
`req_012_wrapping_ignores_horizontal_offset`, and a never-
invalidated wrap cache fails `req_014_edits_invalidate_no_stale_rows`.
