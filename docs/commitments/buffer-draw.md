# Commitment: buffer-draw

Status: Agreed 2026-09-07.

Requirements: BUF-008, BUF-009, BUF-010, BUF-011, BUF-012, BUF-013

## What it delivers

Drawing on top of buffer-core: integer-exact alpha blending with
backdrop substitution, nested scissor rectangles, wide-cell edge
fills, plain text and single-grapheme drawing, per-cell ANSI escape
emission, and framebuffer compositing with clipping plus cross-pool
cluster resolution. Plain `drawText`/`drawGrapheme` live here;
`drawTextBuffer` (text-view input), `drawImage` (media-image input),
and the renderer diff loop stay with their owners.

## Records

- Specification: `docs/spec/buffer.md` (BUF-008 … BUF-013).
- Code: `src/buffer/draw.rs` (plus an `opacity_stack` field on the
  core struct).
- Mechanism: `.cairn/mechanisms/buffer-draw.md` with runner
  `scripts/buffer-draw.sh`.

## Formats

Rust edition 2024, stable toolchain, no `unsafe`. Each requirement has
a spec-literal unit test (`buffer::draw::blending`,
`buffer::draw::scissor`, `buffer::draw::wide_cells`,
`buffer::draw::ansi_output`, `buffer::draw::composite`,
`buffer::draw::cross_pool_composite`) plus ported vectors in
`tests/buffer_draw.rs` named `req_00n_<slug>`. No new external
dependencies.

## Design deviations

- The composite fast path copies row by row instead of one bulk copy
  when fully aligned; the bytes landed are identical.
- Cross-pool composites re-intern cluster bytes into the destination
  pool (extents preserved, orphans become spaces, foreign ids never
  land). Same-pool composites copy ids raw like the reference.
- `cell_ansi` assumes truecolor/256-color capabilities with the
  reference default and transparent-bg rules; quantized fallback
  arrives with render-terminal.
- Placement compositing copies clipped geometry; decoded pixels stay
  source-owned until the media commitment (which also ports the
  `drawImage` placement-merge vectors).
- Failing-allocator halves of the no-allocate copies have no
  stable-Rust equivalent; the copy assertions remain.

## Tests that prove it

- `cargo test -p suprtui buffer::draw::blending` (etc. per requirement).
- `cargo test -p suprtui --test buffer_draw` — all 18 pass.
- `sh scripts/buffer-draw.sh` — prints `cairn: BUF-008: pass`
  through `cairn: BUF-013: pass`.
- `cargo clippy -p suprtui --all-targets` — no warnings.
- `cargo fmt -p suprtui -- --check` — clean.

## Done-when

Every requirement in this commitment passes its mechanism with the
spec-literal unit tests plus the ported suites, and clippy is clean.

## Mechanism demonstration (SPEC-022)

Demonstrated 2026-09-07 with controlled faults, one per requirement:
opaque-source change fails exactly BUF-008, scissor-nest removal
exactly BUF-009, EOL-fill removal exactly BUF-010, style-order swap
exactly BUF-011, first-row fast-path copy exactly BUF-012 (a
slow-path-only fault passed everything, showing the fast path has
independent coverage), translation bypass exactly BUF-013. Each
fault was a source edit, reverted in the same session, not a setup
error.
