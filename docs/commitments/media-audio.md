# Commitment: media-audio

Status: Agreed 2026-09-07.

Requirements: MED-003, MED-004, MED-005, MED-006, MED-007

## What it delivers

A deterministic audio engine over `src/audio.rs` with a built-in
test backend (no hardware, no miniaudio): fallible engine lifecycle
where every operation returns a `Result` and stopping an unstarted
engine or mixer is safe; PCM streams that account every written
sample, apply volume/pan/group at mix time, and reject writes after
close; device listing with names and default flags plus
range-checked selection; capture that delivers only recorded frames
in order with matching statistics; and id-addressed sounds with
voices, groups, clamped volumes, mixed output, and tap capture. No
new external dependency: the engine is pure Rust by construction.

## Records

- Specification: `docs/spec/media.md` (MED-003 … MED-007).
- Code: `src/audio.rs`.
- Mechanism: `.cairn/mechanisms/media-audio.md` with runner
  `scripts/media-audio.sh`.

## Formats

Rust edition 2024, stable toolchain, no `unsafe`. Each requirement
has vectors in `tests/media_audio.rs` named `req_00n_<slug>`. No
new external dependencies.

## Tests that prove it

- `cargo test -p suprtui --test media_audio` — all pass.
- `sh scripts/media-audio.sh` — prints `cairn: MED-003: pass`
  through `cairn: MED-007: pass`.
- `cargo clippy -p suprtui --all-targets` — no warnings.
- `cargo fmt -p suprtui -- --check` — clean.

## Done-when

Every requirement passes its mechanism with the ported suites, and
clippy is clean.

## Mechanism demonstration (SPEC-022)

Demonstrated 2026-09-07 with controlled faults, each a source edit
reverted in the same session, not a setup error: a dropped device
gate fails exactly MED-003, a weakened close gate fails exactly
MED-004, an off-by-one selection bound fails exactly MED-005, a
drop-oldest overflow policy fails exactly MED-006, and sound id
reuse fails exactly MED-007. The first MED-005 probe passed and
exposed a missing boundary assertion, added before re-probing.
