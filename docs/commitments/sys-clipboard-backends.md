# Commitment: sys-clipboard-backends

Status: Agreed 2026-09-07.

Requirements: SYS-009

## What it delivers

Real platform clipboard backends behind the `sys-clipboard`
`Backend` seam: a `ProcessBackend` that reads and writes through
helper processes (`wl-copy`/`wl-paste`, `xclip`/`xsel`,
`pbcopy`/`pbpaste`, `clip` plus PowerShell `Get-Clipboard`),
environment routing ported from the reference `linux.zig`
(`Environment::detect`: Wayland vs X11 vs WSL), and a
`CommandRunner` trait with real (`std::process`) and scripted-fake
implementations so routing and command construction run headless.
New code in `src/clipboard.rs` (platform section) with vectors in
`tests/sys_clipboard_backends.rs`. No new external dependencies, no
`unsafe`. Raw-protocol X11/Wayland stays out: that duplicates the
helpers over thousands of lines, and Wayland fd-passing is
impossible in std-only safe Rust.

## Records

- Specification: `docs/spec/sys.md` (SYS-009).
- Code: `src/clipboard.rs`.
- Mechanism: `.cairn/mechanisms/sys-clipboard-backends.md` with
  runner `scripts/sys-clipboard-backends.sh`.

## Formats

Rust edition 2024, stable toolchain, no `unsafe`. Vectors in
`tests/sys_clipboard_backends.rs` named `req_009_<slug>`. No new
external dependencies.

## Tests that prove it

- `cargo test -p suprtui --test sys_clipboard_backends` — all pass.
- `sh scripts/sys-clipboard-backends.sh` — prints
  `cairn: SYS-009: pass`.
- `cargo clippy -p suprtui --all-targets` — no warnings.
- `cargo fmt -p suprtui -- --check` — clean.

## Done-when

Every platform route builds the right command through the fake
runner; Wayland/X11/WSL detection matches the reference truth
table; helper failure maps to `Failed`, missing helpers to
`Unsupported`; the suite runs with no display server.

## Mechanism demonstration (SPEC-022)

Demonstrated 2026-09-07 with controlled faults, each a source edit
reverted in the same session, not a setup error: routing a Wayland
display to X11 helpers fails `req_009_route_matrix`, reporting a
missing helper as `Failed` instead of `Unsupported` fails
`req_009_missing_helper_is_unsupported`, and dropping the xsel
fallback fails `req_009_xclip_falls_back_to_xsel`. A live roundtrip
(write, read, clear) through auto-detect against the machine's X
server passed and is recorded here, not in the suite, which stays
headless.
