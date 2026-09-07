# Commitment: uni-segments

Status: Agreed 2026-09-07.

Requirements: UNI-004, UNI-005, UNI-006, UNI-007, UNI-009

## What it delivers

Segmentation and navigation on top of the uni-width measurement
layer: a grapheme-break iterator with cluster bounds, wrap and
position search by width with tab stops, line-break scanning with
CRLF as one break, checked byte-level UTF-8 decoding that errors
instead of panicking, and the reference cell-character packing with
its predicates.

## Records

- Specification: `docs/spec/uni.md` (UNI-004 … UNI-007, UNI-009).
- Code: `src/uni/mod.rs`, `src/uni/segments.rs`,
  `src/uni_tables.rs`.
- Mechanism: `.cairn/mechanisms/uni-segments.md` with runner
  `scripts/uni-segments.sh`.
- Golden extractor: `scripts/extract-utf8-goldens.py` regenerates
  `tests/golden_tables.inc` (15 line-break, 11 tab-stop, 24
  layout-break rows) from the reference golden arrays with a
  per-array completeness assert.

## Formats

Rust edition 2024, stable toolchain. Ported vectors live in
`tests/uni_segments.rs`, one `#[test]` per ported case named
`req_00n_<slug>` plus hand-ported builder and table tests. No new
external dependencies beyond the uni-width set.

## Tests that prove it

- `cargo test -p suprtui --test uni_segments` — all 81 pass
  (65 ported vectors plus 16 hand-ported golden/builder/random/
  corpus/kind/packing tests).
- `sh scripts/uni-segments.sh` — prints `cairn: UNI-004: pass`
  through `cairn: UNI-007: pass` and `cairn: UNI-009: pass`.
- `cargo clippy -p suprtui --all-targets` — no warnings.
- `cargo fmt -p suprtui -- --check` — clean.

## Done-when

Every requirement in this commitment passes its mechanism with the
full ported suites (wrap, position, previous-grapheme, line-break,
tab-stop, decode, and packing vectors), and clippy is clean.

## Mechanism demonstration (SPEC-022)

Demonstrated 2026-09-07 with controlled faults: breaking the
no_zwj ZWJ split fails exactly UNI-004, shifting a tab stop fails
exactly UNI-005, splitting CRLF fails exactly UNI-006, accepting a
truncated sequence fails exactly UNI-007, and flipping a packing
flag bit fails exactly UNI-009. Each fault was a source edit,
reverted in the same session, not a setup error.
