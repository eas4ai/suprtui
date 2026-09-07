# Commitment: sys-clipboard

Status: Agreed 2026-09-07.

Requirements: SYS-003

## What it delivers

Clipboard read, write, and clear over the reference `host.zig`
lifecycle (start, poll, result, destroy) with cancellation, in a
new `src/clipboard.rs` module (the spec's `sys::clipboard_lifecycle`
mechanism maps to `clipboard::lifecycle` unit tests plus the
`sys_clipboard` integration suite, as with `sys-core`). The state
machine is drivable by an injected backend trait so tests run
headless against a loopback backend; platform backends plug into
the same seam with `Unsupported` where no display server exists.
No new external dependencies, no `unsafe`.

## Records

- Specification: `docs/spec/sys.md` (SYS-003).
- Code: `src/clipboard.rs`.
- Mechanism: `.cairn/mechanisms/sys-clipboard.md` with runner
  `scripts/sys-clipboard.sh`.

## Formats

Rust edition 2024, stable toolchain, no `unsafe`. Vectors in
`tests/sys_clipboard.rs` named `req_003_<slug>`. No new external
dependencies.

## Tests that prove it

- `cargo test -p suprtui --test sys_clipboard` — all pass.
- `sh scripts/sys-clipboard.sh` — prints `cairn: SYS-003: pass`.
- `cargo clippy -p suprtui --all-targets` — no warnings.
- `cargo fmt -p suprtui -- --check` — clean.

## Done-when

Read, write, and clear each complete start-poll-result-destroy
against the loopback backend; polling never reports completion
early; cancelling suppresses the result; the full suite runs with
no display server.

## Mechanism demonstration (SPEC-022)

To be demonstrated with controlled faults, each a source edit
reverted in the same session, not a setup error.
