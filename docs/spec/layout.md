# Layout (LAY)

Status: Agreed 2026-09-07.
Prefix: LAY.

Reference: `reference/opentui-0.5.11/packages/native/src/yoga.zig`,
`reference/opentui-0.5.11/packages/native/src/native-renderable.zig`,
and the fixtures in
`reference/opentui-0.5.11/packages/native/src/tests/yoga_test.zig`
and `native-renderable_test.zig`. The crate exposes this domain as
`src/layout.rs`. The flexbox engine arrives from a Rust crate,
decided with a decision record; this domain specifies layout
behavior, not the engine's source.

[LAY-001]
Flex layout MUST support row and column directions, grow and shrink
factors, wrapping, justification and alignment, margins, padding,
gaps, and min/max sizes.
Falsifier: the ported row-with-growing-child fixture measures
anything but 100 by 100 within a thousandth of a cell.
Mechanism: `cargo test -p suprtui layout::flex` over ported fixtures.

[LAY-002]
Layout MUST use classical flex defaults (not web defaults) with a
point scale factor of one.
Falsifier: an auto-sized node sizes itself per web-default rules.
Mechanism: `cargo test -p suprtui layout::config_defaults`.

[LAY-003]
A renderable MUST bind one layout node with at most one measure
target (a text view or an editor view), MUST consult that target
during layout, and MUST stop measuring once the target is cleared.
Falsifier: changing the bound text leaves the laid-out size
unchanged, or measuring continues after clearing.
Mechanism: `cargo test -p suprtui layout::measure_targets`.

[LAY-004]
Every node MUST report its computed position and size after layout,
including nested children.
Falsifier: a nested child's reported position ignores its parent's
offset.
Mechanism: `cargo test -p suprtui layout::computed_positions`.

## Review

Attacked the draft for contradictions, weak falsifiers, and
uncheckable requirements. The review confirmed LAY-001's tolerance
against the reference test's own epsilon (0.001), so float engines
pass without bit-exactness. It challenged engine neutrality and kept
it: the fixtures observe sizes and positions, which any conforming
flexbox engine produces. It recorded that the Yoga C dependency is
not ported; the Rust replacement is a decided-later crate choice, not
a gap in behavior. No spec lint binary exists in this repository yet,
so the one-obligation rule was checked by hand.
