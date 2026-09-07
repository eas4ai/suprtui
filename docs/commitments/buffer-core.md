# Commitment: buffer-core

Status: Agreed 2026-09-07.

Requirements: BUF-001, BUF-002, BUF-003, BUF-004, BUF-005, BUF-006, BUF-007

## What it delivers

Cell-buffer storage on top of the uni layers: the packed RGBA color
model with intents, the 16-entry palette and 256-color cube, the cell
struct with style flags and link-id packing, grid bounds discipline,
fallible resize that always clears, and clear with full tracker and
placement cleanup. Cell writes go through the reference paths (`set`
with span cleanup, `set_raw` without tracking, `sync_cell` without
span cleanup) with grapheme and link trackers wired to caller-owned
pools.

## Records

- Specification: `docs/spec/buffer.md` (BUF-001 … BUF-007).
- Code: `src/buffer/mod.rs`, `src/ansi.rs` (color core),
  `src/link.rs` (link pool and tracker).
- Mechanism: `.cairn/mechanisms/buffer-core.md` with runner
  `scripts/buffer-core.sh`.

## Formats

Rust edition 2024, stable toolchain, no `unsafe`. Each requirement has
a spec-literal unit test (`buffer::color_model`, `buffer::palette`,
`buffer::cell_roundtrip`, `buffer::bounds`, `buffer::resize_errors`,
`buffer::resize_clears`, `buffer::clear`) plus ported vectors in
`tests/buffer_core.rs` named `req_00n_<slug>`. `tests/link_pool.rs`
covers the link dependency. No new external dependencies.

## Design deviations

- Layout is `src/buffer/mod.rs` (with `src/ansi.rs`, `src/link.rs`
  beside it) rather than the spec's `src/buffer.rs`, matching the
  evolved `src/uni/mod.rs` layout; draw submodules attach in
  `buffer-draw` with no renames.
- Pools are caller-owned `Rc<RefCell<..>>` handles. A missing link
  pool becomes a fresh owned pool, never the reference global one.
- Image placements keep geometry plus handle; decoded pixels stay
  with the media commitment, and `clear` drops placement geometry.
- Text-drawing cases use explicit `set` loops; `drawText` itself
  arrives with the drawing commitment.
- `LinkTracker::add_cell_ref` records a defined zero for rejected
  ids instead of the reference's garbage count (same map presence);
  no ported vector covers that path.

## Tests that prove it

- `cargo test -p suprtui buffer::color_model` (etc. per requirement).
- `cargo test -p suprtui --test buffer_core` — all 18 pass.
- `cargo test -p suprtui --test link_pool` — all 11 pass.
- `sh scripts/buffer-core.sh` — prints `cairn: BUF-001: pass`
  through `cairn: BUF-007: pass`.
- `cargo clippy -p suprtui --all-targets` — no warnings.
- `cargo fmt -p suprtui -- --check` — clean.

## Done-when

Every requirement in this commitment passes its mechanism with the
spec-literal unit tests plus the ported suites, and clippy is clean.

## Mechanism demonstration (SPEC-022)

Demonstrated 2026-09-07 with controlled faults, one per requirement:
intent-bit flip fails exactly BUF-001, palette entry change exactly
BUF-002, link-bit drop exactly BUF-003, bounds widening exactly
BUF-004, zero-resize accept exactly BUF-005, skipped resize-clear
exactly BUF-006, skipped tracker-clear exactly BUF-007. Each fault
was a source edit, reverted in the same session, not a setup error.
