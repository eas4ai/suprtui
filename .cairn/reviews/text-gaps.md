# Review: text-gaps

commit: 3374826259d3b1806f948f83b193e7497688c5f0
findings:
  - closed: gesture machine resolves word/line/cell units over display cells with reference separator classes (space runs selectable, slash not a boundary, CJK+ASCII grouped); padding clicks are zero-width, backward drags equal forward drags, word/line convert to cell without losing text — 9 vectors
  - closed: viewport selection matches the reference horizontal case exactly (focus-exclusive [10,15)="KLMNO" pattern verified against the Zig test); wrapping ignores the horizontal offset; scrolled selections equal unscrolled equivalents — 5 vectors
  - closed: cluster iteration never splits (combining marks join, emoji ZWJ sequences hold per UAX GB11, zero-width prefixes terminate with full coverage); lines/coords round-trip over every boundary — 3 vectors
  - closed: recompute counter proves one layout per content/width/wrap state and no stale row survives an edit — 2 vectors
  - closed: EditorView owns viewport scroll, cursor visibility with margins, local selection set/update/reset/convert with follow mode, and logical/visual round trips — 6 vectors
  - closed: SPEC-022 faults discriminate exactly (padding expansion, wrapping-offset honor, stale cache); all reverted byte-clean
  - closed: 25/25 integration plus 5 lib unit tests green, full suite 22/22 targets ok, clippy zero, fmt clean, no `unsafe`, no `static`, no new dependencies
