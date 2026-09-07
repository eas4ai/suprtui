# Text (TXT)

Status: Agreed 2026-09-07.
Prefix: TXT.

Reference: `reference/opentui-0.5.11/packages/native/src/rope.zig`,
`reference/opentui-0.5.11/packages/native/src/text-buffer.zig`,
`reference/opentui-0.5.11/packages/native/src/text-buffer-view.zig`,
`reference/opentui-0.5.11/packages/native/src/text-buffer-segment.zig`,
`reference/opentui-0.5.11/packages/native/src/text-buffer-iterators.zig`,
`reference/opentui-0.5.11/packages/native/src/edit-buffer.zig`,
`reference/opentui-0.5.11/packages/native/src/editor-view.zig`,
`reference/opentui-0.5.11/packages/native/src/syntax-style.zig`, and the
fixtures in
`reference/opentui-0.5.11/packages/native/src/tests/text-buffer_test.zig`,
`text-buffer-view_test.zig`, `edit-buffer_test.zig`, and
`rope_test.zig`. The crate exposes this domain as `src/text.rs`.
Painting these structures onto a cell grid is drawing; `buffer` owns
the grid, this domain owns the content.

[TXT-001]
Text storage MUST support insert, delete, and split at arbitrary
offsets, with the full content always readable back as plain text.
Falsifier: plain-text extraction differs from the reference fixture
after a ported edit sequence.
Mechanism: `cargo test -p suprtui text::edits` over ported vectors.

[TXT-002]
Style spans MUST stay attached to their text across edits that occur
before or inside them.
Falsifier: inserting text before a span leaves the span on its old
columns instead of shifting with its text.
Mechanism: `cargo test -p suprtui text::span_tracking`.

[TXT-003]
Undo MUST restore content and styles step by step in reverse order,
redo MUST replay them, and any new edit MUST clear the redo stack.
Falsifier: redo still applies after an intervening edit, or undo
restores content but drops styles.
Mechanism: `cargo test -p suprtui text::undo_redo`, including the
ported edit-buffer history vectors.

[TXT-004]
The buffer MUST mark every registered view dirty on each content edit
and MUST advance the content epoch, clearing the flag per view on
request.
Falsifier: a registered view reads clean after an edit, or the epoch
is unchanged.
Mechanism: `cargo test -p suprtui text::view_dirty_tracking`.

[TXT-005]
Virtual lines MUST reflect the view's width and wrap mode, and wraps
MUST occur only at allowed breaks, never mid-cluster.
Falsifier: a virtual line exceeds the set width, or a wrap splits a
grapheme cluster.
Mechanism: `cargo test -p suprtui text::wrapping`, including the
ported word-wrap vectors.

[TXT-006]
Word selection MUST cover the whole word, line selection the whole
line, and selected-text extraction MUST return exactly the selected
bytes.
Falsifier: a word selection stops mid-word, or extracted text differs
from the selected range.
Mechanism: `cargo test -p suprtui text::selection`.

[TXT-007]
Cursor edits (insert, backspace, delete, arrow moves) MUST operate on
whole grapheme clusters and MUST never leave half a cluster behind.
Falsifier: a backspace on a combining sequence removes only the mark
or only the base.
Mechanism: `cargo test -p suprtui text::cursor_cluster_safety`.

[TXT-008]
Removing highlights by reference MUST remove exactly that
reference's spans and MUST leave other spans untouched.
Falsifier: a foreign span disappears, or a removed span still
renders.
Mechanism: `cargo test -p suprtui text::highlight_refs`.

[TXT-009]
Attaching a syntax style MUST change the style spans of matching text
according to that style.
Falsifier: no span changes after attaching a style to text it covers.
Mechanism: `cargo test -p suprtui text::syntax_style`.

[TXT-010]
Out-of-range offsets MUST return errors, and no text entry point MUST
panic on them.
Falsifier: any offset beyond the content end panics or corrupts
content.
Mechanism: `cargo test -p suprtui text::offset_bounds`.

## Review

Attacked the draft for contradictions, weak falsifiers, and
uncheckable requirements. The review checked TXT-005 against UNI-005
so the two agree that only `uni` decides break positions, with this
domain caching the resulting virtual lines. It challenged TXT-003's
redo-clearing rule and confirmed it against the reference undo
behavior in the ported history tests. It left rope balancing as an
internal detail: TXT-001 observes only edit and read behavior, so any
balanced structure passes. No spec lint binary exists in this
repository yet, so the one-obligation rule was checked by hand.
