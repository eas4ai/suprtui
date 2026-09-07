# Commitment: sys-core

Status: Agreed 2026-09-07.

Requirements: SYS-001, SYS-002, SYS-004, SYS-005, SYS-006, SYS-007, SYS-008

## What it delivers

System foundations over `src/sys.rs` (plus the existing
caller-owned pools): Rust-ownership object identity with no
process-global handle registry and per-instance isolation; an
ordered event bus whose destroyed sinks stop delivery; the link
pool's 512-byte interned URLs with reference counting and
pool-scoped ids; an ordered span feed with contiguous atomic writes
and exactly-once drains; split-scrollback accounting that clamps
the render offset and follows the reference formulas; level-gated
logging through a caller-supplied sink that drops silently without
one; and crate-wide error discipline where every public fallible
function returns a typed `Result`. SYS-003 (clipboard lifecycle)
stays with `sys-clipboard`. No new external dependencies.

## Records

- Specification: `docs/spec/sys.md` (SYS-001, SYS-002, SYS-004 …
  SYS-008).
- Code: `src/sys.rs`.
- Mechanism: `.cairn/mechanisms/sys-core.md` with runner
  `scripts/sys-core.sh`.

## Formats

Rust edition 2024, stable toolchain, no `unsafe`. Each requirement
has vectors in `tests/sys_core.rs` named `req_00n_<slug>`. No new
external dependencies.

## Tests that prove it

- `cargo test -p suprtui --test sys_core` — all pass.
- `sh scripts/sys-core.sh` — prints `cairn: SYS-001: pass`
  through `cairn: SYS-008: pass` (SYS-003 excluded).
- `cargo clippy -p suprtui --all-targets` — no warnings.
- `cargo fmt -p suprtui -- --check` — clean.

## Done-when

Every requirement passes its mechanism with the ported suites, and
clippy is clean.

## Mechanism demonstration (SPEC-022)

To be demonstrated with controlled faults, one per requirement, each
a source edit reverted in the same session, not a setup error.
