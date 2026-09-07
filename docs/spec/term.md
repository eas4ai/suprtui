# Terminal (TRM)

Status: Agreed 2026-09-07.
Prefix: TRM.

Reference: `reference/opentui-0.5.11/packages/native/src/terminal.zig`,
`reference/opentui-0.5.11/packages/native/src/embedded-terminal/`,
`reference/opentui-0.5.11/packages/native/src/ghostty-vt.zig`, and the
fixtures in
`reference/opentui-0.5.11/packages/native/src/tests/terminal_test.zig`
and `ghostty_vt_test.zig`. The crate exposes this domain as
`src/term.rs`. The virtual-terminal engine inside the embedded
terminal arrives from a Rust crate, decided with a decision record;
this domain specifies its observable behavior, not its source.

## Decided dependency

The virtual-terminal engine is `libghostty-rs`
(`https://github.com/Uzaaft/libghostty-rs/`), the safe Rust bindings
for Ghostty's `libghostty-vt`, pinned to a fixed version in
`Cargo.toml`. Reason: it is the same engine family the reference uses,
so its behavior is the closest possible match for TRM-004 through
TRM-007. Two conditions: the ported TRM vectors MUST pass against it,
or the decision reopens; and its build-time Zig dependency MUST be
contained (prebuilt library via `pkg-config`, or vendored Ghostty
source with an offline build path), because an uncontained Zig
requirement would contradict the keystone. Containment details are
decided with the first commitment that touches the embedded
terminal.

[TRM-001]
All capability flags MUST default to off, and processing a terminal
response MUST enable only the capabilities that response proves.
Falsifier: a fresh terminal claims Kitty graphics, or an unknown
response enables any capability.
Mechanism: `cargo test -p suprtui term::capability_defaults`.

[TRM-002]
Restoring terminal modes MUST disable every mode the session enabled
(mouse, bracketed paste, focus tracking, Kitty keyboard, modified
keys, color-scheme updates).
Falsifier: the emitted restore stream leaves mouse reporting enabled
after the session enabled it.
Mechanism: `cargo test -p suprtui term::mode_restore` against a
memory stream.

[TRM-003]
Exiting the alternate screen MUST balance entering it, and MUST
return to the main screen.
Falsifier: an exit without a matching enter emits a switch sequence,
or the terminal reports the alternate screen still active after
exit.
Mechanism: `cargo test -p suprtui term::alt_screen_balance`.

[TRM-004]
Key encoding MUST translate key identity, modifiers, composing
state, and unshifted codepoint into the Kitty-protocol (or legacy
fallback) bytes of the reference vectors, and MUST reject out-of-range
modifier bits with an error.
Falsifier: any encoded key differs from the ported vector, or invalid
modifier bits encode silently.
Mechanism: `cargo test -p suprtui term::key_encoding`.

[TRM-005]
Mouse encoding MUST translate action, button, modifiers, and cell
position into the reference SGR bytes.
Falsifier: any encoded event differs from the ported vector for the
same input.
Mechanism: `cargo test -p suprtui term::mouse_encoding`.

[TRM-006]
The embedded terminal MUST accept program output bytes, resize,
scrolling, and selection, MUST compose its visible screen onto a cell
grid at the requested position, and MUST report its cursor.
Falsifier: the composed grid differs from the reference fixture for
the same input bytes.
Mechanism: `cargo test -p suprtui term::embedded_compose` over ported
fixtures.

[TRM-007]
Draining embedded-terminal responses MUST deliver every pending byte
exactly once and in order.
Falsifier: a drained byte reappears in the next drain, or bytes
arrive out of order.
Mechanism: `cargo test -p suprtui term::response_drain`.

[TRM-008]
The terminal MUST expose its detected environment (terminal name and
version, multiplexer presence) for capability decisions.
Falsifier: a session inside tmux reports no multiplexer, or the name
is empty after identification.
Mechanism: `cargo test -p suprtui term::environment`.

[TRM-009]
Clipboard writes MUST reject payloads beyond the sequence budget with
an error and MUST never truncate silently.
Falsifier: an oversized payload writes a truncated sequence without
an error.
Mechanism: `cargo test -p suprtui term::clipboard_budget`.

[TRM-010]
Image-protocol selection MUST NOT choose Kitty graphics when the
terminal reports no support, and MUST NOT choose Sixel where the
terminal refuses it.
Falsifier: Kitty is selected with `kitty_graphics` off, or Sixel is
selected for a refusing terminal.
Mechanism: `cargo test -p suprtui term::image_protocol`.

## Review

Attacked the draft for contradictions, weak falsifiers, and
uncheckable requirements. The review checked TRM-010 against REN-010:
this domain chooses the protocol, the renderer renders through it, so
neither owns the other's half. It challenged TRM-006's engine
neutrality and kept it: the fixtures observe composed grids, which
any conforming virtual-terminal engine produces. It confirmed
TRM-004's rejection rule against the reference validation of modifier
and composing fields. No spec lint binary exists in this repository
yet, so the one-obligation rule was checked by hand.
