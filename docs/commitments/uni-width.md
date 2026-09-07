# Commitment: uni-width

Status: Agreed 2026-09-07.

Requirements: UNI-001, UNI-002, UNI-003

## What it delivers

The `suprtui` crate compiles as a library with a `uni` module that
offers all four width methods per call and reports verified widths
for ASCII, CJK blocks, combining marks, format controls, and
reference-listed symbols. Full Unicode property tables and the
complete ported vector suites land here too, before Done.

## Records

- Specification: `docs/spec/uni.md` (UNI-001 … UNI-003).
- Code: `src/uni.rs`, `src/lib.rs`, `Cargo.toml`.
- Mechanism: `.cairn/mechanisms/uni-width.md` with runner
  `scripts/uni-width.sh`.

## Formats

Rust edition 2024, stable toolchain. Tests live in `src/uni.rs`
under `#[cfg(test)]`, one test per requirement named
`req_00n`. No external dependencies.

## Tests that prove it

- `cargo test -p suprtui --lib uni::` — all width tests pass.
- `sh scripts/uni-width.sh` — prints `cairn: UNI-001:
  pass`, `cairn: UNI-002: pass`, `cairn: UNI-003: pass`.
- `cargo clippy -p suprtui --all-targets` — no warnings.
- Full ported vector suites from `utf8_wcwidth_test.zig`,
  `utf8_test.zig`, and the reference range tables pass (loop work).

## Done-when

Every requirement in this commitment passes its mechanism with the
full ported vectors, clippy is clean, and the seed default in
`cell_width` is replaced by complete Unicode property data.

## Mechanism demonstration (SPEC-022)

Demonstrated 2026-09-07 with a controlled fault: removing `0x231A`
from the verified-wide list in `src/uni.rs` produced `cairn:
UNI-001: pass`, `cairn: UNI-002: fail`, `cairn: UNI-003: fail`,
catching the violation in exactly the requirements that cover the
symbol. Restoring the entry returned all three to pass, and clippy
reported no warnings. The fault was a source edit, reverted in the
same session, not a setup error.
