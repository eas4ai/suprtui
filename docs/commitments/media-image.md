# Commitment: media-image

Status: Agreed 2026-09-07.

Requirements: MED-001, MED-002

## What it delivers

Image decoding over `src/media.rs` on the decided `image` crate
(per
`docs/decisions/image-decode-and-png-encode-use-the-image-crate.md`):
PNG, JPEG, GIF, and WebP bytes decode to dimensions with RGBA
pixels; corrupt input returns an error and never panics; cloned
images observe identical pixels; PNG re-encoding roundtrips
dimensions and pixels. Lossy fixtures assert dimensions, opacity,
and near-canonical pixels rather than bit-exact values, since exact
lossy output is decoder-specific. Audio (MED-003 … MED-007) stays
with `media-audio`.

## Records

- Specification: `docs/spec/media.md` (MED-001, MED-002).
- Code: `src/media.rs`.
- Mechanism: `.cairn/mechanisms/media-image.md` with runner
  `scripts/media-image.sh`.

## Formats

Rust edition 2024, stable toolchain, no `unsafe`. Each requirement
has ported vectors in `tests/media_image.rs` named `req_00n_<slug>`.
One new external dependency: `image` (pure Rust, no build scripts).

## Tests that prove it

- `cargo test -p suprtui --test media_image` — all pass.
- `sh scripts/media-image.sh` — prints `cairn: MED-001: pass`
  and `cairn: MED-002: pass`.
- `cargo clippy -p suprtui --all-targets` — no warnings.
- `cargo fmt -p suprtui -- --check` — clean.

## Done-when

Both requirements pass their mechanism with the ported suites, and
clippy is clean.

## Mechanism demonstration (SPEC-022)

To be demonstrated with controlled faults, one per requirement, each
a source edit reverted in the same session, not a setup error.
