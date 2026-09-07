# Commitment: term-core

Status: Agreed 2026-09-07.

Requirements: TRM-001, TRM-002, TRM-003, TRM-004, TRM-005, TRM-008, TRM-009, TRM-010

## What it delivers

The terminal domain over `src/term.rs`: capability flags defaulting
off with response-proven enables, mode restore covering every enabled
mode, balanced alternate-screen tracking, Kitty and legacy key
encoding with modifier rejection, SGR mouse encoding, environment
exposure, clipboard sequence budgets, and image-protocol selection.
The embedded terminal (TRM-006, TRM-007) stays with `term-embedded`,
which owns the virtual-terminal engine.

## Records

- Specification: `docs/spec/term.md` (TRM-001 … TRM-005, TRM-008,
  TRM-009, TRM-010).
- Code: `src/term.rs`.
- Mechanism: `.cairn/mechanisms/term-core.md` with runner
  `scripts/term-core.sh`.

## Formats

Rust edition 2024, stable toolchain, no `unsafe`. Each requirement has
a spec-literal unit test (`term::capability_defaults`,
`term::mode_restore`, `term::alt_screen_balance`,
`term::key_encoding`, `term::mouse_encoding`, `term::environment`,
`term::clipboard_budget`, `term::image_protocol`) plus ported vectors
in `tests/term_core.rs` named `req_00n_<slug>`. No new external
dependencies.

## Tests that prove it

- `cargo test -p suprtui term::capability_defaults` (etc. per
  requirement).
- `cargo test -p suprtui --test term_core` — all pass.
- `sh scripts/term-core.sh` — prints `cairn: TRM-001: pass`
  through `cairn: TRM-005: pass` and `cairn: TRM-008: pass` through
  `cairn: TRM-010: pass`.
- `cargo clippy -p suprtui --all-targets` — no warnings.
- `cargo fmt -p suprtui -- --check` — clean.

## Done-when

Every requirement in this commitment passes its mechanism with the
spec-literal unit tests plus the ported suites, and clippy is clean.

## Mechanism demonstration (SPEC-022)

To be demonstrated with controlled faults, one per requirement, each
a source edit reverted in the same session, not a setup error.
