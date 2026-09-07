# Commitment: render-stdout

Status: Agreed 2026-09-07.

Requirements: REN-012

## What it delivers

A `StdoutBackend<W: io::Write>` implementing the render `Backend`
trait: frame bytes commit in order, direct bytes flush immediately,
failed frames drop their partial bytes, writer failures report
`Failed` without panicking, and empty frames write nothing.
Production construction writes to real stdout; tests drive it over
`Cursor<Vec<u8>>` plus a failing writer. A parity vector renders
the same frame sequence through memory and stdout backends and
demands identical streams. New code in `src/render.rs` with
vectors in `tests/render_stdout.rs`. No new dependencies, no
`unsafe`.

## Records

- Specification: `docs/spec/render.md` (REN-012).
- Code: `src/render.rs`.
- Mechanism: `.cairn/mechanisms/render-stdout.md` with runner
  `scripts/render-stdout.sh`.

## Formats

Rust edition 2024, stable toolchain, no `unsafe`. Vectors in
`tests/render_stdout.rs` named `req_012_<slug>`. No new external
dependencies.

## Tests that prove it

- `cargo test -p suprtui --test render_stdout` — all pass.
- `sh scripts/render-stdout.sh` — prints `cairn: REN-012: pass`.
- `cargo clippy -p suprtui --all-targets` — no warnings.
- `cargo fmt -p suprtui -- --check` — clean.

## Done-when

Committed bytes arrive in order, failures report `Failed`, and the
memory/stdout parity vector passes.

## Mechanism demonstration (SPEC-022)

Demonstrated 2026-09-07 with controlled faults, each a source edit
reverted in the same session, not a setup error: dropping direct
bytes fails `stdout_backend_order`, and swallowing writer errors as
`Ok` fails `stdout_backend_broken_writer`.
