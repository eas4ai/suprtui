# Unicode (UNI)

Status: Agreed 2026-09-07.
Prefix: UNI.

Reference: `reference/opentui-0.5.11/packages/native/src/utf8.zig`,
`reference/opentui-0.5.11/packages/native/src/grapheme.zig`, and the
width fixtures in
`reference/opentui-0.5.11/packages/native/src/tests/utf8_wcwidth_test.zig`,
`utf8_test.zig`, and `utf8_no_zwj_test.zig`. The crate exposes this
domain as `src/uni.rs`.

[UNI-001]
The `uni` module MUST offer all four width methods (`wcwidth`,
`unicode`, `no_zwj`, `unicode_wide`) with the caller choosing one per
call.
Falsifier: a test measures a ZWJ sequence under `no_zwj` and observes
the joined width instead of the split width.
Mechanism: `cargo test -p suprtui uni::width_method_selection`.

[UNI-002]
The `uni` module MUST report width 2 for fullwidth and wide
characters, width 0 for combining marks and format controls
(including U+200B, U+200C, U+200D, U+FEFF, and variation selectors),
and MUST NOT let control characters advance the cursor.
Falsifier: any ported width vector disagrees with the module's answer.
Mechanism: `cargo test -p suprtui uni::width_vectors` over fixtures
ported from the reference width tests.

[UNI-003]
The `uni` module MUST report width 2 for every code point in the
reference emoji and symbol ranges (for example U+231A and the
U+1F000–U+1F0F5 blocks).
Falsifier: a listed code point measures anything other than 2.
Mechanism: `cargo test -p suprtui uni::emoji_width_table` over the
ported range table.

[UNI-004]
Grapheme segmentation MUST NOT place a break inside a base-plus-mark
sequence, and MUST join ZWJ sequences except under `no_zwj`.
Falsifier: iterating "e" plus U+0301 yields two clusters instead of
one.
Mechanism: `cargo test -p suprtui uni::grapheme_breaks`, including the
ported `no_zwj` vectors.

[UNI-005]
Wrap and position-by-width searches MUST break only at cluster
boundaries and MUST expand tabs to the caller-supplied tab width.
Falsifier: a reported wrap position lands mid-cluster, or a tab
advances fewer columns than the next tab stop.
Mechanism: `cargo test -p suprtui uni::wrap`.

[UNI-006]
Line-break scanning MUST recognize LF, CR, and CRLF, and MUST report
CRLF as a single break.
Falsifier: a CRLF sequence is reported as two breaks.
Mechanism: `cargo test -p suprtui uni::line_breaks`.

[UNI-007]
The `uni` module MUST NOT panic on invalid UTF-8 input, and every
fallible entry point MUST return an error for truncated or overlong
sequences.
Falsifier: any input in the fixed invalid corpus panics, hangs, or is
accepted as valid.
Mechanism: `cargo test -p suprtui uni::invalid_input`.

[UNI-008]
Grapheme-cluster storage MUST be explicit caller-owned state, and the
`uni` module MUST NOT keep process-global mutable state.
Falsifier: two independently created pools observe each other's
entries, or `src/uni.rs` contains a global mutable cell.
Mechanism: `cargo test -p suprtui uni::pool_isolation` plus review of
`src/uni.rs`.

[UNI-009]
Packed cell characters MUST use the reference bit layout, where the
top two bits select plain, image, grapheme-start, or continuation and
the low 26 bits carry the image or grapheme id.
Falsifier: the `is_continuation`, `is_grapheme`, and `is_image`
predicates disagree with reference vectors for the same packed
values.
Mechanism: `cargo test -p suprtui uni::char_packing`.

## Review

Attacked the draft for contradictions, weak falsifiers, and
uncheckable requirements. The review corrected one false assumption:
an earlier draft required `resize` to preserve overlap, but the
reference clears every cell, so preservation belongs nowhere and the
clearing rule lives in BUF-006. It challenged UNI-008 against the
reference, which keeps a process-global grapheme pool; the deviation
is deliberate because the keystone forbids process-global mutable
state, and the falsifier names the observable difference. It checked
that every falsifier names concrete inputs (ZWJ sequences, U+0301,
CRLF, the invalid corpus) rather than restating its requirement. No
spec lint binary exists in this repository yet, so the one-obligation
rule was checked by hand.
