# Cell buffer (BUF)

Status: Agreed 2026-09-07.
Prefix: BUF.

Reference: `reference/opentui-0.5.11/packages/native/src/buffer.zig`,
`reference/opentui-0.5.11/packages/native/src/ansi.zig`,
`reference/opentui-0.5.11/packages/native/src/buffer-methods.zig`, and
the fixtures in
`reference/opentui-0.5.11/packages/native/src/tests/buffer_test.zig`
and `buffer-methods_test.zig`. The crate exposes this domain as
`src/buffer.rs`. Width questions belong to `uni`; this domain only
stores the widths `uni` reports.

[BUF-001]
Colors MUST carry 8-bit red, green, blue, and alpha channels plus
intent metadata (`rgb`, `indexed` with a palette slot, or terminal
`default`), and every buffer operation MUST preserve that metadata.
Falsifier: a pack, blend, and unpack round trip changes the intent or
the slot.
Mechanism: `cargo test -p suprtui buffer::color_model`.

[BUF-002]
Indexed colors MUST resolve through the reference 16-entry palette
and the six-level 256-color cube.
Falsifier: palette index 9 resolves to anything other than pure red
(255, 0, 0).
Mechanism: `cargo test -p suprtui buffer::palette` over ported palette
fixtures.

[BUF-003]
A cell MUST hold exactly one packed char, one fg color, one bg color,
and one attribute word whose low 8 bits are style flags (bold, dim,
italic, underline, blink, inverse, hidden, strikethrough) and whose
upper 24 bits are the link id, and writing the link id MUST leave the
style flags untouched.
Falsifier: a set-then-get round trip changes any field, or a link id
write alters a style flag.
Mechanism: `cargo test -p suprtui buffer::cell_roundtrip`.

[BUF-004]
`get` MUST return `None` for coordinates outside the grid,
`set` and `set_raw` MUST silently ignore outside writes, and no
buffer entry point MUST panic on coordinates.
Falsifier: an out-of-bounds access panics, or `get` returns `Some`
outside the grid.
Mechanism: `cargo test -p suprtui buffer::bounds`.

[BUF-005]
`resize` MUST reject zero width or zero height with an error.
Falsifier: a zero-size resize succeeds.
Mechanism: `cargo test -p suprtui buffer::resize_errors`.

[BUF-006]
`resize` MUST clear every cell, so no pre-resize value survives a
resize.
Falsifier: a cell value written before a resize is still readable
after it.
Mechanism: `cargo test -p suprtui buffer::resize_clears`.

[BUF-007]
`clear` MUST reset fg to opaque white, attributes to none, chars to
space (or the caller-supplied char), and MUST drop link, grapheme,
and image-placement state.
Falsifier: a link id or image placement written before a clear is
still observable after it.
Mechanism: `cargo test -p suprtui buffer::clear`.

[BUF-008]
Alpha compositing MUST match the reference integer formula
bit-exactly, including the rounding, the opaque fast path, and the
backdrop substitution for transparent destinations.
Falsifier: any channel of a blended fixture differs by 1 or more from
the reference vector.
Mechanism: `cargo test -p suprtui buffer::blending` over ported
blending fixtures.

[BUF-009]
The scissor stack MUST clip all drawing to the active rectangle,
pushes and pops MUST nest, and clearing MUST remove every rectangle.
Falsifier: a draw outside the active scissor changes any cell.
Mechanism: `cargo test -p suprtui buffer::scissor`.

[BUF-010]
A wide grapheme MUST occupy a start cell plus continuation cells, and
overwriting or clipping at the grid edge MUST never leave an orphan
continuation cell.
Falsifier: the reference edge-overwrite fixture leaves a continuation
cell whose start cell is gone.
Mechanism: `cargo test -p suprtui buffer::wide_cells`, including the
synced-write case that skips span cleanup.

[BUF-011]
ANSI emission MUST use truecolor sequences for `rgb` intent, palette
sequences for `indexed` intent, default sequences for `default`
intent, and MUST emit style flags in the reference order.
Falsifier: any emitted byte sequence differs from the reference
escape-sequence fixture for the same cell.
Mechanism: `cargo test -p suprtui buffer::ansi_output`.

[BUF-012]
Compositing one grid onto another MUST clip the source region and
MUST apply alpha and scissor exactly as direct drawing does.
Falsifier: a composited region differs from the same cells drawn
directly one by one.
Mechanism: `cargo test -p suprtui buffer::composite`.

[BUF-013]
Compositing a source grid from a different grapheme pool MUST resolve
cluster bytes through the source grid's pool.
Falsifier: a cross-pool composite renders a multi-codepoint cluster
with wrong bytes or a foreign id.
Mechanism: `cargo test -p suprtui buffer::cross_pool_composite`.

## Review

Attacked the draft for contradictions, weak falsifiers, and
uncheckable requirements. The review verified BUF-004 against the
reference `validateAndIndex` behavior (ignore on write, null on read)
and verified BUF-006 against the reference `resize`, which clears
unconditionally. It challenged BUF-008's bit-exactness: the formula
uses named integer rounding with no floating point, so bit-exactness
is checkable and the falsifier tolerates nothing. It confirmed BUF-010
covers both write paths (full cleanup and synced) so the two cannot
diverge silently. It checked BUF-003's link-id masking against the
reference bit shifts. No spec lint binary exists in this repository
yet, so the one-obligation rule was checked by hand.
