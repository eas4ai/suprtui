# Renderer (REN)

Status: Agreed 2026-09-07.
Prefix: REN.

Reference: `reference/opentui-0.5.11/packages/native/src/renderer.zig`,
`reference/opentui-0.5.11/packages/native/src/renderer-output.zig`,
`reference/opentui-0.5.11/packages/native/src/terminal-image.zig`,
`reference/opentui-0.5.11/packages/native/src/kitty-transport.zig`, and
the fixtures in
`reference/opentui-0.5.11/packages/native/src/tests/renderer_test.zig`.
The crate exposes this domain as `src/render.rs`. Terminal mode
sequences (mouse, Kitty keyboard) belong to `term`; this domain only
calls them during setup and shutdown.

[REN-001]
The caller MUST draw each frame into the next frame buffer, and
`render` MUST compare it against the current frame and publish the
difference.
Falsifier: content drawn into the next buffer never appears in the
output, or appears without a `render` call.
Mechanism: `cargo test -p suprtui render::frame_publish` against the
memory backend.

[REN-002]
When nothing changed and force is false, the renderer MUST report
skipped and MUST NOT rewrite any cell.
Falsifier: rendering an unchanged frame twice rewrites a cell on the
second pass.
Mechanism: `cargo test -p suprtui render::unchanged_skips`.

[REN-003]
The renderer MUST emit cursor moves and cursor style changes exactly
when they change between frames.
Falsifier: an unchanged cursor is re-emitted, or a moved cursor keeps
its old position in the output.
Mechanism: `cargo test -p suprtui render::cursor_tracking`.

[REN-004]
A failed frame MUST publish nothing: the hit grid, image state, and
cursor cache roll back, and the next render MUST repaint fully.
Falsifier: a hit-test answer or cell reflects the failed frame, or
the frame after a failure leaves a stale cell.
Mechanism: `cargo test -p suprtui render::failed_frame_rolls_back`
with a failing backend.

[REN-005]
The memory backend MUST capture exactly the bytes the renderer wrote,
so tests observe the same stream a terminal would receive.
Falsifier: captured bytes differ from a tee'd copy of the written
stream for the same frame.
Mechanism: `cargo test -p suprtui render::memory_backend_exact`.

[REN-006]
Threaded and single-threaded rendering MUST emit identical bytes for
identical frames.
Falsifier: any byte differs between the two modes for the same frame
sequence.
Mechanism: `cargo test -p suprtui render::thread_parity`.

[REN-007]
Setup MUST enter the alternate screen when requested, and shutdown
and suspend MUST restore the prior terminal state; clearing on
shutdown MUST follow the clear-on-shutdown flag.
Falsifier: shutdown leaves the alternate screen active, or clears the
screen when the flag is off.
Mechanism: `cargo test -p suprtui render::lifecycle_sequences`
against the memory backend.

[REN-008]
`check_hit` MUST return the topmost id at the queried coordinates,
clipped additions MUST respect the scissor, and the grid MUST commit
only on a successful render.
Falsifier: a hit answer comes from a failed frame, or an id is
returned outside the active scissor.
Mechanism: `cargo test -p suprtui render::hit_grid`.

[REN-009]
With a nonzero render offset, drawing and cursor addresses MUST shift
by the offset, and resetting the offset to zero MUST clear the footer
surface.
Falsifier: the cursor lands on the unshifted row while an offset is
active.
Mechanism: `cargo test -p suprtui render::split_offset`.

[REN-010]
When the terminal reports no Kitty graphics support, image cells MUST
render through fallback materialization, and the missing capability
MUST NOT fail the frame.
Falsifier: a frame fails, or an image cell emits Kitty sequences to a
backend that reported no support.
Mechanism: `cargo test -p suprtui render::image_fallback`.

[REN-011]
Render statistics MUST count one frame per rendered frame and MUST
NOT count skipped frames.
Falsifier: a skipped frame increments the frame counter or the cells
counter.
Mechanism: `cargo test -p suprtui render::stats_count`.

[REN-012]
Committed frame bytes MUST reach real stdout through a `StdoutBackend`
implementing the `Backend` trait: frame bytes in commit order, direct
bytes immediately, failed frames dropped, and writer failures reported
as `Failed`, never panics. The same frame sequence through the memory
and stdout backends MUST produce identical streams.
Falsifier: committed bytes never arrive, arrive reordered, or a broken
pipe panics.
Mechanism: `cargo test -p suprtui render::stdout_backend` over a
`Cursor<Vec<u8>>` stand-in plus a failing writer.

## Review

Attacked the draft for contradictions, weak falsifiers, and
uncheckable requirements. The review verified REN-002 and REN-004
against the reference `render`, `finishSkippedFrame`, and
`finishFailedFrame`: skipped frames clear the next buffer without
publishing, and failures set full-repaint plus cursor-cache reset, so
both falsifiers name observable memory-backend output. It challenged
REN-006's byte-parity as too strong and kept it: the reference runs
one diff path behind both modes, so parity is the point of the mode.
It moved terminal mode toggles (mouse, Kitty keyboard) to the future
TRM domain to avoid double ownership, and recorded the mapping here.
No spec lint binary exists in this repository yet, so the
one-obligation rule was checked by hand.
