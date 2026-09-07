# Commitment: sys-small

Status: Agreed 2026-09-07.

Requirements: SYS-010, SYS-011

## What it delivers

The two cheap reference modules, both in `src/sys.rs` with
vectors in `tests/sys_small.rs`: a multi-listener `EventEmitter`
keyed by event id with attach/detach/fire-in-order semantics
(SYS-010), and a `FileLogger` appending level-gated lines to a
caller-chosen path without panicking on I/O errors (SYS-011).
No new dependencies, no `unsafe`.

## Records

- Specification: `docs/spec/sys.md` (SYS-010, SYS-011).
- Code: `src/sys.rs`.
- Mechanism: `.cairn/mechanisms/sys-small.md` with runner
  `scripts/sys-small.sh`.

## Formats

Rust edition 2024, stable toolchain, no `unsafe`. Vectors in
`tests/sys_small.rs` named `req_01n_<slug>`. No new external
dependencies.

## Tests that prove it

- `cargo test -p suprtui --test sys_small` — all pass.
- `sh scripts/sys-small.sh` — prints `cairn: SYS-010: pass`
  and `cairn: SYS-011: pass`.
- `cargo clippy -p suprtui --all-targets` — no warnings.
- `cargo fmt -p suprtui -- --check` — clean.

## Done-when

Emission order, detach silence, level gating, and I/O-error
survival each pass their mechanism with ported vectors, and
clippy is clean.

## Mechanism demonstration (SPEC-022)

Demonstrated 2026-09-07 with controlled faults, each a source edit
reverted in the same session, not a setup error: a detach noop
that keeps firing fails `req_010_detach_never_fires_again`, and a
removed level gate fails `req_011_level_gating`.
